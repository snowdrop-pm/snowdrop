use std::num::ParseIntError;

use log::debug;
use miette::{Diagnostic, Result};
use octocrab::{models::repos::Release, Octocrab};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use thiserror::Error;

pub const CURRENT_PROTOCOL_VERSION: u8 = 3;

pub struct IndexClient {
    client: Client,
    pub index: String,
    pat: Option<String>,
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

    #[error("Failed to get latest GitHub Release for repo")]
    GitHubReleaseError(#[from] octocrab::Error),

    #[error("Expected protocol version {0}, got version {1}")]
    #[diagnostic(help("Try updating Snowdrop to the latest version"))]
    ProtocolVersionMismatch(u8, u8),

    #[error("Failed to parse protocol version")]
    ProtocolVersionParseError(#[from] ParseIntError),

    #[error("No GitHub PAT specified")]
    #[diagnostic(help("Run `snowflake auth` to set this up"))]
    NoPat,
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
            .await?
            .trim()
            .parse()?;
        debug!("Parsed proto version: {proto_version}");

        if proto_version != CURRENT_PROTOCOL_VERSION {
            debug!("Proto version being used by Snowdrop ({CURRENT_PROTOCOL_VERSION}) != index server proto version ({proto_version}), bailing out...");
            return Err(IndexClientError::ProtocolVersionMismatch(
                CURRENT_PROTOCOL_VERSION,
                proto_version,
            ));
        }

        Ok(Self {
            client,
            index,
            pat: None,
        })
    }

    pub fn with_pat(&mut self, pat: Option<String>) -> &mut IndexClient {
        self.pat = pat;
        self
    }

    pub async fn get_package(&self, name: &str) -> Result<PackageMetadata, IndexClientError> {
        let index = &self.index;
        let endpoint = format!("{index}/packages/{name}.json");
        log::debug!("Index server endpoint for package `{name}` is `{endpoint}`");

        if self.pat.is_none() {
            return Err(IndexClientError::NoPat);
        }

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

        let mut metadata = http_response.json::<PackageMetadata>().await?;
        metadata.pat(self.pat.clone());
        Ok(metadata)
    }

    pub async fn get_names(&self) -> Result<Vec<String>> {
        let index = &self.index;
        let endpoint = format!("{index}/names.json");
        log::debug!("Index server endpoint for package name list is `{endpoint}`");

        Ok(self
            .client
            .get(endpoint)
            .send()
            .await
            .map_err(IndexClientError::RequestError)?
            .json::<Vec<String>>()
            .await
            .map_err(IndexClientError::RequestError)?)
    }
}

#[derive(Deserialize, Debug)]
pub struct PackageMetadata {
    pub name: String,
    pub pretty_name: String,
    pub repo: [String; 2],
    pub naming_scheme: String,
    pat: Option<String>,
}

impl PackageMetadata {
    pub async fn get_latest_release(&self) -> Result<Release, IndexClientError> {
        let [owner, repo] = &self.repo;

        Ok(octocrab(&self.pat.clone().unwrap())?
            .repos(owner, repo)
            .releases()
            .get_latest()
            .await?)
    }

    fn pat(&mut self, pat: Option<String>) -> &mut Self {
        self.pat = pat;
        self
    }
}

fn octocrab(pat: &String) -> Result<Octocrab, IndexClientError> {
    Ok(Octocrab::builder().personal_token(pat.to_string()).build()?)
}
