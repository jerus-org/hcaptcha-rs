use std::collections::HashSet;
use crate::error::Code;

#[derive(Debug, Deserialize)]
pub struct RecaptchaResponse {
    pub success: bool,
    #[serde(rename="error-codes")]
    pub error_codes: Option<HashSet<Code>>
}

#[test]
fn decoding_test() {
    use serde_json::json;
    use crate::error::Code::*;

    let response = json!({
        "success": true,
        "error-codes": ["missing-input-secret", "foo"],
    });
    let response: RecaptchaResponse = serde_json::from_value(response).unwrap();
    
    assert!(response.success);
    assert!(response.error_codes.is_some());

    let errors = response.error_codes.unwrap();
    assert!(errors.len() == 2);
    assert!(errors.contains(&MissingSecret));
    assert!(errors.contains(&Unknown("foo".to_string())));
}
