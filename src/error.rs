use std::error::{Error, FromError};
use std::fmt::{self, Display, Debug};
use std::collections::HashSet;
use std::io;
use rustc_serialize::json::DecoderError;
use hyper::HttpError;
use super::RecaptchaErrorCode;

pub enum RecaptchaError {
    Codes(HashSet<RecaptchaErrorCode>),
    Decoder(DecoderError),
    Http(HttpError),
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

impl FromError<DecoderError> for RecaptchaError {
    fn from_error(err: DecoderError) -> RecaptchaError {
        RecaptchaError::Decoder(err)
    }
}

impl FromError<HttpError> for RecaptchaError {
    fn from_error(err: HttpError) -> RecaptchaError {
        RecaptchaError::Http(err)
    }
}

impl FromError<io::Error> for RecaptchaError {
    fn from_error(err: io::Error) -> RecaptchaError {
        RecaptchaError::Io(err)
    }
}
