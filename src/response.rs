use std::collections::HashSet;
use rustc_serialize::{Decodable, Decoder};
use super::RecaptchaErrorCode;

#[derive(Debug)]
pub struct RecaptchaResponse {
    pub success: bool,
    pub error_codes: Option<HashSet<RecaptchaErrorCode>>
}

impl Decodable for RecaptchaResponse {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<RecaptchaResponse, D::Error> {
        decoder.read_struct("RecaptchaResponse", 2, |d|
            Ok(RecaptchaResponse {
                success: try!(d.read_struct_field("success", 0, Decodable::decode)),
                error_codes: try!(d.read_struct_field("error-codes", 1, Decodable::decode))
            })
        )
    }
}

#[test]
fn decoding_test() {
    use rustc_serialize::json::decode;
    use super::RecaptchaErrorCode::*;

    let resp = decode::<RecaptchaResponse>(r#"{
        "success": true,
        "error-codes": ["missing-input-secret", "foo"]
    }"#).unwrap();
    
    assert!(resp.success);
    assert!(resp.error_codes.is_some());

    let errors = resp.error_codes.unwrap();
    assert!(errors.len() == 2);
    assert!(errors.contains(&MissingSecret));
    assert!(errors.contains(&Unknown("foo".to_string())));
}
