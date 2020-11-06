use failure::Fail;
use reqwest;
use serde::{Deserialize, Deserializer};
use std::collections::HashSet;
use std::io;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{:?}", _0)]
    Codes(HashSet<Code>),
    #[fail(display = "{}", _0)]
    Reqwest(#[cause] reqwest::Error),
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    Json(#[cause] serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Reqwest(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Code {
    MissingSecret,
    InvalidSecret,
    MissingResponse,
    InvalidResponse,
    BadRequest,
    InvalidAlreadySeen,
    SiteSecretMismatch,
    Unknown(String),
}

impl<'de> Deserialize<'de> for Code {
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
