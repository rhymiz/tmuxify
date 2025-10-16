use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "tmuxify")]
#[command(about = "Interactive tmuxp configuration generator", long_about = None)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Enable dry-run mode (print output without writing files)
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Force overwrite without creating backups
    #[arg(long, global = true)]
    pub force: bool,

    /// Project root directory (defaults to current directory)
    #[arg(long, global = true)]
    pub project: Option<PathBuf>,

    /// Where to store the tmuxp file (home or project)
    #[arg(long, global = true, value_name = "LOCATION")]
    pub tmuxp_location: Option<String>,

    /// Session name (defaults to directory name)
    #[arg(long, global = true)]
    pub session: Option<String>,

    /// Override start_directory in tmuxp config
    #[arg(long, global = true)]
    pub start_dir: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run diagnostics to check dependencies and shell hooks
    Doctor,
}
