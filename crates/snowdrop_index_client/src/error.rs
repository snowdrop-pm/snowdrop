use std::num::ParseIntError;

use miette::Diagnostic;
use reqwest::StatusCode;
use thiserror::Error;

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

    #[error("No GitHub PAT set")]
    #[diagnostic(help("Run `snowdrop auth` to set this up"))]
    NoPat
}
