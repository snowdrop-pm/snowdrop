use miette::{Diagnostic, IntoDiagnostic, Result, WrapErr};
use secrecy::SecretString;
use serde::Deserialize;
use thiserror::Error;

use crate::{defaults::default_package_index, dirs::get_project_dirs};

pub fn get_config() -> Result<Config> {
    let config = config::Config::builder()
        .add_source(
            config::File::with_name(get_project_dirs()?.config_dir().join("config.toml").to_str().unwrap())
                .required(false),
        )
        .add_source(
            config::File::with_name(get_project_dirs()?.config_dir().join("pat.toml").to_str().unwrap())
                .required(false),
        )
        .add_source(config::Environment::with_prefix("SNOWDROP"))
        .build()
        .into_diagnostic()
        .wrap_err("failed to read config")?
        .try_deserialize::<Config>()
        .into_diagnostic()
        .wrap_err("failed to deserialize config")?;

    Ok(config)
}

#[derive(Deserialize)]
pub struct Config {
    /// The root URL of the package index.
    #[serde(default = "default_package_index")]
    pub index: String,

    /// The GitHub PAT.
    pub pat: Option<SecretString>,
}

#[derive(Error, Diagnostic, Debug)]
pub enum ConfigError {
    #[error("No GitHub PAT specified")]
    #[diagnostic(help("Run `snowflake auth` to set this up"))]
    NoPat,
}

impl Config {
    // TODO: Find better way to get PAT
    pub const fn get_pat(&self) -> Result<&SecretString, ConfigError> {
        let Some(ref pat) = self.pat else {
            return Err(ConfigError::NoPat)
        };
        Ok(pat)
    }
}
