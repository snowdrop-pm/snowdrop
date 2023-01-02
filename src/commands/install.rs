use miette::Result;

pub struct Install;

impl Install {
    pub fn execute(package: &String, dry_run: &bool) -> Result<()> {
        Ok(())
    }
}