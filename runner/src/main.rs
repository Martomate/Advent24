use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};

use anyhow::{bail, Context};
use clap::Parser;
use colored::Colorize;

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
                println!();
                run_test_case(1, "samples/d01", &mut Command::new("./main"))?;
                run_test_case(1, "inputs/d01", &mut Command::new("./main"))?;
                println!();
                Ok(())
            })?;
        }

        2 => {
            build(Command::new("gleam").arg("test").current_dir("days/d02"))?;
            build(Command::new("gleam").arg("build").current_dir("days/d02"))?;
            build(
                Command::new("gleam")
                    .arg("run")
                    .arg("-m")
                    .arg("gleescript")
                    .current_dir("days/d02"),
            )?;

            TemporaryArtifact::defer_deletion(PathBuf::from("./days/d02/d2"), || {
                println!();
                run_test_case(2, "samples/d02", &mut Command::new("./d2"))?;
                run_test_case(2, "inputs/d02", &mut Command::new("./d2"))?;
                println!();
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
    fn defer_deletion<T>(
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

fn build(cmd: &mut Command) -> anyhow::Result<()> {
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

    Ok(())
}

fn run_test_case(day: u8, test_dir: &str, cmd: &mut Command) -> anyhow::Result<()> {
    let success = run_program(
        cmd.current_dir(format!("days/d{:02}", day)),
        &PathBuf::from(format!("{}/in.txt", test_dir)),
        &PathBuf::from(format!("{}/out.txt", test_dir)),
    )
    .context("failed to run test case")?;
    if !success {
        bail!("test case failed: {}", test_dir);
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
