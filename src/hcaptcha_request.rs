//! Provides a struct to collect the required and optional parameters for
//! the hcaptcha api request.
//!
//! # Example
//!
//! ```
//!     use hcaptcha::HcaptchaRequest;
//! # #[tokio::main]
//! # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
//!     let secret = get_your_secret();         // your secret key
//!     let bad_token = get_your_token();       // user's response token
//!     let site_key = get_your_site_key();     // your site key
//!     let user_ip = get_user_ip_address();    // user's ip address
//!
//!     let request = HcaptchaRequest::new(&secret, &bad_token)? // <- returns error
//!         .set_site_key(site_key)
//!         .set_user_ip(user_ip);
//! # Ok(())
//! # }
//! # fn get_your_secret() -> String {
//! #   "0x123456789abcde0f123456789abcdef012345678".to_string()
//! # }
//! # fn get_your_token() -> String {
//! #    "thisisnotapropertoken".to_string()
//! # }
//! # use std::net::{IpAddr, Ipv4Addr};
//! # fn get_user_ip_address() -> IpAddr {
//! #    IpAddr::V4(Ipv4Addr::new(192, 168, 0, 17))
//! # }
//! # use uuid::Uuid;
//! # fn get_your_site_key() -> Uuid {
//! #    Uuid::new_v4()
//! # }
//!
//! ```

use crate::HcaptchaError;
use std::net::IpAddr;
use uuid::Uuid;

mod hcaptcha_client_response;
mod hcaptcha_secret;

use hcaptcha_client_response::HcaptchaClientResponse;
use hcaptcha_secret::HcaptchaSecret;

/// Type to capture the required and optional data for a call to the hcaptcha API
#[allow(missing_doc_code_examples)]
#[derive(Debug, Default, serde::Serialize)]
pub struct HcaptchaRequest {
    /// The response from the client's call to Hcaptcha client side API.
    response: HcaptchaClientResponse,
    /// The secret_key related to the site_key used to capture the response.
    secret: HcaptchaSecret,
    /// Optional: The ip address of the client providing the response.
    user_ip: Option<IpAddr>,
    /// Optional: The site_key used by the client to generate the response (recommended).
    site_key: Option<String>,
}

#[allow(missing_doc_code_examples)]
impl HcaptchaRequest {
    /// Create a new HcaptchaRequest
    ///
    /// # Input
    ///
    /// The Hcaptcha API has two mandatory paramaters:
    ///     secret:     The client's secret key for authentication
    ///     response:   The response code to validate
    ///
    /// # Output
    ///
    /// A HcaptchaRequest struct is returned if the input strings are valid.
    /// A HcaptchaError is returned if the validation fails.
    ///
    /// # Example
    ///
    /// ``` no_run
    ///     use hcaptcha::HcaptchaRequest;
    /// # fn main() -> Result<(), hcaptcha::HcaptchaError>{
    ///     let secret = get_your_secret();     // your secret key
    ///     let bad_token = get_your_token();   // user's token
    ///
    ///     let request = HcaptchaRequest::new(&secret, &bad_token)?; // <- returns error
    /// # Ok(())
    /// # }
    /// # fn get_your_secret() -> String {
    /// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    /// # }
    /// # fn get_your_token() -> String {
    /// #    "thisisnotapropertoken".to_string()
    /// # }
    ///  ```
    /// # Logging
    ///
    /// If the tracing feature is enabled a debug level span is set for the
    /// method.
    /// The secret field will not be logged.
    ///
    #[allow(dead_code)]
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(
            name = "Create new HcaptchaRequest with required paramaters.",
            skip(secret),
            level = "debug"
        )
    )]
    pub fn new(secret: &str, response: &str) -> Result<HcaptchaRequest, HcaptchaError> {
        Ok(HcaptchaRequest {
            response: HcaptchaClientResponse::parse(response.to_owned())?,
            secret: HcaptchaSecret::parse(secret.to_owned())?,
            ..HcaptchaRequest::default()
        })
    }

    /// Specify the optional ip address value
    ///
    /// Mutate the HcaptchaRequest by adding the client Ip Address.
    ///
    /// # Example
    /// ``` no_run
    ///     use hcaptcha::HcaptchaRequest;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    ///     let secret = get_your_secret();         // your secret key
    ///     let bad_token = get_your_token();       // user's response token
    ///     let user_ip = get_user_ip_address();    // user's ip address
    ///
    ///     let request = HcaptchaRequest::new(&secret, &bad_token)? // <- returns error
    ///         .set_user_ip(user_ip);
    /// # Ok(())
    /// # }
    /// # fn get_your_secret() -> String {
    /// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    /// # }
    /// # fn get_your_token() -> String {
    /// #    "thisisnotapropertoken".to_string()
    /// # }
    /// # use std::net::{IpAddr, Ipv4Addr};
    /// # fn get_user_ip_address() -> IpAddr {
    /// #    IpAddr::V4(Ipv4Addr::new(192, 168, 0, 17))
    /// # }
    ///
    /// ```
    ///
    /// #Logging
    ///
    /// If the `trace` feature is enabled a debug level span is set for the
    /// method.
    /// The secret field is not logged.
    ///
    #[allow(dead_code)]
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(
            name = "Request verification from hcaptcha.",
            skip(self),
            fields(response = %self.response),
            level = "debug"
        )
    )]
    pub fn set_user_ip(mut self, user_ip: IpAddr) -> HcaptchaRequest {
        self.user_ip = Some(user_ip);
        self
    }

    /// Specify the optional site_key value
    ///
    /// Mutate the HcaptchaRequest by adding the site_key.
    ///
    /// # Example
    /// Create a new request and set the site_key field in the request.
    /// ```
    ///     use hcaptcha::HcaptchaRequest;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    ///     let secret = get_your_secret();     // your secret key
    ///     let bad_token = get_your_token();   // user's token
    ///     let site_key = get_your_site_key(); // your site key
    ///
    ///     let request = HcaptchaRequest::new(&secret, &bad_token)? // <- returns error
    ///         .set_site_key(site_key);
    /// # Ok(())
    /// # }
    /// # fn get_your_secret() -> String {
    /// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    /// # }
    /// # fn get_your_token() -> String {
    /// #    "thisisnotapropertoken".to_string()
    /// # }
    /// # use uuid::Uuid;
    /// # fn get_your_site_key() -> Uuid {
    /// #    Uuid::new_v4()
    /// # }
    ///
    /// ```
    ///
    /// #Logging
    ///
    /// If the `trace` feature is enabled a debug level span is created for the
    /// method.
    /// The secret field is not logged.
    ///
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(
            name = "Request verification from hcaptcha.",
            skip(self),
            fields(response = %self.response),
            level = "debug"
        )
    )]
    pub fn set_site_key(mut self, site_key: Uuid) -> HcaptchaRequest {
        self.site_key = Some(site_key.to_hyphenated().to_string());
        self
    }
}
#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::error::Code::*;
    // use crate::HcaptchaError::*;
    // use std::collections::HashSet;
}
