use log::debug;
use miette::Result;
use reqwest::{Client, StatusCode};
use secrecy::SecretString;

mod error;
mod metadata;
use error::IndexClientError;
use metadata::PackageMetadata;

pub const CURRENT_PROTOCOL_VERSION: u8 = 3;

pub struct IndexClient {
    client: Client,
    pub index: String,
    pat: SecretString,
}

impl IndexClient {
    pub async fn new(index: String, user_version: &str, pat: SecretString) -> Result<Self, IndexClientError> {
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

        Ok(Self { client, index, pat })
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

        let mut metadata = http_response.json::<PackageMetadata>().await?;
        metadata.pat = self.pat.clone();
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
