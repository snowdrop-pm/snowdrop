use crate::commands::*;
use clap::{Parser, Subcommand};
use miette::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Install a package.
    Install {
        /// The name of the package you want to install
        package: String,

        /// Whether to run a "dry-run". If this flag is set, then no
        /// files will be written
        dry_run: bool,
    },
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        match self {
            Command::Install { dry_run, package } => install::Install::execute(package, dry_run),
        }
    }
}
