use std::error::Error;
use std::fmt::{self, Display, Debug};
use std::collections::HashSet;
use std::io;
use std::convert::From;
use super::RecaptchaErrorCode;
use reqwest;

pub enum RecaptchaError {
    Codes(HashSet<RecaptchaErrorCode>),
    Reqwest(reqwest::Error),
    Io(io::Error),
}

impl Display for RecaptchaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::RecaptchaError::*;
        match self {
            &Codes(ref errs) => write!(f, "{} ({:?})", self.description(), errs),
            &Reqwest(ref e) => Display::fmt(e, f),
            &Io(ref e) => Display::fmt(e, f)
        }
    }
}

impl Debug for RecaptchaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        <RecaptchaError as Display>::fmt(self, f)
    }
}

impl Error for RecaptchaError {
    fn description(&self) -> &str {
        "recaptcha verification failed"
    }

    fn cause(&self) -> Option<&Error> {
        use self::RecaptchaError::*;
        match self {
            &Codes(_) => None,
            &Reqwest(ref e) => Some(e),
            &Io(ref e) => Some(e)
        }
    }
}

impl From<reqwest::Error> for RecaptchaError {
    fn from(err: reqwest::Error) -> RecaptchaError {
        RecaptchaError::Reqwest(err)
    }
}

impl From<io::Error> for RecaptchaError {
    fn from(err: io::Error) -> RecaptchaError {
        RecaptchaError::Io(err)
    }
}
