#![deny(missing_docs)]
//! A package manager for GitHub Releases
use clap::Parser;
use miette::Result;

mod cli_struct;
mod commands;

use cli_struct::Cli;

fn main() -> Result<()> {
    let command = Cli::parse().command;
    command.execute()?;
    println!("Hello, world!");

    Ok(())
}
