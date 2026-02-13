// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Collect the required and optional parameters for the hcaptcha api request.
//!
//! # Example
//!
//! ```
//!     use hcaptcha::Request;
//! # #[tokio::main]
//! # async fn main() -> Result<(), hcaptcha::Error> {
//!     let secret = get_your_secret();         // your secret key
//!     let captcha = get_captcha();            // user's response token
//!     let sitekey = get_your_sitekey();     // your site key
//!     let remoteip = get_remoteip_address();    // user's ip address
//!
//!     let request = Request::new(&secret, captcha)?
//!         .set_sitekey(&sitekey)?
//!         .set_remoteip(&remoteip)?;
//! # Ok(())
//! # }
//! # fn get_your_secret() -> String {
//! #   "0x123456789abcde0f123456789abcdef012345678".to_string()
//! # }
//! # use hcaptcha::Captcha;
//! # use rand::distr::Alphanumeric;
//! # use rand::{rng, RngExt};
//! # use std::iter;
//! # fn random_response() -> String {
//! #    let mut rng = rng();
//! #     (&mut rng)
//! #        .sample_iter(Alphanumeric)
//! #        .take(100)
//! #        .map(char::from)
//! #        .collect()
//! # }
//! # fn get_captcha() -> Captcha {
//! #    Captcha::new(&random_response())
//! #       .unwrap()
//! #       .set_remoteip(&mockd::internet::ipv4_address())
//! #       .unwrap()
//! #       .set_sitekey(&mockd::unique::uuid_v4())
//! #       .unwrap()
//! #       }
//! # fn get_remoteip_address() -> String {
//! #    "192.168.71.17".to_string()
//! # }
//! # use uuid::Uuid;
//! # fn get_your_sitekey() -> String {
//! #    Uuid::new_v4().to_string()
//! # }
//!
//! ```

use crate::domain::Secret;
use crate::Captcha;
use crate::Error;

/// Capture the required and optional data for a call to the hcaptcha API
#[cfg_attr(docsrs, allow(rustdoc::missing_doc_code_examples))]
#[derive(Debug, Default, serde::Serialize)]
pub struct Request {
    /// [Captcha] captures the response and, optionally, the remoteip
    /// and sitekey reported by the client.
    captcha: Captcha,
    /// The secret_key related to the sitekey used to capture the response.
    secret: Secret,
}

#[cfg_attr(docsrs, allow(rustdoc::missing_doc_code_examples))]
impl Request {
    /// Create a new Request
    ///
    /// # Input
    ///
    /// The Hcaptcha API has two mandatory parameters:
    ///     `secret`:     The client's secret key for authentication
    ///     `captcha`:    [Captcha] (including response token)
    ///
    /// # Output
    ///
    /// Request is returned if the input strings are valid.
    /// [Error] is returned if the validation fails.
    ///
    /// # Example
    ///
    /// ``` no_run
    ///     use hcaptcha::Request;
    /// # fn main() -> Result<(), hcaptcha::Error>{
    ///     let secret = get_your_secret();     // your secret key
    ///     let captcha = get_captcha();        // captcha with response token
    ///
    ///     let request = Request::new(&secret, captcha)?;
    /// # Ok(())
    /// # }
    /// # fn get_your_secret() -> String {
    /// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    /// # }
    /// # use hcaptcha::Captcha;
    /// # use rand::distr::Alphanumeric;
    /// # use rand::{rng, RngExt};
    /// # use std::iter;
    /// # fn random_response() -> String {
    /// #    let mut rng = rng();
    /// #     (&mut rng)
    /// #        .sample_iter(Alphanumeric)
    /// #        .take(100)
    /// #        .map(char::from)
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
            name = "Create new Request from Captcha struct.",
            skip(secret),
            level = "debug"
        )
    )]
    pub fn new(secret: &str, captcha: Captcha) -> Result<Request, Error> {
        Ok(Request {
            captcha,
            secret: Secret::parse(secret.to_owned())?,
        })
    }

    /// Create a new Request from only the response string
    ///
    /// # Input
    ///
    /// The Hcaptcha API has two mandatory parameters:
    ///     secret:     The client's secret key for authentication
    ///     response:    The response code to validate
    ///
    /// # Output
    ///
    /// Request is returned if the inputs are valid.
    /// [Error] is returned if the validation fails.
    ///
    /// # Example
    ///
    /// ``` no_run
    ///     use hcaptcha::Request;
    /// # fn main() -> Result<(), hcaptcha::Error>{
    ///     let secret = get_your_secret();     // your secret key
    ///     let response = get_response();    // Hcaptcha client response
    ///
    ///     let request = Request::new_from_response(&secret, &response)?;
    /// # Ok(())
    /// # }
    /// # fn get_your_secret() -> String {
    /// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    /// # }
    /// # use rand::distr::Alphanumeric;
    /// # use rand::{rng, RngExt};
    /// # use std::iter;
    /// # fn get_response() -> String {
    /// #    let mut rng = rng();
    /// #     (&mut rng)
    /// #        .sample_iter(Alphanumeric)
    /// #        .take(100)
    /// #        .map(char::from)
    /// #        .collect()
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
            name = "Create new Request from response string.",
            skip(secret),
            level = "debug"
        )
    )]
    pub fn new_from_response(secret: &str, response: &str) -> Result<Request, Error> {
        let captcha = Captcha::new(response)?;
        Request::new(secret, captcha)
    }

    /// Specify the optional ip address value
    ///
    /// Update client IP address.
    ///
    /// # Example
    /// ``` no_run
    ///     use hcaptcha::Request;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::Error> {
    ///     let secret = get_your_secret();         // your secret key
    ///     let response = get_response();          // user's response token
    ///     let remoteip = get_remoteip_address();    // user's ip address
    ///
    ///     let request = Request::new_from_response(&secret, &response)?
    ///         .set_remoteip(&remoteip)?;
    /// # Ok(())
    /// # }
    /// # fn get_your_secret() -> String {
    /// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    /// # }
    /// # use hcaptcha::Captcha;
    /// # use rand::distr::Alphanumeric;
    /// # use rand::{rng, RngExt};
    /// # use std::iter;
    /// # fn get_response() -> String {
    /// #    let mut rng = rng();
    /// #     (&mut rng)
    /// #        .sample_iter(Alphanumeric)
    /// #        .take(100)
    /// #        .map(char::from)
    /// #        .collect()
    /// # }
    /// # use std::net::{IpAddr, Ipv4Addr};
    /// # fn get_remoteip_address() -> String {
    /// #    "192.168.71.17".to_string()
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
            fields(captcha = ?self.captcha),
            level = "debug"
        )
    )]
    pub fn set_remoteip(mut self, remoteip: &str) -> Result<Self, Error> {
        self.captcha.set_remoteip(remoteip)?;
        Ok(self)
    }

    /// Specify the optional sitekey value
    ///
    /// Update the sitekey.
    ///
    /// # Example
    /// Create a new request and set the sitekey field in the request.
    /// ```
    ///     use hcaptcha::Request;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::Error> {
    ///     let secret = get_your_secret();     // your secret key
    ///     let captcha = get_captcha();        // captcha
    ///     let sitekey = get_your_sitekey();   // your site key
    ///
    ///     let request = Request::new(&secret, captcha)?
    ///         .set_sitekey(&sitekey);
    /// # Ok(())
    /// # }
    /// # fn get_your_secret() -> String {
    /// #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    /// # }
    /// # use hcaptcha::Captcha;
    /// # use rand::distr::Alphanumeric;
    /// # use rand::{rng, RngExt};
    /// # use std::iter;
    /// # fn random_response() -> String {
    /// #    let mut rng = rng();
    /// #     (&mut rng)
    /// #        .sample_iter(Alphanumeric)
    /// #        .take(100)
    /// #        .map(char::from)
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
    /// # use uuid::Uuid;
    /// # fn get_your_sitekey() -> String {
    /// #    Uuid::new_v4().to_string()
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
            fields(captcha = ?self.captcha),
            level = "debug"
        )
    )]
    pub fn set_sitekey(mut self, sitekey: &str) -> Result<Self, Error> {
        self.captcha.set_sitekey(sitekey)?;
        Ok(self)
    }

    #[allow(dead_code)]
    pub(crate) fn secret(&self) -> Secret {
        self.secret.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn captcha(&self) -> Captcha {
        self.captcha.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Captcha;
    use claims::{assert_none, assert_ok};
    use rand::distr::Alphanumeric;
    use rand::{rng, RngExt};

    fn random_hex_string(len: usize) -> String {
        let mut rng = rng();

        let chars: String = (&mut rng)
            .sample_iter(Alphanumeric)
            .take(len / 2)
            .map(char::from)
            .collect();

        hex::encode(chars)
    }

    fn random_response() -> String {
        let mut rng = rng();
        (&mut rng)
            .sample_iter(Alphanumeric)
            .take(100)
            .map(char::from)
            .collect()
    }

    fn dummy_captcha() -> Captcha {
        Captcha::new(&random_response())
            .unwrap()
            .set_remoteip(&mockd::internet::ipv4_address())
            .unwrap()
            .set_sitekey(&mockd::unique::uuid_v4())
            .unwrap()
    }

    #[test]
    fn valid_new_from_captcha_struct() {
        let secret = format!("0x{}", random_hex_string(40));
        let captcha = dummy_captcha();

        assert_ok!(Request::new(&secret, captcha));
    }

    #[test]
    fn valid_new_from_response() {
        let secret = format!("0x{}", random_hex_string(40));
        let response = random_response();

        let request = Request::new_from_response(&secret, &response).unwrap();

        assert_eq!(&secret, &request.secret().to_string().as_str());

        let Captcha {
            response: resp,
            remoteip: ip,
            sitekey: key,
        } = request.captcha;

        assert_eq!(response, resp.to_string());
        assert_none!(ip);
        assert_none!(key);
    }
}
