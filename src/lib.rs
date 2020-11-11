pub mod error;
mod request;
mod response;

use log::debug;
use request::HcaptchaRequest;
use response::HcaptchaResponse;
use std::collections::HashSet;
use std::net::IpAddr;

pub use error::Error;
/// Hcaptcha
///
/// # Build the request and verify
///
/// The request must include your secret_key and the response submitted
/// with the posted data. The new method on Hcaptcha requires these values.
///
/// The additional optional values of the ip address from the user and the
/// your site_key can be added to the request using builder functions
///
/// Execute verify on the request to confirm.
///
/// # Additinal Response Data
///
/// Hcaptcha offers additional response data to the success flag and error
/// codes.
///
/// The following can be accessed after a succcessful verification:
///   timestamp  timestamp of the captcha (ISO: yyyy-MM-dd'T'HH:mm:ssZZ)
///   hostname   the hostname of the site where the captcha was solved
///   credit         whether the response will be credited
///   score          ENTERPRISE feature: a score denoting malicious activity.
///   score_reason   ENTERPRISE feature: reason(s) for score.
///
/// All these values are stored in an Option enum and may not always be present
/// in the response (see Hcaptcha documentation [here](https://docs.hcaptcha.com/#server)).
///
/// # Example
///
/// ```
/// use hcaptcha::Hcaptcha;
/// use std::net::{IpAddr, Ipv4Addr};
///
/// #[tokio::main]
/// async fn main() {
///     let remote_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 17));
///
///     let res = Hcaptcha::new("your_private_key", "user_response")
///                 .set_user_ip(&remote_ip)
///                 .verify()
///                 .await;
///
///     if res.is_ok() {
///         println!("Success");
///     } else {
///         println!("Failure");
///     }
/// }
/// ```

#[derive(Debug, Default)]
pub struct Hcaptcha {
    request: HcaptchaRequest,
    response: HcaptchaResponse,
}

impl Hcaptcha {
    /// Create a new Hcaptcha Request
    ///
    /// # Example
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() {
    /// # use hcaptcha::Hcaptcha;
    /// # #[allow(unused_imports)]
    /// # use tokio_compat_02::FutureExt;
    ///
    /// let secret = ""; // your secret key
    /// let token = "";  // user's token
    ///
    /// let hcaptcha = Hcaptcha::new(secret, token)
    ///                 .verify()
    ///                 .compat()
    ///                 .await;
    ///
    /// assert!(hcaptcha.is_err());
    ///
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn new(secret: &str, response: &str) -> Hcaptcha {
        let request = HcaptchaRequest::new(secret, response);

        Hcaptcha {
            request,
            ..Hcaptcha::default()
        }
    }

    /// Specify the optional ip address value
    ///
    /// # Example
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() {
    /// # use hcaptcha::Hcaptcha;
    /// # use std::net::{IpAddr, Ipv4Addr};
    ///
    /// let secret = ""; // your secret key
    /// let token = "";  // user's token
    /// let user_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 17));
    ///
    /// let hcaptcha = Hcaptcha::new(secret, token)
    ///                 .set_user_ip(&user_ip)
    ///                 .verify()
    ///                 .await;
    ///
    /// assert!(hcaptcha.is_err());
    ///
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn set_user_ip(mut self, user_ip: &IpAddr) -> Hcaptcha {
        self.request.set_user_ip(user_ip);
        self
    }

    /// Specify the optional site key value
    ///
    /// # Example
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() {
    /// # use hcaptcha::Hcaptcha;
    ///
    /// let secret = ""; // your secret key
    /// let token = "";  // user's token
    /// let site_key = "10000000-ffff-ffff-ffff-000000000001";
    ///
    /// let hcaptcha = Hcaptcha::new(secret, token)
    ///                 .set_site_key(site_key)
    ///                 .verify()
    ///                 .await;
    ///
    /// assert!(hcaptcha.is_err());
    ///
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn set_site_key(mut self, site_key: &str) -> Hcaptcha {
        self.request.set_site_key(site_key);
        self
    }

    /// Verify a hcaptcha user response
    ///
    /// # Example
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::Error> {
    /// # use hcaptcha::Hcaptcha;
    /// # use hcaptcha::error::Code::*;
    /// # use std::net::{IpAddr, Ipv4Addr};
    ///
    /// let secret = "0x0000000000000000000000000000000000000000";
    /// let token = "";
    /// let user_ip = IpAddr::V4(Ipv4Addr::new(123, 123, 123, 123));
    /// let site_key = "10000000-ffff-ffff-ffff-000000000001";
    ///
    /// let response = Hcaptcha::new(secret, token)
    ///                 .set_user_ip(&user_ip)
    ///                 .set_site_key(&site_key)
    ///                 .verify()
    ///                 .await;
    ///
    /// assert!(response.is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn verify(&mut self) -> Result<(), Error> {
        debug!("State of request: {:?}", self);
        self.response = self.request.verify().await?;

        match (self.response.success(), self.response.error_codes()) {
            (true, _) => Ok(()),
            (false, Some(errors)) => Err(Error::Codes(errors)),
            (false, _) => Err(Error::Codes(HashSet::new())),
        }
    }

    /// Get the hostname returned in the response
    /// Option string containig the hostname of the site
    /// where the captcha was solved
    ///
    #[allow(dead_code)]
    pub fn hostname(&self) -> Option<String> {
        self.response.hostname()
    }

    /// Get the timestamp from the response
    /// Option string containing the timestamp of the captcha
    /// (ISO format yyyy-MM-dd'T'HH:mm:ssZZ)
    ///
    #[allow(dead_code)]
    pub fn timestamp(&self) -> Option<String> {
        self.response.timestamp()
    }

    /// Get the credit flag
    /// Optional flag showing whether the response will be credited
    ///
    #[allow(dead_code)]
    pub fn credit(&self) -> Option<bool> {
        self.response.credit()
    }

    /// Get the score
    ///
    /// ENTERPRISE feature: a score denoting malicious activity.
    ///
    #[allow(dead_code)]
    pub fn score(&self) -> Option<f32> {
        self.response.score()
    }

    /// Get the reasons for the score
    ///
    /// ENTERPRISE feature: reason(s) for score.
    /// See [BotStop.com](https://BotStop.com) for details.
    ///
    #[allow(dead_code)]
    pub fn score_reason(&self) -> Option<HashSet<String>> {
        self.response.score_reason()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use error::Code::*;
    use error::Error::*;
    use serde_json::json;
    #[allow(unused_imports)]
    use tokio_compat_02::FutureExt;

    #[tokio::test]
    async fn test_invalid_secret_missing_response() {
        let response = Hcaptcha::new("", "").verify().compat().await;

        match response {
            Ok(()) => panic!("unexpected response: Ok(())"),
            Err(Codes(ref errors)) => {
                println!("Errors found {:?}", errors);
                assert!(errors.contains(&MissingSecret));
                assert!(errors.contains(&MissingResponse));
            }
            Err(e) => panic!("unexpected error: {}", e),
        };

        println!("{:?}", response);
    }

    #[tokio::test]
    async fn test_invalid_secret_missing_response_with_ip() {
        use std::net::Ipv4Addr;

        let user_ip = IpAddr::V4(Ipv4Addr::new(123, 123, 123, 123));

        let response = Hcaptcha::new("", "")
            .set_user_ip(&user_ip)
            .verify()
            .compat()
            .await;

        match response {
            Ok(()) => panic!("unexpected response: Ok(())"),
            Err(Codes(ref errors)) => {
                assert!(errors.contains(&MissingSecret));
                assert!(errors.contains(&MissingResponse));
            }
            Err(e) => panic!("unexpected error: {}", e),
        };

        println!("{:?}", response);
    }

    #[tokio::test]
    async fn test_invalid_secret_missing_response_with_site_key() {
        let response = Hcaptcha::new("", "")
            .set_site_key("10000000-ffff-ffff-ffff-000000000001")
            .verify()
            .compat()
            .await;

        match response {
            Ok(()) => panic!("unexpected response: Ok(())"),
            Err(Codes(ref errors)) => {
                assert!(errors.contains(&MissingSecret));
                assert!(errors.contains(&MissingResponse));
            }
            Err(e) => panic!("unexpected error: {}", e),
        };

        println!("{:?}", response);
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
        match response.error_codes() {
            Some(hash_set) => assert_eq!(hash_set.len(), 2),
            None => {}
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
        match response.score_reason() {
            Some(hash_set) => assert!(hash_set.is_empty()),
            None => {}
        }
    }
}
