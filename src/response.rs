use crate::error::Code;
use serde_derive::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
pub struct HcaptchaResponse {
    pub success: bool,
    #[serde(rename = "error-codes")]
    pub error_codes: Option<HashSet<Code>>,
    pub hostname: String,
    pub credit: bool,
    pub challenge_ts: String, // timestamp of the captcha (ISO format yyyy-MM-dd'T'HH:mm:ssZZ)
                              // pub score: f32,
                              // pub score_reason, Option<HashSet<Scores>>,
}

/*
{
    "success": true|false,
    "challenge_ts": timestamp, // timestamp of the captcha (ISO format yyyy-MM-dd'T'HH:mm:ssZZ)
    "hostname": string,        // the hostname of the site where the captcha was solved
    "credit": true|false,      // optional: whether the response will be credited
    "error-codes": [...]       // optional: any error codes
    "score": float,            // ENTERPRISE feature: a score denoting malicious activity.
    "score_reason": [...]      // ENTERPRISE feature: reason(s) for score. See BotStop.com for details.
 }
*/

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
