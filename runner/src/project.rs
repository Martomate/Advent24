use crate::program::Program;

use std::{
    collections::HashSet,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};

use anyhow::{bail, Context};
use colored::Colorize;

pub struct Project {
    root: PathBuf,
}

impl Project {
    pub fn at_root(root: PathBuf) -> Self {
        Project { root }
    }

    pub fn run_build_steps(&self, steps: impl IntoIterator<Item = Program>) -> anyhow::Result<()> {
        for step in steps {
            let mut cmd = Command::from(&step);
            let cmd = cmd.current_dir(&self.root);

            print_cmd(cmd);
            let out = cmd
                .stdout(Stdio::piped())
                .output()
                .context("could not run build command")?;

            if !out.status.success() {
                println!("{}", "Command failed to run. Output:\n".red());
                println!("{}", String::from_utf8_lossy(&out.stdout));
                eprintln!("{}", String::from_utf8_lossy(&out.stderr));
                bail!("build failed");
            }
        }
        Ok(())
    }

    pub fn run_tests(&self, program: Program, test_dir: &Path) -> anyhow::Result<()> {
        let mut in_files: Vec<String> = Vec::new();
        let mut out_files: Vec<String> = Vec::new();

        for e in (test_dir.read_dir()?).flatten() {
            if let Ok(m) = e.metadata() {
                if m.is_file() {
                    let file_name = e.file_name().into_string().unwrap();
                    if let Some(name) = file_name.strip_suffix(".in") {
                        in_files.push(name.to_string());
                    } else if let Some(name) = file_name.strip_suffix(".out") {
                        out_files.push(name.to_string());
                    } else {
                        eprintln!("{}", format!("Unexpected testfile: {}", file_name).yellow());
                    }
                }
            }
        }

        let in_files: HashSet<_> = HashSet::from_iter(in_files);
        let out_files: HashSet<_> = HashSet::from_iter(out_files);

        for name in in_files.difference(&out_files) {
            eprintln!(
                "{}",
                format!("Found '{}.in' but not '{}'.out", name, name).yellow()
            );
        }
        for name in out_files.difference(&in_files) {
            eprintln!(
                "{}",
                format!("Found '{}.out' but not '{}'.in", name, name).yellow()
            );
        }

        let mut test_cases: Vec<String> = in_files.intersection(&out_files).cloned().collect();
        test_cases.sort();

        for name in test_cases {
            let in_file_name = test_dir.join(format!("{name}.in"));
            let out_file_name = test_dir.join(format!("{name}.out"));

            let success = run_program(
                Command::from(&program).current_dir(&self.root),
                &in_file_name,
                &out_file_name,
            )
            .context("failed to run program")?;
            if !success {
                bail!("test case failed: {}", name);
            }
        }
        Ok(())
    }
}

pub struct TemporaryArtifact {
    path: PathBuf,
}

impl TemporaryArtifact {
    pub fn defer_deletion<T>(
        path: PathBuf,
        use_artifact: impl FnOnce() -> anyhow::Result<T>,
    ) -> anyhow::Result<T> {
        let artifact = TemporaryArtifact { path };
        let res = use_artifact();
        drop(artifact);
        res
    }
}

impl Drop for TemporaryArtifact {
    fn drop(&mut self) {
        println!(
            "{}",
            format!("Removing file at {:?}", self.path.as_path()).bright_black()
        );
        fs::remove_file(&self.path).unwrap();
    }
}

fn run_program(cmd: &mut Command, sample_in: &Path, sample_out: &Path) -> anyhow::Result<bool> {
    let mut input_file = File::open(sample_in).context("failed to open input file")?;
    let mut input_bytes = Vec::new();
    input_file
        .read_to_end(&mut input_bytes)
        .context("failed to read input file")?;

    let mut out_file = File::open(sample_out).context("failed to open expectation file")?;
    let mut out_bytes = Vec::new();
    out_file
        .read_to_end(&mut out_bytes)
        .context("failed to read expectation file")?;

    print_cmd(cmd);
    let mut proc = cmd
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .context("could not run program")?;

    let mut stdin = proc.stdin.take().context("failed to open stdin")?;
    thread::spawn(move || {
        stdin
            .write_all(&input_bytes)
            .expect("failed to write to stdin");
    });

    let out = proc.wait_with_output().context("could not read stdout")?;

    if !out.status.success() {
        bail!(String::from_utf8_lossy(&out.stderr).to_string());
    }
    let program_output = String::from_utf8_lossy(&out.stdout).to_string();

    // Compare with expectation
    let out_string = String::from_utf8_lossy(&out_bytes).to_string();

    let expected = out_string.trim();
    let actual = program_output.trim();
    if expected != actual {
        eprintln!();
        eprintln!("{}", "The output was not correct.".red());
        eprintln!();
        eprintln!("Actual:\n{}", actual);
        eprintln!();
        eprintln!("Expected:\n{}", expected);
        eprintln!();

        Ok(false)
    } else {
        Ok(true)
    }
}

fn print_cmd(cmd: &Command) {
    println!("{}", format!("Running: {:?}", cmd).bright_black());
}
