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
    styled_env_logger::formatted_builder()
        .filter(None, LevelFilter::Info)
        .parse_filters(&env::var("RUST_LOG").unwrap_or(String::from("INFO")))
        .parse_write_style(&env::var("RUST_LOG_STYLE").unwrap_or(String::from("auto")))
        .try_init()
        .into_diagnostic()?;

    let command = Cli::parse().command;
    command.execute().await?;

    Ok(())
}
