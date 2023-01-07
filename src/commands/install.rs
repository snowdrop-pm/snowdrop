use miette::Result;

use crate::config::get_config;
use crate::defaults::DEFAULT_PACKAGE_INDEX;

pub struct Install;

impl Install {
    pub async fn execute(package: &String, dry_run: &bool) -> Result<()> {
        let config = get_config()?;
        let index = config
            .get_string("package_index")
            .unwrap_or_else(|_| DEFAULT_PACKAGE_INDEX.to_string());
        dbg!(index);

        Ok(())
    }
}
