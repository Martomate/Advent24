use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};

use anyhow::{bail, Context};
use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    day: u8,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.day {
        1 => {
            build(
                Command::new("roc")
                    .arg("build")
                    .arg("main.roc")
                    .current_dir("days/d01"),
            )?;

            TemporaryArtifact::defer_deletion(PathBuf::from("./days/d01/main"), || {
                let success = run_program(
                    Command::new("./main").current_dir("days/d01"),
                    &PathBuf::from("samples/d01/in.txt"),
                    &PathBuf::from("samples/d01/out.txt"),
                )?;
                if !success {
                    bail!("sample failed");
                }
    
                let success = run_program(
                    Command::new("./main").current_dir("days/d01"),
                    &PathBuf::from("inputs/d01/in.txt"),
                    &PathBuf::from("inputs/d01/out.txt"),
                )?;
                if !success {
                    bail!("main test case failed");
                }
                Ok(())
            })?;
        }

        d => bail!("day {d} is not supported yet"),
    };

    Ok(())
}

struct TemporaryArtifact {
    path: PathBuf,
}

impl TemporaryArtifact {
    fn defer_deletion<T>(path: PathBuf, use_artifact: impl FnOnce() -> anyhow::Result<T>) -> anyhow::Result<T> {
        let artifact = TemporaryArtifact { path };
        let res = use_artifact();
        drop(artifact);
        res
    }
}

impl Drop for TemporaryArtifact {
    fn drop(&mut self) {
        println!("Removing file at {:?}", self.path.as_path());
        fs::remove_file(&self.path).unwrap();
    }
}

fn build(cmd: &mut Command) -> anyhow::Result<()> {
    let out = cmd
        .stdout(Stdio::piped())
        .output()
        .context("could not run build command")?;

    if !out.status.success() {
        println!("{}", String::from_utf8_lossy(&out.stdout));
        eprintln!("{}", String::from_utf8_lossy(&out.stderr));
        bail!("build failed");
    }

    Ok(())
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

    let out = proc
        .wait_with_output()
        .context("could not read stdout")?;

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
        eprintln!("The output was not correct.\n");
        eprintln!("Expected:\n{}\n", expected);
        eprintln!("Actual:\n{}", actual);
        eprintln!();

        Ok(false)
    } else {
        Ok(true)
    }
}
