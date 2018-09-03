use std::error::Error;
use std::fmt::{self, Display, Debug};
use std::collections::HashSet;
use std::io;
use std::convert::From;
use rustc_serialize::json::DecoderError;
use super::RecaptchaErrorCode;
use reqwest;

pub enum RecaptchaError {
    Codes(HashSet<RecaptchaErrorCode>),
    Decoder(DecoderError),
    Http(reqwest::Error),
    Io(io::Error),
}

impl Display for RecaptchaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::RecaptchaError::*;
        match self {
            &Codes(ref errs) => write!(f, "{} ({:?})", self.description(), errs),
            &Decoder(ref e) => Display::fmt(e, f),
            &Http(ref e) => Display::fmt(e, f),
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
            &Decoder(ref e) => Some(e),
            &Http(ref e) => Some(e),
            &Io(ref e) => Some(e)
        }
    }
}

impl From<DecoderError> for RecaptchaError {
    fn from(err: DecoderError) -> RecaptchaError {
        RecaptchaError::Decoder(err)
    }
}

impl From<reqwest::Error> for RecaptchaError {
    fn from(err: reqwest::Error) -> RecaptchaError {
        RecaptchaError::Http(err)
    }
}

impl From<io::Error> for RecaptchaError {
    fn from(err: io::Error) -> RecaptchaError {
        RecaptchaError::Io(err)
    }
}
