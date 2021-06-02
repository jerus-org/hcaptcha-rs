use crate::request::HcaptchaRequest;
use crate::request::HcaptchaServerResponse;
use crate::HcaptchaError;
#[cfg(feature = "logging")]
use log::debug;
use std::collections::HashSet;
use std::net::IpAddr;
use uuid::Uuid;

/// Builder to compose a request for the hcaptcha validation endpoint, verify
/// the request and read the additional information that may be supplied in
/// the response.
#[derive(Debug, Default)]
pub struct Hcaptcha {
    request: HcaptchaRequest,
    response: HcaptchaServerResponse,
}

impl Hcaptcha {
    /// Create a new Hcaptcha Request
    ///
    /// # Example 1
    /// In the following example the new method returns an error as the secret
    /// and response inputs are blank.
    /// The new method validates these without calling the hcaptcha API.  
    /// ```should_panic
    /// use hcaptcha::Hcaptcha;
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    /// let secret = ""; // your secret key
    /// let token = "";  // user's token
    ///
    /// Hcaptcha::new(secret, token)? // <- returns error
    ///     .verify()
    ///     .await
    /// # }
    /// ```
    /// # Example 2
    /// In the following example the call to new returns an error as the
    /// response input is blank.
    /// The new method validates these without calling the hcaptcha API.
    /// ```should_panic
    /// use hcaptcha::Hcaptcha;
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    /// let secret = ""; // your secret key
    /// let token = "";  // user's token
    ///
    /// Hcaptcha::new(secret, token)? // <- returns error
    ///     .verify()
    ///     .await
    /// # }
    /// ```
    /// # Example 3
    /// This example likely fails as the token is specified is unlikely to be
    /// a valid token. The new function will however pass the input as it
    /// only validates that the token is not blank or empty.
    /// ```should_panic no_run
    /// use hcaptcha::Hcaptcha;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    /// let secret = "0x0000000000000000000000000000000000000000"; // your secret key
    /// let token = "this_is_likely_invalid";  // user's token
    ///
    /// Hcaptcha::new(secret, token)?
    ///     .verify()
    ///     .await  // <- likely returns InvalidResponse error
    /// # }
    /// ```
    #[cfg(feature = tracing)]
    #[tracing::instrument(name = "Construct new request with secret and client response.")]
    #[allow(dead_code)]
    pub fn new(secret: &str, response: &str) -> Result<Hcaptcha, HcaptchaError> {
        let r = HcaptchaRequest::new(secret, response);
        Ok(Hcaptcha {
            request: r?,
            ..Hcaptcha::default()
        })
    }

    /// Specify the optional ip address value
    ///
    /// # Example
    /// This example will most likely fail as the client response token will be
    /// reported by the hcaptcah API as invalid.
    /// The sample serves to illustrate the construction of a data structure
    /// that includes the user ip address using the set_user_ip method.
    /// ```should_panic no_run
    /// use hcaptcha::Hcaptcha;
    /// use std::net::{IpAddr, Ipv4Addr};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    /// let secret = "0x0000000000000000000000000000000000000000"; // your secret key
    /// let token = "this_is_likely_invalid";  // user's token
    /// let user_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 17));
    ///
    /// Hcaptcha::new(secret, token)?
    ///     .set_user_ip(&user_ip)
    ///     .verify()
    ///     .await // <- likely returns InvalidResponse error
    /// # }
    /// ```
    #[cfg(feature = tracing)]
    #[tracing::instrument(name = "Add client IP to request.")]
    #[allow(dead_code)]
    pub fn set_user_ip(mut self, user_ip: &IpAddr) -> Hcaptcha {
        self.request.set_user_ip(user_ip);
        self
    }
    /// Specify the optional site key value
    ///
    /// # Example
    /// This example will most likely fail as the client response token will be
    /// reported by the hcaptcah API as invalid.
    /// The sample serves to illustrate the construction of a data structure
    /// that includes the site_key using the set_site_key method.
    /// ```should_panic no_run
    /// use hcaptcha::Hcaptcha;
    /// use uuid::Uuid;
    /// # use hcaptcha::HcaptchaError;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), HcaptchaError> {
    /// let secret = "0x0000000000000000000000000000000000000000"; // your secret key
    /// let token = "this_is_likely_invalid";  // user's token
    /// let site_key = Uuid::parse_str("10000000-ffff-ffff-ffff-000000000001")?;
    ///
    /// Hcaptcha::new(secret, token)?
    ///     .set_site_key(&site_key)
    ///     .verify()
    ///     .await // <- likely returns InvalidResponse error
    /// # }
    /// ```
    #[cfg(feature = tracing)]
    #[tracing::instrument(name = "Add site key to request.")]
    #[allow(dead_code)]
    pub fn set_site_key(mut self, site_key: &Uuid) -> Hcaptcha {
        self.request.set_site_key(site_key);
        self
    }
    /// Verify a hcaptcha user response
    ///
    /// # Example
    /// The example illustrates a call to verify the full data structure, including
    /// the optional user_ip and site_key fields. The example will yield as error
    /// as the token value will not be valid.
    /// ```should_panic no_run
    /// # use hcaptcha::Hcaptcha;
    /// # use hcaptcha::Code::*;
    /// # use std::net::{IpAddr, Ipv4Addr};
    /// # use uuid::Uuid;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError>{
    /// let secret = "0x0000000000000000000000000000000000000000";
    /// let token = "this_is_likely_invalid";  // user's token
    /// let user_ip = IpAddr::V4(Ipv4Addr::new(123, 123, 123, 123));
    /// let site_key = Uuid::parse_str("10000000-ffff-ffff-ffff-000000000001")?;
    ///
    /// let response = Hcaptcha::new(secret, token)?
    ///                 .set_user_ip(&user_ip)
    ///                 .set_site_key(&site_key)
    ///                 .verify()
    ///                 .await;
    ///
    /// assert!(response.is_err());
    /// Ok(())
    /// # }
    /// ```
    #[cfg(feature = tracing)]
    #[tracing::instrument(name = "Submit request to Hcaptcha for verification.")]
    pub async fn verify(&mut self) -> Result<(), HcaptchaError> {
        #[cfg(feature = "logging")]
        debug!("State of request: {:?}", self);
        self.response = self.request.verify().await?;
        println!("verify response: {:#?}", &self.response);

        match (self.response.success(), self.response.error_codes()) {
            (true, _) => Ok(()),
            (false, Some(errors)) => Err(HcaptchaError::Codes(errors)),
            (false, _) => Err(HcaptchaError::Codes(HashSet::new())),
        }
    }

    /// Get the hostname returned in the response
    /// Option string containig the hostname of the site
    /// where the captcha was solved
    ///
    #[cfg(feature = tracing)]
    #[tracing::instrument(name = "Return hostname from the Hcaptcha API response.")]
    #[allow(dead_code)]
    pub fn hostname(&self) -> Option<String> {
        self.response.hostname()
    }

    /// Get the timestamp from the response
    /// Option string containing the timestamp of the captcha
    /// (ISO format yyyy-MM-dd'T'HH:mm:ssZZ)
    ///
    #[cfg(feature = tracing)]
    #[tracing::instrument(name = "Return timestampt from the Hcaptcha API response.")]
    #[allow(dead_code)]
    pub fn timestamp(&self) -> Option<String> {
        self.response.timestamp()
    }

    /// Get the credit flag
    /// Optional flag showing whether the response will be credited
    ///
    #[cfg(feature = tracing)]
    #[tracing::instrument(name = "Return credit from the Hcaptcha API response.")]
    #[allow(dead_code)]
    pub fn credit(&self) -> Option<bool> {
        self.response.credit()
    }

    /// Get the score
    ///
    /// ENTERPRISE feature: a score denoting malicious activity.
    ///
    #[cfg(feature = tracing)]
    #[tracing::instrument(name = "Return score from the Hcaptcha API response.")]
    #[cfg(feature = "enterprise")]
    #[allow(dead_code)]
    pub fn score(&self) -> Option<f32> {
        self.response.score()
    }

    /// Get the reasons for the score
    ///
    /// ENTERPRISE feature: reason(s) for score.
    /// See [BotStop.com](https://BotStop.com) for details.
    ///
    #[cfg(feature = tracing)]
    #[tracing::instrument(name = "Return score reason from the Hcaptcha API response.")]
    #[cfg(feature = "enterprise")]
    #[allow(dead_code)]
    pub fn score_reason(&self) -> Option<HashSet<String>> {
        self.response.score_reason()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn test_response() -> HcaptchaServerResponse {
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
    fn success_test() {
        let response = test_response();

        assert_eq!(response.success(), true);
    }

    #[test]
    fn timestamp_test() {
        let response = test_response();

        assert_eq!(
            response.timestamp(),
            Some("2020-11-11T23:27:00Z".to_owned())
        );
    }

    #[test]
    fn hostname_test() {
        let response = test_response();

        assert_eq!(response.hostname(), Some("my-host.ie".to_owned()));
    }

    #[test]
    fn credit_test() {
        let response = test_response();

        assert_eq!(response.credit(), Some(false));
    }

    #[test]
    fn error_codes_test() {
        let response = test_response();

        assert!(response.error_codes().is_some());
        if let Some(hash_set) = response.error_codes() {
            assert_eq!(hash_set.len(), 2)
        }
    }

    #[test]
    fn score_test() {
        let response = test_response();

        assert!(response.score().is_none());
    }

    #[test]
    fn score_reason_test() {
        let response = test_response();
        println!("The response: {:?}", response);

        assert!(response.score_reason().is_some());
        if let Some(hash_set) = response.score_reason() {
            assert!(hash_set.is_empty())
        }
    }
}
