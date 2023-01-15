use colored::Colorize;
use dialoguer::Confirm;
use index_client::IndexClient;
use log::{debug, info};
use miette::{IntoDiagnostic, Report, Result};

use crate::{config::get_config, defaults::theme};

pub struct Install;

impl Install {
    pub async fn execute(package: &str, _dry_run: &bool) -> Result<()> {
        let config = get_config()?;

        let mut index_client =
            IndexClient::from_index_and_user_version(config.index, env!("CARGO_PKG_VERSION")).await?;
        let index_client = index_client.with_pat(config.pat);

        info!("Fetching package metadata for package {}.", package.bold());
        let package_metadata = index_client.get_package(package).await?;
        debug!("Fetched package metadata: {:#?}", package_metadata);

        let release = package_metadata.get_latest_release().await?;

        debug!("{}", "===============BEGIN ASSET LIST===============".blue().bold());
        debug!("This information is logged to help debug errors made by the file selection algorithm.");
        for asset in release.assets {
            debug!(
                "{asset_id_section_marker} {} | {name_section_marker} {} | {size_section_marker} {} | {url_section_marker} {}",
                asset.id, asset.name, asset.size, asset.browser_download_url,
                asset_id_section_marker = "Asset ID:".bold(),
                name_section_marker = "Name:".bold(),
                size_section_marker = "Size:".bold(),
                url_section_marker = "Download URL:".bold(),
            )
        }
        debug!("{}", "================END ASSET LIST================".blue().bold());

        let should_install = Confirm::with_theme(&theme())
            .with_prompt(format!(
                "Install `{package}` {}?",
                release.name.unwrap_or_else(|| "(version unspecified)".to_string())
            ))
            .default(false)
            .interact()
            .into_diagnostic()?;

        if !should_install {
            return Err(Report::msg("User aborted operation"));
        }

        Ok(())
    }
}
