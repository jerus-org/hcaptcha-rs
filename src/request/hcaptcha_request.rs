//! Request module
//! Provides a struct to collect the data required for
//! the hcaptcha api request.

const VERIFY_URL: &str = "https://hcaptcha.com/siteverify";

use super::HcaptchaClientResponse;
use super::HcaptchaSecret;
use super::HcaptchaServerResponse;
use crate::HcaptchaError;
use reqwest::{Client, Url};
use std::net::IpAddr;
use uuid::Uuid;

/// Type to capture the required and optional data for a call to the hcaptcha API
#[derive(Debug, Default, serde::Serialize)]
pub struct HcaptchaRequest {
    /// The response from the client's call to API
    response: HcaptchaClientResponse,
    /// The secret_key for the site_key used by the client to call the API
    secret: HcaptchaSecret,
    /// The ip address of the client making the call (optional)
    user_ip: Option<String>,
    /// The site_key used by the client to make the call (optional, recommended)
    site_key: Option<String>,
}

impl HcaptchaRequest {
    /// Create a new HcaptchaRequest
    #[allow(dead_code)]
    pub fn new(secret: &str, response: &str) -> Result<HcaptchaRequest, HcaptchaError> {
        Ok(HcaptchaRequest {
            response: HcaptchaClientResponse::parse(response.to_owned())?,
            secret: HcaptchaSecret::parse(secret.to_owned())?,
            ..HcaptchaRequest::default()
        })
    }

    /// Specify the optional ip address value
    #[allow(dead_code)]
    pub fn set_user_ip(&mut self, user_ip: &IpAddr) -> &HcaptchaRequest {
        self.user_ip = Some(user_ip.to_string());
        self
    }

    /// Specify the optional site key value
    #[allow(dead_code)]
    pub fn set_site_key(&mut self, site_key: &Uuid) -> &HcaptchaRequest {
        self.site_key = Some(site_key.to_hyphenated().to_string());
        self
    }

    /// Call the api to verify the response code recieved from the client
    #[allow(dead_code)]
    pub async fn verify(&self) -> Result<HcaptchaServerResponse, HcaptchaError> {
        let url = Url::parse(VERIFY_URL).unwrap();
        let response = Client::new().post(url).form(&self).send().await?;
        let response = response.json::<HcaptchaServerResponse>().await?;
        #[cfg(feature = "trace")]
        tracing::debug!("The response is: {:?}", response);
        Ok(response)
    }
}
#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::error::Code::*;
    // use crate::HcaptchaError::*;
    // use std::collections::HashSet;
}
