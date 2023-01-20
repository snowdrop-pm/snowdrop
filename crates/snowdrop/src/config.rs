use index_client::error::IndexClientError;
use miette::{IntoDiagnostic, Result, WrapErr};
use secrecy::SecretString;
use serde::Deserialize;

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

type PatError = IndexClientError;
impl Config {
    // TODO: Find better way to get PAT
    pub const fn get_pat(&self) -> Result<&SecretString, PatError> {
        let Some(ref pat) = self.pat else {
            // FIXME: don't return index client errors for config issues
            return Err(PatError::NoPat)
        };
        Ok(pat)
    }
}
