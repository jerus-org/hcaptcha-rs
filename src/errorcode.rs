use rustc_serialize::{Decodable, Decoder};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum RecaptchaErrorCode {
    MissingSecret,
    InvalidSecret,
    MissingResponse,
    InvalidResponse,
    Unknown(String)
}

impl Decodable for RecaptchaErrorCode {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<RecaptchaErrorCode, D::Error> {
        use self::RecaptchaErrorCode::*;
        Ok(match &*try!(decoder.read_str()) {
            "missing-input-secret" => MissingSecret,
            "invalid-input-secret" => InvalidSecret,
            "missing-input-response" => MissingResponse,
            "invalid-input-response" => InvalidResponse,
            unknown => Unknown(unknown.to_string())
        })
    }
}
