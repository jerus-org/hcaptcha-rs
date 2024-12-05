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
//!     use hcaptcha::Client;
//!     let client = Client::new();
//! ```
//!
//! Create a client and submit for verification.
//!```no_run
//!     use hcaptcha::{Captcha, Client, Request};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), hcaptcha::Error> {
//! #   let secret = get_your_secret();
//! #   let captcha = dummy_captcha();
//! #   let request = Request::new(&secret, captcha)?; // <- returns error
//!     let client = Client::new();
//!     let response = client.verify(request).await?;
//! # Ok(())
//! # }
//!
//! # fn get_your_secret() -> String {
//! #   "0x123456789abcde0f123456789abcdef012345678".to_string()
//! # }
//! # use rand::distributions::Alphanumeric;
//! # use rand::{thread_rng, Rng};
//! # use std::iter;
//! # fn random_response() -> String {
//! #    let mut rng = thread_rng();
//! #    iter::repeat(())
//! #        .map(|()| rng.sample(Alphanumeric))
//! #        .map(char::from)
//! #        .take(100)
//! #        .collect()
//! # }
//! # fn dummy_captcha() -> Captcha {
//! #    Captcha::new(&random_response())
//! #       .unwrap()
//! #       .set_remoteip(&mockd::internet::ipv4_address())
//! #       .unwrap()
//! #       .set_sitekey(&mockd::unique::uuid_v4())
//! #       .unwrap()
//! #       }
//!
//! ```

// const CYAN: &str = "\u{001b}[35m";
// const RESET: &str = "\u{001b}[0m";

use crate::Error;
use crate::Request;
use crate::Response;
use reqwest::Url;
// #[cfg(target_arch = "wasm32")]
// use tokio::runtime;

mod form;

use form::Form;

/// Endpoint url for the Hcaptcha siteverify API.
pub const VERIFY_URL: &str = "https://hcaptcha.com/siteverify";

/// Client to submit a request to a Hcaptcha validation endpoint.
#[cfg_attr(docsrs, allow(rustdoc::missing_doc_code_examples))]
#[derive(Debug)]
pub struct Client {
    /// HTTP Client to submit request to endpoint and read the response.
    client: reqwest::Client,
    /// Url for the endpoint.
    url: Url,
}

#[cfg_attr(docsrs, allow(rustdoc::missing_doc_code_examples))]
impl Default for Client {
    fn default() -> Client {
        Client::new()
    }
}

#[cfg_attr(docsrs, allow(rustdoc::missing_doc_code_examples))]
impl Client {
    /// Create a new Hcaptcha Client to connect with the default Hcaptcha
    /// siteverify API endpoint specified in [VERIFY_URL].
    ///
    /// # Example
    /// Initialise client to connect to default API endpoint.
    /// ```
    ///     use hcaptcha::Client;
    ///     let client = Client::new();
    /// ```
    /// # Panic
    ///
    /// If the default API url constant is corrupted the function with
    /// will panic.
    #[allow(unknown_lints)]
    #[cfg_attr(docsrs, allow(rustdoc::bare_urls))]
    pub fn new() -> Client {
        Client {
            client: reqwest::Client::new(),
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
    ///     use hcaptcha::Client;
    ///     use url::Url;
    ///
    ///     let url = "https://domain.com/siteverify";
    ///     let _client = Client::new_with(url);
    /// ```
    pub fn new_with(url: &str) -> Result<Client, url::ParseError> {
        Ok(Client {
            client: reqwest::Client::new(),
            url: Url::parse(url)?,
        })
    }

    /// Set the url.
    ///
    /// Specify the url for the hcaptcha API. This method is useful
    /// during testing to provide a mock url.
    ///
    /// # Example
    /// Initialise client to connect to custom Hcaptcha API
    /// ```no_run
    /// # fn main() -> Result<(), hcaptcha::Error> {
    ///     use hcaptcha::Client;
    ///     use url::Url;
    ///
    ///     let url = "https://domain.com/siteverify";
    ///     let client = Client::new()
    ///                        .set_url(url)?;
    /// #    Ok(())
    /// # }
    /// ```
    pub fn set_url(mut self, url: &str) -> Result<Self, Error> {
        self.url = Url::parse(url)?;
        Ok(self)
    }

    /// Verify the client token with the Hcaptcha service API.
    ///
    /// Call the Hcaptcha api and provide a [Request] struct.
    ///
    /// # Inputs
    ///
    /// Request contains the required and optional fields
    /// for the Hcaptcha API. The required fields include the response
    /// code to validate and the secret key.
    ///
    /// # Outputs
    ///
    /// This method returns [Response] if successful and [Error] if
    /// unsuccessful.
    ///
    /// # Example
    ///
    ///
    ///  ```no_run
    ///     use hcaptcha::{Client, Request};
    /// # use hcaptcha::Captcha;
    /// # use rand::distributions::Alphanumeric;
    /// # use rand::{thread_rng, Rng};
    /// # use std::iter;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::Error> {
    ///     let secret = get_your_secret(); // your secret key
    ///     let captcha = get_captcha();  // user's token
    ///
    ///     let request = Request::new(&secret, captcha)?;
    ///
    ///     let client = Client::new();
    ///
    ///     let response = client.verify_client_response(request).await?;
    ///
    /// # #[cfg(feature = "enterprise")]
    ///     let score = response.score();
    /// # #[cfg(feature = "enterprise")]
    ///     let score_reasons = response.score_reason();
    ///
    /// # Ok(())
    /// # }
    /// # fn get_your_secret() -> String {
    /// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    /// # }
    /// # fn random_response() -> String {
    /// #    let mut rng = thread_rng();
    /// #    iter::repeat(())
    /// #        .map(|()| rng.sample(Alphanumeric))
    /// #        .map(char::from)
    /// #        .take(100)
    /// #        .collect()
    /// # }
    /// # fn get_captcha() -> Captcha {
    /// #    Captcha::new(&random_response())
    /// #       .unwrap()
    /// #       .set_remoteip(&mockd::internet::ipv4_address())
    /// #       .unwrap()
    /// #       .set_sitekey(&mockd::unique::uuid_v4())
    /// #       .unwrap()
    /// #       }
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
    #[deprecated(since = "3.0.0", note = "please use `verify` instead")]
    pub async fn verify_client_response(self, request: Request) -> Result<Response, Error> {
        let form: Form = request.into();
        #[cfg(feature = "trace")]
        tracing::debug!(
            "The form to submit to Hcaptcha API: {:?}",
            serde_urlencoded::to_string(&form).unwrap_or_else(|_| "form corrupted".to_owned())
        );
        let response = self
            .client
            .post(self.url.clone())
            .form(&form)
            .send()
            .await?
            .json::<Response>()
            .await?;
        #[cfg(feature = "trace")]
        tracing::debug!("The response is: {:?}", response);
        response.check_error()?;
        Ok(response)
    }

    /// Verify the client token with the Hcaptcha service API.
    ///
    /// Call the Hcaptcha api and provide a [Request] struct.
    ///
    /// # Inputs
    ///
    /// Request contains the required and optional fields
    /// for the Hcaptcha API. The required fields include the response
    /// code to validate and the secret key.
    ///
    /// # Outputs
    ///
    /// This method returns [Response] if successful and [Error] if
    /// unsuccessful.
    ///
    /// # Example
    ///
    ///
    ///  ```no_run
    ///     use hcaptcha::{Client, Request};
    /// # use hcaptcha::Captcha;
    /// # use rand::distributions::Alphanumeric;
    /// # use rand::{thread_rng, Rng};
    /// # use std::iter;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::Error> {
    ///     let secret = get_your_secret(); // your secret key
    ///     let captcha = get_captcha();  // user's token
    ///
    ///     let request = Request::new(&secret, captcha)?;
    ///
    ///     let client = Client::new();
    ///
    ///     let response = client.verify(request).await?;
    ///
    /// # #[cfg(feature = "enterprise")]
    ///     let score = response.score();
    /// # #[cfg(feature = "enterprise")]
    ///     let score_reasons = response.score_reason();
    ///
    /// # Ok(())
    /// # }
    /// # fn get_your_secret() -> String {
    /// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    /// # }
    /// # fn random_response() -> String {
    /// #    let mut rng = thread_rng();
    /// #    iter::repeat(())
    /// #        .map(|()| rng.sample(Alphanumeric))
    /// #        .map(char::from)
    /// #        .take(100)
    /// #        .collect()
    /// # }
    /// # fn get_captcha() -> Captcha {
    /// #    Captcha::new(&random_response())
    /// #       .unwrap()
    /// #       .set_remoteip(&mockd::internet::ipv4_address())
    /// #       .unwrap()
    /// #       .set_sitekey(&mockd::unique::uuid_v4())
    /// #       .unwrap()
    /// #       }
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
    pub async fn verify(self, request: Request) -> Result<Response, Error> {
        let form: Form = request.into();
        #[cfg(feature = "trace")]
        tracing::debug!(
            "The form to submit to Hcaptcha API: {:?}",
            serde_urlencoded::to_string(&form).unwrap_or_else(|_| "form corrupted".to_owned())
        );
        let response = self
            .client
            .post(self.url.clone())
            .form(&form)
            .send()
            .await?
            .json::<Response>()
            .await?;
        #[cfg(feature = "trace")]
        tracing::debug!("The response is: {:?}", response);
        response.check_error()?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Code, Error};
    use chrono::{TimeDelta, Utc};
    use claims::{assert_err, assert_ok};
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use serde_json::json;
    use std::iter;
    #[cfg(feature = "trace")]
    use tracing_test::traced_test;
    use wiremock::matchers::{body_string, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // const RED: &str = "\t\u{001b}[31m";
    // const GREEN: &str = "\t\u{001b}[32m";
    // const CYAN: &str = "\t\u{001b}[36m";
    // const RESET: &str = "\t\u{001b}[0m";

    fn random_string(characters: usize) -> String {
        let mut rng = thread_rng();
        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(characters)
            .collect()
    }

    #[tokio::test]
    #[cfg_attr(feature = "trace", traced_test)]
    async fn hcaptcha_mock_verify() {
        let token = random_string(100);
        let secret = format!("0x{}", hex::encode(random_string(20)));
        let request = Request::new_from_response(&secret, &token).unwrap();

        let expected_body = format!("response={}&secret={}", &token, &secret);

        let timestamp = Utc::now()
            .checked_sub_signed(TimeDelta::try_minutes(10).unwrap())
            .unwrap()
            .to_rfc3339();

        let response_template = ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "challenge_ts": timestamp,
            "hostname": "test-host",
        }));
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/siteverify"))
            .and(body_string(&expected_body))
            .respond_with(response_template)
            .mount(&mock_server)
            .await;
        let uri = format!("{}{}", mock_server.uri(), "/siteverify");

        let client = Client::new_with(&uri).unwrap();
        let response = client.verify(request).await;
        assert_ok!(&response);
        let response = response.unwrap();
        assert!(&response.success());
        assert_eq!(&response.timestamp().unwrap(), &timestamp);
        #[cfg(feature = "trace")]
        assert!(logs_contain("Hcaptcha API"));
        #[cfg(feature = "trace")]
        assert!(logs_contain("The response is"));
    }

    #[tokio::test]
    #[cfg_attr(feature = "trace", traced_test)]
    async fn hcaptcha_mock_verify_not_found() {
        let token = random_string(100);
        let secret = format!("0x{}", hex::encode(random_string(20)));
        let request = Request::new_from_response(&secret, &token).unwrap();

        let expected_body = format!("response={}&secret={}", &token, &secret);

        let response_template = ResponseTemplate::new(404);
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/siteverify"))
            .and(body_string(&expected_body))
            .respond_with(response_template)
            .mount(&mock_server)
            .await;
        let uri = format!("{}{}", mock_server.uri(), "/siteverify");

        let client = Client::new_with(&uri).unwrap();
        let response = client.verify(request).await;
        assert_err!(&response);
    }

    #[tokio::test]
    #[cfg_attr(feature = "trace", traced_test)]
    async fn hcaptcha_mock_verify_client_response() {
        let token = random_string(100);
        let secret = format!("0x{}", hex::encode(random_string(20)));
        let request = Request::new_from_response(&secret, &token).unwrap();

        let expected_body = format!("response={}&secret={}", &token, &secret);

        let timestamp = Utc::now()
            .checked_sub_signed(TimeDelta::try_minutes(10).unwrap())
            .unwrap()
            .to_rfc3339();

        let response_template = ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "challenge_ts": timestamp,
            "hostname": "test-host",
        }));
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/siteverify"))
            .and(body_string(&expected_body))
            .respond_with(response_template)
            .mount(&mock_server)
            .await;
        let uri = format!("{}{}", mock_server.uri(), "/siteverify");

        let client = Client::new_with(&uri).unwrap();
        #[allow(deprecated)]
        let response = client.verify_client_response(request).await;
        assert_ok!(&response);
        let response = response.unwrap();
        assert!(&response.success());
        assert_eq!(&response.timestamp().unwrap(), &timestamp);
        #[cfg(feature = "trace")]
        assert!(logs_contain("Hcaptcha API"));
        #[cfg(feature = "trace")]
        assert!(logs_contain("The response is"));
    }

    #[tokio::test]
    #[cfg_attr(feature = "trace", traced_test)]
    async fn hcaptcha_mock_verify_client_response_not_found() {
        let token = random_string(100);
        let secret = format!("0x{}", hex::encode(random_string(20)));
        let request = Request::new_from_response(&secret, &token).unwrap();

        let expected_body = format!("response={}&secret={}", &token, &secret);

        let response_template = ResponseTemplate::new(404);
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/siteverify"))
            .and(body_string(&expected_body))
            .respond_with(response_template)
            .mount(&mock_server)
            .await;
        let uri = format!("{}{}", mock_server.uri(), "/siteverify");

        let client = Client::new_with(&uri).unwrap();
        #[allow(deprecated)]
        let response = client.verify_client_response(request).await;
        assert_err!(&response);
    }

    #[tokio::test]
    #[cfg_attr(feature = "trace", traced_test)]
    async fn hcaptcha_mock_with_remoteip() {
        let token = random_string(100);
        let secret = format!("0x{}", hex::encode(random_string(20)));
        let remoteip = mockd::internet::ipv4_address();
        let request = Request::new_from_response(&secret, &token)
            .unwrap()
            .set_remoteip(&remoteip)
            .unwrap();

        let expected_body = format!(
            "response={}&remoteip={}&secret={}",
            &token, &remoteip, &secret
        );

        let timestamp = Utc::now()
            .checked_sub_signed(TimeDelta::try_minutes(10).unwrap())
            .unwrap()
            .to_rfc3339();

        let response_template = ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "challenge_ts": timestamp,
            "hostname": "test-host",
        }));
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/siteverify"))
            .and(body_string(&expected_body))
            .respond_with(response_template)
            .mount(&mock_server)
            .await;
        let uri = format!("{}{}", mock_server.uri(), "/siteverify");

        let client = Client::new_with(&uri).unwrap();
        let response = client.verify(request).await;
        assert_ok!(&response);
        let response = response.unwrap();
        assert!(&response.success());
        assert_eq!(&response.timestamp().unwrap(), &timestamp);
        #[cfg(feature = "trace")]
        assert!(logs_contain("Hcaptcha API"));
        #[cfg(feature = "trace")]
        assert!(logs_contain("The response is"));
    }

    #[tokio::test]
    #[cfg_attr(feature = "trace", traced_test)]
    async fn hcaptcha_mock_with_sitekey() {
        let token = random_string(100);
        let secret = format!("0x{}", hex::encode(random_string(20)));
        let sitekey = mockd::unique::uuid_v4();
        let request = Request::new_from_response(&secret, &token)
            .unwrap()
            .set_sitekey(&sitekey)
            .unwrap();

        let expected_body = format!(
            "response={}&sitekey={}&secret={}",
            &token, &sitekey, &secret
        );

        let timestamp = Utc::now()
            .checked_sub_signed(TimeDelta::try_minutes(10).unwrap())
            .unwrap()
            .to_rfc3339();

        let response_template = ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "challenge_ts": timestamp,
            "hostname": "test-host",
        }));
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/siteverify"))
            .and(body_string(&expected_body))
            .respond_with(response_template)
            .mount(&mock_server)
            .await;
        let uri = format!("{}{}", mock_server.uri(), "/siteverify");

        let client = Client::new_with(&uri).unwrap();
        let response = client.verify(request).await;
        assert_ok!(&response);
        let response = response.unwrap();
        assert!(&response.success());
        assert_eq!(&response.timestamp().unwrap(), &timestamp);
        #[cfg(feature = "trace")]
        assert!(logs_contain("Hcaptcha API"));
        #[cfg(feature = "trace")]
        assert!(logs_contain("The response is"));
    }

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
        let response: Response = serde_json::from_value(api_response).unwrap();
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
        let response: Response = serde_json::from_value(api_response).unwrap();
        assert!(!response.success());
        assert!(response.error_codes().is_some());
        if let Some(hash_set) = response.error_codes() {
            assert_eq!(hash_set.len(), 2);
            assert!(hash_set.contains(&Code::MissingSecret));
            assert!(hash_set.contains(&Code::Unknown("foo".to_owned())));
        }
    }

    #[test]
    fn test_hcaptcha_client_default_initialization() {
        let client = Client::default();
        assert!(matches!(client, Client { .. }));
    }

    #[test]
    fn test_hcaptcha_client_default_calls_new() {
        // Assuming Client::new() has some side effect or state change
        // that can be checked to ensure it was called.
        let client = Client::default();
        // Here we would check the side effect or state change
        // For example, if new() sets a specific field, we would assert that field's value
        let expected_value = Url::parse(VERIFY_URL).unwrap();
        assert!(client.url == expected_value);
    }

    #[test]
    fn test_set_url_with_valid_url() {
        let client = Client::default();
        let result = client.set_url("https://example.com");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().url.as_str(), "https://example.com/");
    }

    #[test]
    fn test_set_url_with_invalid_url() {
        let client = Client::default();
        let result = client.set_url("invalid-url");
        assert!(result.is_err());
        match result {
            Err(Error::Url(_)) => (),
            _ => panic!("Expected UrlParseError"),
        }
    }

    #[test]
    fn test_set_url_with_empty_string() {
        let client = Client::default();
        let result = client.set_url("");
        assert!(result.is_err());
        match result {
            Err(Error::Url(_)) => (),
            _ => panic!("Expected UrlParseError"),
        }
    }
}
