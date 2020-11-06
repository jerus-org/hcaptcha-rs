use crate::error::Code;
use serde_derive::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Default, Deserialize)]
pub struct HcaptchaResponse {
    pub success: bool,
    pub challenge_ts: Option<String>, // timestamp of the captcha (ISO format yyyy-MM-dd'T'HH:mm:ssZZ)
    pub hostname: Option<String>,
    pub credit: Option<bool>,
    #[serde(rename = "error-codes")]
    pub error_codes: Option<HashSet<Code>>,
    pub score: Option<f32>,
    pub score_reason: Option<HashSet<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoding_test() {
        use crate::error::Code::*;
        use serde_json::json;

        let response = json!({
            "success": true,
            "error-codes": ["missing-input-secret", "foo"],
            "hostname": "hostname"
        });
        let response: HcaptchaResponse = serde_json::from_value(response).unwrap();

        assert!(response.success);
        assert!(response.error_codes.is_some());

        let errors = response.error_codes.unwrap();
        assert!(errors.len() == 2);
        assert!(errors.contains(&MissingSecret));
        assert!(errors.contains(&Unknown("foo".to_string())));
    }
}
