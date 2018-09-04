use std::collections::HashSet;
use error::Code;

#[derive(Debug, Deserialize)]
pub struct RecaptchaResponse {
    pub success: bool,
    #[serde(rename="error-codes")]
    pub error_codes: Option<HashSet<Code>>
}

#[test]
fn decoding_test() {
    extern crate serde_json as json;
    use error::Code::*;

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
