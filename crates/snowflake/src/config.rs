use crate::dirs::get_project_dirs;
use config::Config;
use miette::{IntoDiagnostic, Result, WrapErr};

pub fn get_config() -> Result<Config> {
    let settings = Config::builder()
        .add_source(
            config::File::with_name(
                get_project_dirs()?
                    .config_dir()
                    .join("config.toml")
                    .to_str()
                    .unwrap(),
            )
            .required(false),
        )
        .add_source(config::Environment::with_prefix("SNOWFLAKE_"))
        .build()
        .into_diagnostic()
        .wrap_err("failed to read config")?;

    Ok(settings)
}
