use std::error::{Error, FromError};
use std::fmt::{self, Display, Debug};
use std::collections::HashSet;
use std::old_io::IoError;
use rustc_serialize::json::DecoderError;
use hyper::HttpError;
use super::RecaptchaErrorCode;

pub enum RecaptchaError {
    Codes(HashSet<RecaptchaErrorCode>),
    Wrapped(Box<Error>)
}

impl Display for RecaptchaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::RecaptchaError::*;
        match *self {
            Codes(ref errs) => write!(f, "{} ({:?})", self.description(), errs),
            Wrapped(ref err) => write!(f, "{} ({})", self.description(), err)
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
        match *self {
            Codes(_) => None,
            Wrapped(ref err) => Some(&**err)
        }
    }
}

impl FromError<DecoderError> for RecaptchaError {
    fn from_error(err: DecoderError) -> RecaptchaError {
        RecaptchaError::Wrapped(Box::new(err))
    }
}

impl FromError<HttpError> for RecaptchaError {
    fn from_error(err: HttpError) -> RecaptchaError {
        RecaptchaError::Wrapped(Box::new(err))
    }
}

impl FromError<IoError> for RecaptchaError {
    fn from_error(err: IoError) -> RecaptchaError {
        RecaptchaError::Wrapped(Box::new(err))
    }
}
