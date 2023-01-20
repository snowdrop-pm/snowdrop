use clap::{Parser, Subcommand};
use log::debug;
use miette::Result;

use crate::commands::*;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Install a package.
    Install {
        /// The name of the package you want to install
        package: String,

        /// Whether to run a "dry-run". If this flag is set, then no
        /// files will be written
        #[clap(long)]
        dry_run: bool,
    },

    /// Set a GitHub PAT for authentication.
    Auth,

    /// Search the index for packages.
    Search {
        /// The query you want to make.
        query: String,

        /// The minimum score to use when doing a fuzzy search
        #[clap(long, default_value_t = 0.7)]
        min_score: f32,
    },
}

impl Command {
    pub async fn execute(&self) -> Result<()> {
        debug!("Command invoked: {:#?}", self);
        match self {
            Self::Install { dry_run, package } => install::Install::execute(package, dry_run).await,
            Self::Auth => auth::Auth::execute().await,
            Self::Search {
                query,
                min_score: minimum_score,
            } => search::Search::execute(query.to_string(), minimum_score).await,
        }
    }
}
