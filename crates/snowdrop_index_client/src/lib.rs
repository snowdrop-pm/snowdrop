use std::num::ParseIntError;

use log::debug;
use miette::{Diagnostic, Result};
use octocrab::models::repos::Release;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use thiserror::Error;

pub const CURRENT_PROTOCOL_VERSION: u8 = 2;

pub struct IndexClient {
    client: Client,
    pub index: String,
}

#[derive(Error, Diagnostic, Debug)]
pub enum IndexClientError {
    #[error("An error occured while sending or receiving a request from the index server")]
    RequestError(#[from] reqwest::Error),

    #[error("The index server returned a status code of `{0}`")]
    #[diagnostic(help(
        "This is very likely a problem with the index server, try contacting the server administrator"
    ))]
    StatusCodeNotOk(StatusCode),

    #[error("Failed to initialize TLS backend")]
    TlsBackendInitError,

    #[error("Package not found")]
    PackageNotFound,

    #[error("Failed to get latest GitHub Release for repo `{0}/{1}`")]
    GitHubReleaseError(String, String, octocrab::Error),

    #[error("Expected protocol version {0}, got version {1}")]
    #[diagnostic(help("Try updating Snowdrop to the latest version"))]
    ProtocolVersionMismatch(u8, u8),

    #[error("Failed to parse protocol version")]
    ProtocolVersionParseError(#[from] ParseIntError),
}

impl IndexClient {
    pub async fn from_index_and_user_version(index: String, user_version: &str) -> Result<Self, IndexClientError> {
        let Ok(client) = Client::builder()
            .user_agent(format!(
                "SnowdropIndexClient/{} SnowdropCLI/{user_version}",
                env!("CARGO_PKG_VERSION")
            ))
            .build() else {
                return Err(IndexClientError::TlsBackendInitError)
            };
        let proto_version = client
            .get(format!("{index}/proto_version"))
            .send()
            .await?
            .text()
            .await?;
        debug!("Protocol version TEXT returned: {proto_version}");
        let proto_version = "2".parse()?;
        debug!("Parsed proto version: {proto_version}");

        if proto_version != CURRENT_PROTOCOL_VERSION {
            debug!("Proto version being used by Snowdrop ({CURRENT_PROTOCOL_VERSION}) != index server proto version ({proto_version}), bailing out...");
            return Err(IndexClientError::ProtocolVersionMismatch(
                CURRENT_PROTOCOL_VERSION,
                proto_version,
            ));
        }

        Ok(Self { client, index })
    }

    pub async fn get_package(&self, name: &str) -> Result<PackageMetadata, IndexClientError> {
        let index = &self.index;
        let endpoint = format!("{index}/packages/{name}.json");
        log::debug!("Index server endpoint for package `{name}` is `{endpoint}`");

        let http_response = self
            .client
            .get(endpoint)
            .send()
            .await
            .map_err(IndexClientError::RequestError)?;

        if let Err(err) = http_response.error_for_status_ref() {
            if err.status() == Some(StatusCode::NOT_FOUND) {
                debug!("The index server returned a 404, quitting...");
                return Err(IndexClientError::PackageNotFound);
            }
            return Err(IndexClientError::StatusCodeNotOk(err.status().unwrap()));
        }

        Ok(http_response.json::<PackageMetadata>().await?)
    }
}

#[derive(Deserialize, Debug)]
pub struct PackageMetadata {
    pub name: String,
    pub pretty_name: String,
    pub repo: [String; 2],
}

impl PackageMetadata {
    pub async fn get_latest_release(&self) -> Result<Release, IndexClientError> {
        let [owner, repo] = &self.repo;

        match octocrab::instance().repos(owner, repo).releases().get_latest().await {
            Ok(release) => Ok(release),
            Err(err) => Err(IndexClientError::GitHubReleaseError(
                owner.to_string(),
                repo.to_string(),
                err,
            )),
        }
    }
}
