use crate::{program::Program, project::Project};
use anyhow::bail;

pub fn run(day: u8) -> anyhow::Result<()> {
    let p = Project::for_day(day);

    match day {
        1 => {
            p.run_build_steps([Program::new("roc").with_args(["build", "main.roc"])])?;

            p.defer_deletion("main", || {
                p.run_tests(Program::new("./main"), ["samples/d01", "inputs/d01"])
            })?;
        }

        2 => {
            p.run_build_steps([
                Program::new("gleam").with_args(["test"]),
                Program::new("gleam").with_args(["build"]),
                Program::new("gleam").with_args(["run", "-m", "gleescript"]),
            ])?;

            p.defer_deletion("d2", || {
                p.run_tests(Program::new("./d2"), ["samples/d02", "inputs/d02"])
            })?;
        }

        3 => {
            p.run_build_steps([Program::new("elixirc").with_args(["main.ex"])])?;

            p.defer_deletion("Elixir.D3.beam", || {
                p.run_tests(
                    Program::new("elixir").with_args(["-e", "D3.run"]),
                    ["samples/d03", "inputs/d03"],
                )
            })?;
        }

        4 => {
            p.run_build_steps([
                Program::new("dune").with_args(["test"]),
                Program::new("dune").with_args(["build"]),
            ])?;

            let bin_path = "./_build/default/bin/main.exe"; // they call it "exe" on all platforms

            p.run_tests(
                Program::new(bin_path),
                ["samples/d04", "inputs/d04"],
            )?;
        }

        d => bail!("day {d} is not supported yet"),
    };

    Ok(())
}
