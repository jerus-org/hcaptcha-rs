//! Error types for hcaptcha

use serde::{Deserialize, Deserializer};
use std::collections::HashSet;
use std::fmt;
use std::io;
use thiserror::Error;

/// The error type for hcaptcha.
/// Provides error types to capture error codes from the Hcaptcha API
/// and errors output from crates used by the library.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum HcaptchaError {
    /// Error(s) returned from the hcaptcha API and mapped to the [Code] enum.
    #[error("{0:?}")]
    Codes(HashSet<Code>),
    /// Error returned by reqwest
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    /// Error returned by io
    #[error("{0}")]
    Io(#[from] io::Error),
    /// Error returned by serde_json
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    /// Error returned by serde_urlencoded
    #[error("{0}")]
    UrlEncoded(#[from] serde_urlencoded::ser::Error),
    /// Error returned by uuid
    #[error("{0}")]
    Uuid(#[from] uuid::Error),
    /// Error returned by url parser
    #[error("{0}")]
    Url(#[from] url::ParseError),
}

/// Error code mapping for the error responses from the hcaptcha API.
/// Returned in the [HcaptchaError] type.
#[non_exhaustive]
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Code {
    /// Secret key is missing.
    MissingSecret,
    /// Secret key is invalid or malformed.
    InvalidSecret,
    /// User ip string is missing.
    MissingUserIp,
    /// User ip is invalid or malformed.
    InvalidUserIp,
    /// The response parameter (verification token) is missing.
    MissingResponse,
    /// The response parameter (verification token) is invalid or malformed.
    InvalidResponse,
    /// The request is invalid or malformed.
    BadRequest,
    /// The response parameter has already been checked, or has another issue.
    InvalidAlreadySeen,
    /// The sitekey is not registered with the provided secret.
    SiteSecretMismatch,
    /// Extended secret check reports that the secret string is the wrong length.
    InvalidSecretExtWrongLen,
    /// Extended secret check reports that the secret string is not a hex string.
    InvalidSecretExtNotHex,
    /// Collect any new error codes issued by the API.
    Unknown(String),
}

impl<'de> Deserialize<'de> for Code {
    /// Custom deserialize to map the hcaptcha API error codes for reporting as
    /// a [Code] in [HcaptchaError].
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = String::deserialize(de)?;
        Ok(match &*code {
            "missing-input-secret" => Code::MissingSecret,
            "invalid-input-secret" => Code::InvalidSecret,
            "missing-input-response" => Code::MissingResponse,
            "invalid-input-response" => Code::InvalidResponse,
            "bad-request" => Code::BadRequest,
            "invalid-or-already-seen-response" => Code::InvalidAlreadySeen,
            "sitekey-secret-mismatch" => Code::SiteSecretMismatch,
            _ => Code::Unknown(code),
        })
    }
}

impl fmt::Display for Code {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Code::MissingSecret => write!(f, "Secret key is missing."),
            Code::InvalidSecret => write!(f, "Secret key is invalid or malformed."),
            Code::MissingUserIp => write!(f, "User ip string is missing."),
            Code::InvalidUserIp => write!(f, "User ip string is invalid."),
            Code::InvalidSecretExtWrongLen => {
                write!(f, "Secret key is not the correct length.")
            }
            Code::InvalidSecretExtNotHex => write!(f, "Secret key is not a hex string."),
            Code::MissingResponse => {
                write!(f, "The response parameter (verification token) is missing.")
            }
            Code::InvalidResponse => write!(
                f,
                "The response parameter (verification token) is invalid or malformed."
            ),
            Code::BadRequest => write!(f, "The request is invalid or malformed."),
            Code::InvalidAlreadySeen => write!(
                f,
                "The response parameter has already been checked, or has another issue."
            ),
            Code::SiteSecretMismatch => {
                write!(f, "The sitekey is not registered with the provided secret.")
            }
            Code::Unknown(e) => write!(f, "Unkown error: {}", e),
        }
    }
}
