use colored::Colorize;
use index_client::IndexClient;
use log::{debug, info};
use miette::Result;

use crate::config::get_config;

pub struct Install;

impl Install {
    pub async fn execute(package: &str, _dry_run: &bool) -> Result<()> {
        let config = get_config()?;

        let index_client = IndexClient::from_index_and_user_version(config.index, env!("CARGO_PKG_VERSION")).await?;

        info!("Fetching package metadata for package {}.", package.bold());
        let package_metadata = index_client.get_package(package).await?;
        debug!("Fetched package metadata: {:#?}", package_metadata);

        let [owner, repo] = &package_metadata.repo;
        info!(
            "Installing {package_name} {repo_info}",
            package_name = package.bold(),
            repo_info = format!("(repo: {owner}/{repo})").black().bold()
        );

        let release = package_metadata.get_latest_release().await?;
        let release_id = release.id;
        info!(
            "Latest release is {} {}",
            release.name.unwrap_or_else(|| "unspecified".to_string()).bold(),
            format!("(release ID: {release_id})").black().bold()
        );

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

        Ok(())
    }
}
