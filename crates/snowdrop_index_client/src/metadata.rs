use octocrab::{models::repos::Release, Octocrab};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::error::IndexClientError;

#[derive(Deserialize, Debug)]
pub struct PackageMetadata {
    pub name: String,
    pub pretty_name: String,
    pub repo: [String; 2],
    pub naming_scheme: String,
    pub(crate) pat: SecretString,
}

impl PackageMetadata {
    pub async fn get_latest_release(&self) -> Result<Release, IndexClientError> {
        let [owner, repo] = &self.repo;

        Ok(octocrab(&self.pat)?.repos(owner, repo).releases().get_latest().await?)
    }
}

fn octocrab(pat: &SecretString) -> Result<Octocrab, IndexClientError> {
    Ok(Octocrab::builder()
        .personal_token(pat.expose_secret().to_string())
        .build()?)
}
