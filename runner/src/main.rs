mod config;
mod days;
mod program;
mod project;

use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    day: u8,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    days::run(args.day)?;

    Ok(())
}
