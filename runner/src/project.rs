use crate::program::Program;

use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};

use anyhow::{bail, Context};
use colored::Colorize;

pub struct Project {
    day: u8,
}

impl Project {
    pub fn for_day(day: u8) -> Self {
        Project { day }
    }

    pub fn run_build_steps(&self, steps: impl IntoIterator<Item = Program>) -> anyhow::Result<()> {
        for step in steps {
            let mut cmd = Command::from(&step);
            let cmd = cmd.current_dir(format!("days/d{:02}", self.day));

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

    pub fn run_tests<D>(&self, program: Program, test_dirs: D) -> anyhow::Result<()>
    where
        D: IntoIterator,
        D::Item: AsRef<str>,
    {
        println!();
        for test_dir in test_dirs {
            let test_dir = test_dir.as_ref();
            let success = run_program(
                Command::from(&program).current_dir(format!("days/d{:02}", self.day)),
                &PathBuf::from(format!("{}/in.txt", test_dir)),
                &PathBuf::from(format!("{}/out.txt", test_dir)),
            )
            .context("failed to run test case")?;
            if !success {
                bail!("test case failed: {}", test_dir);
            }
        }
        println!();
        Ok(())
    }

    pub fn defer_deletion<T>(
        &self,
        file_name: &str,
        use_file: impl FnOnce() -> anyhow::Result<T>,
    ) -> anyhow::Result<T> {
        TemporaryArtifact::defer_deletion(
            PathBuf::from(format!("./days/d{:02}/{}", self.day, file_name)),
            use_file,
        )
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
