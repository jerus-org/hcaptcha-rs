//!
//! # Hcaptcha Client
//!
//! The Hcaptcha Client struct provides an http client to connect to
//! the Hcaptcha API.
//!
//! The url for the API is stored in the url field of the struct.
//! A default url is stored in the const VERIFY_URL.
//! The new_with method allows the specification of an alternative url.
//!
//! # Examples
//! Create client to connect to default API endpoint.
//! ```
//!     use hcaptcha::HcaptchaClient;
//!     let client = HcaptchaClient::new();
//! ```
//!
//! Create a client and submit for verification.
//!```no_run
//!     use hcaptcha::{HcaptchaClient, HcaptchaRequest};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
//! #    let secret = ""; // your secret key
//! #    let token = "";  // user's token
//! #   let request = HcaptchaRequest::new(secret, token)?; // <- returns error
//!     let client = HcaptchaClient::new();
//!     let response = client.verify_client_response(request).await?;
//! # Ok(())
//! # }
//! ```

use crate::HcaptchaError;
use crate::HcaptchaRequest;
use crate::HcaptchaResponse;
use reqwest::{Client, Url};

/// Endpoint url for the Hcaptcha siteverify API.
pub const VERIFY_URL: &str = "https://hcaptcha.com/siteverify";

/// Client to submit a request to a Hcaptcha validation endpoint.
#[allow(missing_doc_code_examples)]
#[derive(Debug)]
pub struct HcaptchaClient {
    /// HTTP Client to submit request to endpoint and read the response.
    client: Client,
    /// Url for the endpoint.
    url: Url,
}

#[allow(missing_doc_code_examples)]
impl Default for HcaptchaClient {
    fn default() -> HcaptchaClient {
        HcaptchaClient::new()
    }
}

#[allow(missing_doc_code_examples)]
impl HcaptchaClient {
    /// Create a new Hcaptcha Client to connect with the default Hcaptcha
    /// siteverify API endpoint specified in [VERIFY_URL].
    ///
    /// # Example
    /// Initialise client to connect to default API endpoint.
    /// ```
    ///     use hcaptcha::HcaptchaClient;
    ///     let client = HcaptchaClient::new();
    /// ```
    /// # Panic
    ///
    /// If the default API url constant is corrupted the function with
    /// will panic.
    #[allow(unknown_lints)]
    #[allow(bare_urls)]
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
    ///     use hcaptcha::HcaptchaClient;
    ///     use url::Url;
    ///
    ///     if let Ok(url) = Url::parse("https://domain.com/siteverify") {
    ///         let client = HcaptchaClient::new_with(url);
    ///     };
    /// ```
    pub fn new_with(url: Url) -> HcaptchaClient {
        HcaptchaClient {
            client: Client::new(),
            url,
        }
    }

    /// Verify the client token with the Hcaptcha API.
    ///
    /// Call the Hcaptcha api providing a [HcaptchaRequest] struct.
    ///
    /// # Inputs
    ///
    /// HcaptchaRequest contains the required and optional fields
    /// for the Hcaptcha API. The required fields include the response
    /// code to validate and the secret key.
    ///
    /// # Outputs
    ///
    /// This method returns [HcaptchaResponse] if successful and [HcaptchaError] if
    /// unsuccessful.
    ///
    /// # Example
    ///
    ///
    ///  ```no_run
    ///     use hcaptcha::{HcaptchaClient, HcaptchaRequest};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    ///     let secret = get_your_secret(); // your secret key
    ///     let bad_token = get_user_token();  // user's token
    ///
    ///     let request = HcaptchaRequest::new(&secret, &bad_token)?; // <- returns error
    ///
    ///     let client = HcaptchaClient::new();
    ///
    ///     let response = client.verify_client_response(request).await?;
    ///
    ///     let score = response.score();
    ///     let score_reasons = response.score_reason();
    ///
    /// # Ok(())
    /// # }
    /// # fn get_your_secret() -> String {
    /// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    /// # }
    /// # fn get_user_token() -> String {
    /// #    "thisisnotapropertoken".to_string()
    /// # }
    /// ```
    ///
    /// # Logging
    ///
    /// If the `trace` feature is enabled a debug level span is set for the
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Code;
    use serde_json::json;

    #[test]
    fn test_success_response() {
        let api_response = json!({
            "success": true,
            "challenge_ts": "2020-11-11T23:27:00Z",
            "hostname": "my-host.ie",
            "credit": true,
            "error-codes": [],
            "score": null,
            "score_reason": [],
        });
        let response: HcaptchaResponse = serde_json::from_value(api_response).unwrap();
        assert!(response.success());
        assert_eq!(
            response.timestamp(),
            Some("2020-11-11T23:27:00Z".to_owned())
        );
        assert_eq!(response.hostname(), Some("my-host.ie".to_owned()));
    }
    #[test]
    fn test_error_response() {
        let api_response = json!({
            "success": false,
            "challenge_ts": null,
            "hostname": null,
            "credit": null,
            "error-codes": ["missing-input-secret", "foo"],
            "score": null,
            "score_reason": [],
        });
        let response: HcaptchaResponse = serde_json::from_value(api_response).unwrap();
        assert!(!response.success());
        assert!(response.error_codes().is_some());
        if let Some(hash_set) = response.error_codes() {
            assert_eq!(hash_set.len(), 2);
            assert!(hash_set.contains(&Code::MissingSecret));
            assert!(hash_set.contains(&Code::Unknown("foo".to_owned())));
        }
    }
}
