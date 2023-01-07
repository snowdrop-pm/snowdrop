#![deny(missing_docs)]
//! A package manager for GitHub Releases
use clap::Parser;
use miette::Result;

mod cli_struct;
mod commands;
mod config;
mod defaults;
mod dirs;

use cli_struct::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let command = Cli::parse().command;
    command.execute().await?;
    println!("Hello, world!");

    Ok(())
}
