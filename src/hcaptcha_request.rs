//! Request module
//! Provides a struct to collect the data required for
//! the hcaptcha api request.

use crate::HcaptchaError;
use std::net::IpAddr;
use uuid::Uuid;

mod hcaptcha_client_response;
mod hcaptcha_secret;

use hcaptcha_client_response::HcaptchaClientResponse;
use hcaptcha_secret::HcaptchaSecret;

/// Type to capture the required and optional data for a call to the hcaptcha API
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
    /// #Input
    ///
    /// The Hcaptcha API has two mandatory paramaters:
    ///     secret:     The client's secret key for authentication
    ///     response:   The response code to validate
    ///
    /// #Output
    ///
    /// A HcaptchaRequest struct is returned if the input strings are valid.
    /// A HcaptchaError is returned if the validation fails.
    ///
    /// #Logging
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
