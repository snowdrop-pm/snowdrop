use colored::Colorize;
use dialoguer::Confirm;
use index_client::IndexClient;
use log::{debug, info};
use miette::{miette, IntoDiagnostic, Result};

mod picker;
use picker::AssetPicker;

use crate::{config::get_config, defaults::theme};

pub struct Install;

impl Install {
    pub async fn execute(package: &str, _dry_run: &bool) -> Result<()> {
        let config = get_config()?;
        let picker = AssetPicker::new();

        let pat = config.get_pat()?;
        let index_client = IndexClient::new(&config.index, env!("CARGO_PKG_VERSION"), pat.clone()).await?;

        info!("Fetching package metadata for package {}.", package.bold());
        let package_metadata = index_client.get_package(package).await?;
        debug!("Fetched package metadata: {:#?}", package_metadata);

        let release = package_metadata.get_latest_release().await?;

        let should_install = Confirm::with_theme(&theme())
            .with_prompt(format!(
                "Install {}?",
                release.name.unwrap_or_else(|| "(version unspecified)".to_string())
            ))
            .default(false)
            .interact()
            .into_diagnostic()?;

        if !should_install {
            return Err(miette!("User aborted operation"));
        }

        dbg!(
            picker
                .choose_asset(release.assets, package_metadata.naming_scheme)?
                .name
        );

        Ok(())
    }
}
