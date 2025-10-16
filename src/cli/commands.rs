use anyhow::Result;

use super::{Args, Commands};
use crate::ops;

/// Execute the appropriate command based on CLI arguments
pub fn run_command(args: Args) -> Result<()> {
    match args.command {
        Some(Commands::Doctor) => {
            ops::doctor::run()?;
        }
        None => {
            // Default: run interactive configuration
            ops::interactive::run(args)?;
        }
    }

    Ok(())
}
