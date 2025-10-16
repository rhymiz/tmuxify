mod cli;
mod model;
mod ops;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    cli::run_command(args)?;
    Ok(())
}
