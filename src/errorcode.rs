use serde::{Deserializer, Deserialize};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum RecaptchaErrorCode {
    MissingSecret,
    InvalidSecret,
    MissingResponse,
    InvalidResponse,
    BadRequest,
    Unknown(String)
}

impl<'de> Deserialize<'de> for RecaptchaErrorCode {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let code = String::deserialize(de)?;
        Ok(match &*code {
            "missing-input-secret" => RecaptchaErrorCode::MissingSecret,
            "invalid-input-secret" => RecaptchaErrorCode::InvalidSecret,
            "missing-input-response" => RecaptchaErrorCode::MissingResponse,
            "invalid-input-response" => RecaptchaErrorCode::InvalidResponse,
            "bad-request" => RecaptchaErrorCode::BadRequest,
            _ => RecaptchaErrorCode::Unknown(code),
        })
    }
}
