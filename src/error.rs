use std::collections::HashSet;
use std::io;
use super::RecaptchaErrorCode;
use reqwest;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{:?}", _0)]
    Codes(HashSet<RecaptchaErrorCode>),
    #[fail(display = "{}", _0)]
    Reqwest(#[cause] reqwest::Error),
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
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
