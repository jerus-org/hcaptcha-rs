use crate::error::Code;
use serde_derive::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct HcaptchaResponse {
    success: bool,
    challenge_ts: Option<String>, //yyyy-MM-dd'T'HH:mm:ssZZ
    hostname: Option<String>,
    credit: Option<bool>,
    #[serde(rename = "error-codes")]
    error_codes: Option<HashSet<Code>>,
    score: Option<f32>,
    score_reason: Option<HashSet<String>>,
}

impl HcaptchaResponse {
    #[allow(dead_code)]
    pub fn get_success(&self) -> bool {
        self.success
    }

    #[allow(dead_code)]
    pub fn get_hostname(&self) -> Option<String> {
        self.hostname.clone()
    }
    #[allow(dead_code)]
    pub fn get_timestamp(&self) -> Option<String> {
        self.challenge_ts.clone()
    }
    #[allow(dead_code)]
    pub fn get_credit(&self) -> Option<bool> {
        self.credit
    }
    #[allow(dead_code)]
    pub fn get_error_codes(&self) -> Option<HashSet<Code>> {
        self.error_codes.clone()
    }
    #[allow(dead_code)]
    pub fn get_score(&self) -> Option<f32> {
        self.score
    }
    #[allow(dead_code)]
    pub fn get_score_reason(&self) -> Option<HashSet<String>> {
        self.score_reason.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn decoding_test() {
        use crate::error::Code::*;

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

    fn test_response() -> HcaptchaResponse {
        let response = json!({
            "success": true,
            "challenge_ts": "2020-11-11T23:27:00Z",
            "hostname": "my-host.ie",
            "credit": false,
            "error-codes": ["missing-input-secret", "foo"],
            "score": null,
            "score_reason": [],
        });
        serde_json::from_value(response).unwrap()
    }

    #[test]
    fn get_success_test() {
        let response = test_response();

        assert_eq!(response.get_success(), true);
    }

    #[test]
    fn get_timestamp_test() {
        let response = test_response();

        assert_eq!(
            response.get_timestamp(),
            Some("2020-11-11T23:27:00Z".to_owned())
        );
    }

    #[test]
    fn get_hostname_test() {
        let response = test_response();

        assert_eq!(response.get_hostname(), Some("my-host.ie".to_owned()));
    }

    #[test]
    fn get_credit_test() {
        let response = test_response();

        assert_eq!(response.get_credit(), Some(false));
    }

    #[test]
    fn get_error_codes_test() {
        let response = test_response();

        assert!(response.get_error_codes().is_some());
        match response.get_error_codes() {
            Some(hash_set) => assert_eq!(hash_set.len(), 2),
            None => {}
        }
    }

    #[test]
    fn get_score_test() {
        let response = test_response();

        assert!(response.get_score().is_none());
    }

    #[test]
    fn get_score_reason_test() {
        let response = test_response();
        println!("The response: {:?}", response);

        assert!(response.get_score_reason().is_some());
        match response.get_score_reason() {
            Some(hash_set) => assert!(hash_set.is_empty()),
            None => {}
        }
    }
}
