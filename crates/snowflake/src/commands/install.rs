use index_client::IndexClient;
use miette::Result;

use crate::config::get_config;
use crate::defaults::DEFAULT_PACKAGE_INDEX;

pub struct Install;

impl Install {
    pub async fn execute(package: &str, dry_run: &bool) -> Result<()> {
        let config = get_config()?;
        let index = config
            .get_string("package_index")
            .unwrap_or_else(|_| DEFAULT_PACKAGE_INDEX.to_string());
        let index_client =
            IndexClient::from_index_and_user_version(index, env!("CARGO_PKG_VERSION"))?;
        index_client.get_package(package).await?;

        Ok(())
    }
}
