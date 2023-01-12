use colored::Colorize;
use index_client::IndexClient;
use miette::Result;

use crate::config::get_config;
use crate::defaults::DEFAULT_PACKAGE_INDEX;

use log::{debug, info};

pub struct Install;

impl Install {
    pub async fn execute(package: &str, _dry_run: &bool) -> Result<()> {
        let config = get_config()?;
        let index = config
            .get_string("package_index")
            .unwrap_or_else(|_| DEFAULT_PACKAGE_INDEX.to_string());
        let index_client =
            IndexClient::from_index_and_user_version(index, env!("CARGO_PKG_VERSION"))?;

        debug!("Fetching package metadata for package `{package}`.");
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
            release.name.unwrap_or("unspecified".to_string()).bold(),
            format!("(release ID: {release_id})").black().bold()
        );

        Ok(())
    }
}
