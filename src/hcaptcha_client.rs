use crate::HcaptchaError;
use crate::HcaptchaRequest;
use crate::HcaptchaResponse;
use reqwest::{Client, Url};

const VERIFY_URL: &str = "https://hcaptcha.com/siteverify";

/// Client to submit a request to a Hcaptcha validation endpoint.
#[derive(Debug)]
pub struct HcaptchaClient {
    /// HTTP Client to submit request to endpoint and read the response.
    client: Client,
    /// Url for the endpoint.
    url: Url,
}

impl Default for HcaptchaClient {
    fn default() -> HcaptchaClient {
        HcaptchaClient::new()
    }
}

impl HcaptchaClient {
    /// Create a new Hcaptcha Client.
    ///
    /// New implements a client to connect to the Hcaptcha siteverify
    /// API (https://hcaptcha.com/siteverify)
    ///
    /// # Example
    /// Initialise client to connect to default Hcaptcha API
    /// ```
    /// use hcaptcha::HcaptchaClient;
    /// # fn main() {
    ///
    ///     let client = HcaptchaClient::new();
    ///
    /// # }
    /// ```
    /// # Panic
    ///
    /// If the default API url constant is corrupted the function with
    /// will panic.
    #[allow(unknown_lints)]
    #[allow(non_autolinks)]
    pub fn new() -> HcaptchaClient {
        HcaptchaClient {
            client: Client::new(),
            url: Url::parse(VERIFY_URL).expect("API url string corrupt"),
        }
    }

    /// Create a new Hcaptcha Client and specify the url for the API.
    ///
    /// Specify the url for the hcaptcha API.
    ///
    /// # Example
    /// Initialise client to connect to custom Hcaptcha API
    /// ```
    /// use hcaptcha::HcaptchaClient;
    /// use url::Url;
    /// # fn main() {
    ///
    ///     if let Ok(url) = Url::parse("https://domain.com/siteverify") {
    ///         let client = HcaptchaClient::new_with(url);
    ///     };
    /// # }
    /// ```
    pub fn new_with(url: Url) -> HcaptchaClient {
        HcaptchaClient {
            client: Client::new(),
            url,
        }
    }

    /// Verify the client token with the Hcaptcha API
    ///
    /// Call the Hcaptcha api providing a HcaptchaRequest structure.
    ///
    /// # Inputs
    ///
    /// HcaptchaRequest contains the required and optional fields
    /// for the Hcaptcha API. The required fields include the response
    /// code to validate and the secret key.
    ///
    /// # Outputs
    ///
    /// This method returns HcaptchaResponse if successful and HcaptchaError if
    /// unsuccessful.
    ///
    /// Example
    ///
    ///  ```should_panic
    /// use hcaptcha::{Hcaptcha, HcaptchaClient, HcaptchaRequest};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    /// let secret = ""; // your secret key
    /// let token = "";  // user's token
    ///
    /// let request = HcaptchaRequest::new(secret, token)?; // <- returns error
    ///
    /// let client = HcaptchaClient::new();
    ///
    /// let response = client.verify_client_response(request).await?;
    ///
    /// let score = response.score();
    /// let score_reasons = response.score_reason();
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Logging
    ///
    /// If the tracing features is enabled a debug level span is set for the
    /// method and an event logs the response.
    ///
    #[allow(dead_code)]
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(
            name = "Request verification from hcaptcha.",
            skip(self),
            level = "debug"
        )
    )]
    pub async fn verify_client_response(
        &self,
        client_response: HcaptchaRequest,
    ) -> Result<HcaptchaResponse, HcaptchaError> {
        let response = self
            .client
            .post(self.url.clone())
            .form(&client_response)
            .send()
            .await?;
        let response = response.json::<HcaptchaResponse>().await?;
        #[cfg(feature = "trace")]
        tracing::debug!("The response is: {:?}", response);
        response.check_error()?;
        Ok(response)
    }
}

// impl HcaptchaClient {
//     /// Create a new Hcaptcha Request
//     ///
//     /// # Example 1
//     /// In the following example the new method returns an error as the secret
//     /// and response inputs are blank.
//     /// The new method validates these without calling the hcaptcha API.
//     /// ```should_panic
//     /// use hcaptcha::Hcaptcha;
//     /// # use std::error::Error;
//     ///
//     /// # #[tokio::main]
//     /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
//     /// let secret = ""; // your secret key
//     /// let token = "";  // user's token
//     ///
//     /// Hcaptcha::new(secret, token)? // <- returns error
//     ///     .verify()
//     ///     .await
//     /// # }
//     /// ```
//     /// # Example 2
//     /// In the following example the call to new returns an error as the
//     /// response input is blank.
//     /// The new method validates these without calling the hcaptcha API.
//     /// ```should_panic
//     /// use hcaptcha::Hcaptcha;
//     /// # use std::error::Error;
//     ///
//     /// # #[tokio::main]
//     /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
//     /// let secret = ""; // your secret key
//     /// let token = "";  // user's token
//     ///
//     /// Hcaptcha::new(secret, token)? // <- returns error
//     ///     .verify()
//     ///     .await
//     /// # }
//     /// ```
//     /// # Example 3
//     /// This example likely fails as the token is specified is unlikely to be
//     /// a valid token. The new function will however pass the input as it
//     /// only validates that the token is not blank or empty.
//     /// ```should_panic no_run
//     /// use hcaptcha::Hcaptcha;
//     ///
//     /// # #[tokio::main]
//     /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
//     /// let secret = "0x0000000000000000000000000000000000000000"; // your secret key
//     /// let token = "this_is_likely_invalid";  // user's token
//     ///
//     /// Hcaptcha::new(secret, token)?
//     ///     .verify()
//     ///     .await  // <- likely returns InvalidResponse error
//     /// # }
//     /// ```
//     #[cfg_attr(
//         feature = "trace",
//         tracing::instrument(
//             name = "Construct new request with secret and client response.",
//             skip(secret)
//         )
//     )]
//     #[allow(dead_code)]
//     pub fn new(secret: &str, response: &str) -> Result<Hcaptcha, HcaptchaError> {
//         Ok(Hcaptcha {
//             request: HcaptchaRequest::new(secret, response)?,
//             ..Hcaptcha::default()
//         })
//     }

//     /// Specify the optional ip address value
//     ///
//     /// # Example
//     /// This example will most likely fail as the client response token will be
//     /// reported by the hcaptcah API as invalid.
//     /// The sample serves to illustrate the construction of a data structure
//     /// that includes the user ip address using the set_user_ip method.
//     /// ```should_panic no_run
//     /// use hcaptcha::Hcaptcha;
//     /// use std::net::{IpAddr, Ipv4Addr};
//     /// # #[tokio::main]
//     /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
//     /// let secret = "0x0000000000000000000000000000000000000000"; // your secret key
//     /// let token = "this_is_likely_invalid";  // user's token
//     /// let user_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 17));
//     ///
//     /// Hcaptcha::new(secret, token)?
//     ///     .set_user_ip(&user_ip)
//     ///     .verify()
//     ///     .await // <- likely returns InvalidResponse error
//     /// # }
//     /// ```
//     #[cfg_attr(
//         feature = "trace",
//         tracing::instrument(name = "Add client IP to request.", skip(self, user_ip))
//     )]
//     #[allow(dead_code)]
//     pub fn set_user_ip(mut self, user_ip: &IpAddr) -> Hcaptcha {
//         self.request.set_user_ip(user_ip);
//         self
//     }
//     /// Specify the optional site key value
//     ///
//     /// # Example
//     /// This example will most likely fail as the client response token will be
//     /// reported by the hcaptcah API as invalid.
//     /// The sample serves to illustrate the construction of a data structure
//     /// that includes the site_key using the set_site_key method.
//     /// ```should_panic no_run
//     /// use hcaptcha::Hcaptcha;
//     /// use uuid::Uuid;
//     /// # use hcaptcha::HcaptchaError;
//     /// # #[tokio::main]
//     /// # async fn main() -> Result<(), HcaptchaError> {
//     /// let secret = "0x0000000000000000000000000000000000000000"; // your secret key
//     /// let token = "this_is_likely_invalid";  // user's token
//     /// let site_key = Uuid::parse_str("10000000-ffff-ffff-ffff-000000000001")?;
//     ///
//     /// Hcaptcha::new(secret, token)?
//     ///     .set_site_key(&site_key)
//     ///     .verify()
//     ///     .await // <- likely returns InvalidResponse error
//     /// # }
//     /// ```
//     #[cfg_attr(
//         feature = "trace",
//         tracing::instrument(name = "Add site key to request.", skip(self, site_key))
//     )]
//     #[allow(dead_code)]
//     pub fn set_site_key(mut self, site_key: &Uuid) -> Hcaptcha {
//         self.request.set_site_key(site_key);
//         self
//     }
//     /// Verify a hcaptcha user response
//     ///
//     /// # Example
//     /// The example illustrates a call to verify the full data structure, including
//     /// the optional user_ip and site_key fields. The example will yield as error
//     /// as the token value will not be valid.
//     /// ```should_panic no_run
//     /// # use hcaptcha::Hcaptcha;
//     /// # use hcaptcha::Code::*;
//     /// # use std::net::{IpAddr, Ipv4Addr};
//     /// # use uuid::Uuid;
//     ///
//     /// # #[tokio::main]
//     /// # async fn main() -> Result<(), hcaptcha::HcaptchaError>{
//     /// let secret = "0x0000000000000000000000000000000000000000";
//     /// let token = "this_is_likely_invalid";  // user's token
//     /// let user_ip = IpAddr::V4(Ipv4Addr::new(123, 123, 123, 123));
//     /// let site_key = Uuid::parse_str("10000000-ffff-ffff-ffff-000000000001")?;
//     ///
//     /// let response = Hcaptcha::new(secret, token)?
//     ///                 .set_user_ip(&user_ip)
//     ///                 .set_site_key(&site_key)
//     ///                 .verify()
//     ///                 .await;
//     ///
//     /// assert!(response.is_err());
//     /// Ok(())
//     /// # }
//     /// ```
//     #[cfg_attr(
//         feature = "trace",
//         tracing::instrument(
//             name = "Submit request to Hcaptcha for verification.",
//             skip(self),
//             fields(response = %self.response),
//         )
//     )]
//     pub async fn verify(&mut self) -> Result<(), HcaptchaError> {
//         self.response = self.request.verify().await?;
//         match (self.response.success(), self.response.error_codes()) {
//             (true, _) => {
//                 #[cfg(feature = "trace")]
//                 tracing::debug!("Validated successfully");
//                 Ok(())
//             }
//             (false, Some(errors)) => {
//                 #[cfg(feature = "trace")]
//                 tracing::debug!("Validation failed with known errors: {:?}", errors);
//                 Err(HcaptchaError::Codes(errors))
//             }
//             (false, _) => {
//                 #[cfg(feature = "trace")]
//                 tracing::debug!("Validation failed with and unkown error.");
//                 Err(HcaptchaError::Codes(HashSet::new()))
//             }
//         }
//     }

//     /// Get the hostname returned in the response
//     /// Option string containig the hostname of the site
//     /// where the captcha was solved
//     ///
//     #[cfg_attr(
//         feature = "trace",
//         tracing::instrument(name = "Return hostname from the Hcaptcha API response.", skip(self))
//     )]
//     #[allow(dead_code)]
//     pub fn hostname(&self) -> Option<String> {
//         self.response.hostname()
//     }

//     /// Get the timestamp from the response
//     /// Option string containing the timestamp of the captcha
//     /// (ISO format yyyy-MM-dd'T'HH:mm:ssZZ)
//     ///
//     #[cfg_attr(
//         feature = "trace",
//         tracing::instrument(
//             name = "Return timestampt from the Hcaptcha API response.",
//             skip(self)
//         )
//     )]
//     #[allow(dead_code)]
//     pub fn timestamp(&self) -> Option<String> {
//         self.response.timestamp()
//     }

//     /// Get the credit flag
//     /// Optional flag showing whether the response will be credited
//     ///
//     #[cfg_attr(
//         feature = "trace",
//         tracing::instrument(name = "Return credit from the Hcaptcha API response.", skip(self))
//     )]
//     #[allow(dead_code)]
//     pub fn credit(&self) -> Option<bool> {
//         self.response.credit()
//     }

//     /// Get the score
//     ///
//     /// Provides the score from botstop for the malicious activity.
//     ///
//     #[cfg_attr(
//         feature = "trace",
//         tracing::instrument(name = "Return score from the Hcaptcha API response.", skip(self))
//     )]
//     #[cfg(feature = "enterprise")]
//     #[cfg_attr(docsrs, doc(cfg(feature = "enterprise")))]
//     #[allow(dead_code)]
//     pub fn score(&self) -> Option<f32> {
//         self.response.score()
//     }

//     /// Get the reasons for the score
//     ///
//     /// Provide the reason(s) for the score.
//     /// See [BotStop.com](https://BotStop.com) for details.
//     ///
//     #[cfg_attr(
//         feature = "trace",
//         tracing::instrument(
//             name = "Return score reason from the Hcaptcha API response.",
//             skip(self)
//         )
//     )]
//     #[cfg(feature = "enterprise")]
//     #[cfg_attr(docsrs, doc(cfg(feature = "enterprise")))]
//     #[allow(dead_code)]
//     pub fn score_reason(&self) -> Option<HashSet<String>> {
//         self.response.score_reason()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
