use std::collections::HashSet;
use super::RecaptchaErrorCode;

#[derive(Debug, Deserialize)]
pub struct RecaptchaResponse {
    pub success: bool,
    #[serde(rename="error-codes")]
    pub error_codes: Option<HashSet<RecaptchaErrorCode>>
}

#[test]
fn decoding_test() {
    extern crate serde_json as json;
    use super::RecaptchaErrorCode::*;

    let resp = json::from_str::<RecaptchaResponse>(r#"{
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
