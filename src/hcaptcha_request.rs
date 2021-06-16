//! Provides a struct to collect the required and optional parameters for
//! the hcaptcha api request.

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
    /// The response from the client's call to API
    response: HcaptchaClientResponse,
    /// The secret_key for the site_key used by the client to call the API
    secret: HcaptchaSecret,
    /// The ip address of the client making the call (optional)
    user_ip: Option<IpAddr>,
    /// The site_key used by the client to make the call (optional, recommended)
    site_key: Option<String>,
}

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
    /// ```no_run
    /// use hcaptcha::HcaptchaRequest;
    /// # main() {
    /// let secret = get_seret_from_store();
    /// let response = get_response_from_client();
    ///  
    /// let request = HcaptchaRequest::new(secret, response);
    ///
    /// # }
    /// ```
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
    /// #Input
    ///
    ///     user_ip     The ip address of the client providing the response
    ///
    /// #Output
    ///
    ///     Updated HcaptchaRequest structure.
    ///
    /// #Logging
    ///
    /// If the tracing feature is enabled a debug level span is set for the
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
    pub fn set_user_ip(&mut self, user_ip: IpAddr) -> &HcaptchaRequest {
        self.user_ip = Some(user_ip);
        self
    }

    /// Specify the optional site_key value
    ///
    /// Mutate the HcaptchaRequest by adding the site_key.
    ///
    /// #Input
    ///
    ///     site_key     The site_key used to generate the response
    ///
    /// #Output
    ///
    ///     Updated HcaptchaRequest structure.
    ///
    /// # Example
    /// Create a new request and set the user_ip fied in the request.
    /// ```should_panic no_run
    /// use hcaptcha::HcaptchaRequest;
    /// use std::net::{IpAddr, Ipv4Addr};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    /// let secret = "0x0000000000000000000000000000000000000000"; // your secret key
    /// let token = "this_is_likely_invalid";  // user's token
    /// let user_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 17));
    ///
    /// let request = HcaptchaRequest::new(secret, token)?
    ///     .set_user_ip(&user_ip);
    ///
    /// # }
    /// ```
    ///
    /// #Logging
    ///
    /// If the tracing feature is enabled a debug level span is set for the
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
    pub fn set_site_key(&mut self, site_key: Uuid) -> &HcaptchaRequest {
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
