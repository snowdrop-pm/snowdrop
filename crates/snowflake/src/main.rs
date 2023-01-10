#![deny(missing_docs)]
//! A package manager for GitHub Releases
use clap::Parser;
use log::LevelFilter;
use miette::{IntoDiagnostic, Result};
use std::env;

mod cli_struct;
mod commands;
mod config;
mod defaults;
mod dirs;

use cli_struct::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::formatted_builder()
        .filter(None, LevelFilter::Info)
        .parse_filters(&env::var("RUST_LOG").unwrap_or(String::from("INFO")))
        .try_init()
        .into_diagnostic()?;

    let command = Cli::parse().command;
    command.execute().await?;

    Ok(())
}
