//! Collect the required and optional parameters supplied by the client for the
//! hcaptcha api request.
//!
//! # Example
//! Create HcaptcaCaptcha struct from the body of JSON submitted as an event.
//! ```no_run
//!     use hcaptcha::HcaptchaCaptcha;
//! # #[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
//! # pub struct CustomEvent {
//! #   body: Option<String>,
//! # }
//! # #[tokio::main]
//! # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
//! # let e = CustomEvent {
//! #         body: Some("{\"response\":\"thisisthelonglistofcharactersthatformsaresponse\",\"user_ip\":\"10.10.20.10\"}".to_owned()),
// //! #         body: None,
//! # };
//!     // Create captcha struct from json string provided by client in
//!     // the body of the http post that submitted a form.
//!     let body_str = e.body.unwrap_or_else(|| "".to_owned());
//!     let captcha: HcaptchaCaptcha = serde_json::from_str(&body_str)?;
//!
//!  # Ok(())
//! # }
//! ```

use crate::domain::{HcaptchaClientResponse, HcaptchaSiteKey, HcaptchaUserIp};
use crate::HcaptchaError;

/// Capture the Hcaptcha data coming from the client.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct HcaptchaCaptcha {
    /// The response string collected by client from Hcaptcha.
    pub(crate) response: HcaptchaClientResponse,
    /// The user_ip of the client submitting the request.
    pub(crate) user_ip: Option<HcaptchaUserIp>,
    /// The site_key submitted to Hcaptcha by the client.
    pub(crate) site_key: Option<HcaptchaSiteKey>,
}

impl HcaptchaCaptcha {
    /// Create a new HcaptchaCaptcha from a response string slice.
    ///
    /// # Input
    ///
    /// response - The response token from the client
    ///
    /// # Output
    ///
    /// The HcaptchaCaptcha is returned if the input is valid.
    /// A [HcaptchaError] is returned if the validation fails.
    ///
    /// # Example
    /// Create HcaptchaCaptcha from response key.
    /// ```no_run
    ///     use hcaptcha::HcaptchaCaptcha;
    /// # #[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
    /// # struct CustomEvent {
    /// #   body: Option<String>,
    /// # }
    /// #
    /// # #[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
    /// # struct Form {
    /// #   name: Option<String>,
    /// #   email: Option<String>,
    /// #   phone: Option<String>,
    /// #   note: Option<String>,
    /// #   response: String,
    /// # }
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    /// # let e = CustomEvent {
    /// #         body: Some("{\"response\":\"thisisthelonglistofcharactersthatformsaresponse\",\"user_ip\":\"10.10.20.10\"}".to_owned()),
    // //! #         body: None,
    /// # };
    ///     // Get the body JSON string from the event.
    ///     let body_str = e.body.unwrap_or_else(|| "".to_owned());
    ///     // Get the form data from the body string.
    ///     let form: Form = serde_json::from_str(&body_str)?;
    ///
    ///     let captcha = HcaptchaCaptcha::new(&form.response)?;
    ///  # Ok(())
    /// # }
    ///
    /// ```
    /// # Logging
    ///
    /// If the tracing feature is enabled a debug level span is set for the
    /// method.
    ///
    #[allow(dead_code)]
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(
            name = "Create new HcaptchaCaptcha from a response string.",
            level = "debug"
        )
    )]
    pub fn new(response: &str) -> Result<Self, HcaptchaError> {
        Ok(HcaptchaCaptcha {
            response: HcaptchaClientResponse::parse(response.to_owned())?,
            user_ip: None,
            site_key: None,
        })
    }

    /// Update the user_ip field in HcaptchaCaptcha.
    ///
    /// # Input
    ///
    /// user_ip - The response token from the client
    ///
    /// # Output
    ///
    /// If the user_ip string is empty the field is set to None.
    /// If the user_ip string is a valid v4 or v6 ip address the field is
    /// set to Some(user_ip).
    /// If the user_ip string is invalid a [HcaptchaError] is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    ///     use hcaptcha::HcaptchaCaptcha;
    /// # use claim::assert_some;
    /// # #[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
    /// # struct CustomEvent {
    /// #   body: Option<String>,
    /// # }
    /// #
    /// # #[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
    /// # struct Form {
    /// #   name: Option<String>,
    /// #   email: Option<String>,
    /// #   phone: Option<String>,
    /// #   note: Option<String>,
    /// #   response: String,
    /// # }
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    /// # let e = CustomEvent {
    /// #         body: Some("{\"response\":\"thisisthelonglistofcharactersthatformsaresponse\",\"user_ip\":\"10.10.20.10\"}".to_owned()),
    // //! #         body: None,
    /// # };
    /// #     // Get the body JSON string from the event.
    /// #    let body_str = e.body.unwrap_or_else(|| "".to_owned());
    /// #    // Get the form data from the body string.
    /// #    let form: Form = serde_json::from_str(&body_str)?;
    ///     let user_ip = get_user_ip_address();
    ///
    ///     let captcha = HcaptchaCaptcha::new(&form.response)?
    ///                     .set_user_ip(&user_ip)?;
    ///
    ///     assert_some!(captcha.user_ip());
    ///     if let Some(sk) = captcha.user_ip() {
    ///             assert_eq!(user_ip, sk.to_string());
    ///     };
    ///  # Ok(())
    /// # }
    /// # fn get_user_ip_address() -> String {
    /// #    fakeit::internet::ipv4_address()
    /// # }
    /// ```
    /// # Logging
    ///
    /// If the tracing feature is enabled a debug level span is set for the
    /// method.
    ///
    #[allow(dead_code)]
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(name = "Update user_ip field in HcaptchaCaptcha.", level = "debug")
    )]
    pub fn set_user_ip(&mut self, user_ip: &str) -> Result<Self, HcaptchaError> {
        if user_ip.is_empty() {
            self.user_ip = None;
        } else {
            self.user_ip = Some(HcaptchaUserIp::parse(user_ip.to_owned())?);
        };

        Ok(self.clone())
    }

    /// Update the user_ip field in HcaptchaCaptcha.
    ///
    /// # Input
    ///
    /// site_key - The response token from the client
    ///
    /// # Output
    ///
    /// If the site_key string is empty the field is set to None.
    /// If the site_key string is a valid uuid the field is set to Some(site_key).
    /// If the site_key string is invalid a [HcaptchaError] is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    ///     use hcaptcha::HcaptchaCaptcha;
    /// # use claim::assert_some;
    /// # #[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
    /// # struct CustomEvent {
    /// #   body: Option<String>,
    /// # }
    /// #
    /// # #[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
    /// # struct Form {
    /// #   name: Option<String>,
    /// #   email: Option<String>,
    /// #   phone: Option<String>,
    /// #   note: Option<String>,
    /// #   response: String,
    /// # }
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    /// # let e = CustomEvent {
    /// #         body: Some("{\"response\":\"thisisthelonglistofcharactersthatformsaresponse\",\"user_ip\":\"10.10.20.10\"}".to_owned()),
    // //! #         body: None,
    /// # };
    ///     // Get the body JSON string from the event.
    ///     let body_str = e.body.unwrap_or_else(|| "".to_owned());
    ///     // Get the form data from the body string.
    ///     let form: Form = serde_json::from_str(&body_str)?;
    ///     let site_key = get_site_key();
    ///
    ///     let captcha = HcaptchaCaptcha::new(&form.response)?
    ///                     .set_site_key(&site_key)?;
    ///
    ///     assert_some!(captcha.site_key());
    ///     if let Some(sk) = captcha.site_key() {
    ///             assert_eq!(site_key, sk.to_string());
    ///     };
    ///
    ///  # Ok(())
    /// # }
    /// # fn get_site_key() -> String {
    /// #    fakeit::unique::uuid_v4()
    /// # }
    /// ```
    /// # Logging
    ///
    /// If the tracing feature is enabled a debug level span is set for the
    /// method.
    ///
    #[allow(dead_code)]
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(name = "Update site_key field in HcaptchaCaptcha.", level = "debug")
    )]
    pub fn set_site_key(&mut self, site_key: &str) -> Result<Self, HcaptchaError> {
        if site_key.is_empty() {
            self.site_key = None;
        } else {
            self.site_key = Some(HcaptchaSiteKey::parse(site_key.to_owned())?);
        };

        Ok(self.clone())
    }

    /// Return the valud of the response field.
    ///
    /// # Output
    ///
    /// A string containing the value of the response field.
    ///
    /// # Example
    ///
    /// ```no_run
    ///     use hcaptcha::HcaptchaCaptcha;
    /// # fn main() {
    ///     let (response, captcha) = get_captcha();
    ///     
    ///     assert_eq!(response, captcha.response().to_string());
    ///
    /// # }
    /// # use rand::distributions::Alphanumeric;
    /// # use rand::{thread_rng, Rng};
    /// # use std::iter;
    /// # fn random_response() -> String {
    /// #     let mut rng = thread_rng();
    /// #     iter::repeat(())
    /// #         .map(|()| rng.sample(Alphanumeric))
    /// #         .map(char::from)
    /// #         .take(100)
    /// #         .collect()
    /// # }
    /// #
    /// # fn get_captcha() -> (String, HcaptchaCaptcha) {
    /// #     let response = random_response();
    /// #     let captcha = HcaptchaCaptcha::new(&response)
    /// #         .unwrap()
    /// #         .set_user_ip(&fakeit::internet::ipv4_address())
    /// #         .unwrap()
    /// #         .set_site_key(&fakeit::unique::uuid_v4())
    /// #         .unwrap();
    /// #     (response, captcha)
    /// # }
    /// # fn get_site_key() -> String {
    /// #    fakeit::unique::uuid_v4()
    /// # }
    /// ```
    /// # Logging
    ///
    /// If the tracing feature is enabled a debug level span is set for the
    /// method.
    ///
    #[allow(dead_code)]
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(name = "Get response field.", level = "debug")
    )]
    pub fn response(self) -> HcaptchaClientResponse {
        self.response
    }

    /// Get the value of the user_ip field.
    ///
    /// # Output
    ///
    /// An [Option] enum containing the value of the user_ip in the [Some]
    /// variant or a [None] variant if the value is not set.
    ///
    /// # Example
    ///
    /// ```no_run
    ///     use hcaptcha::HcaptchaCaptcha;
    /// # use claim::assert_some;
    /// # fn main() {
    ///     let (user_ip, captcha) = get_captcha();
    ///     
    ///     let value = captcha.user_ip();
    ///     assert_some!(&value);
    ///
    ///     if let Some(v) = value {
    ///         assert_eq!(user_ip, v.to_string());
    ///     }
    ///
    /// # }
    /// # use rand::distributions::Alphanumeric;
    /// # use rand::{thread_rng, Rng};
    /// # use std::iter;
    /// # fn random_response() -> String {
    /// #     let mut rng = thread_rng();
    /// #     iter::repeat(())
    /// #         .map(|()| rng.sample(Alphanumeric))
    /// #         .map(char::from)
    /// #         .take(100)
    /// #         .collect()
    /// # }
    /// #
    /// # fn get_captcha() -> (String, HcaptchaCaptcha) {
    /// #     let user_ip = fakeit::internet::ipv4_address();
    /// #     let captcha = HcaptchaCaptcha::new(&random_response())
    /// #         .unwrap()
    /// #         .set_user_ip(&user_ip)
    /// #         .unwrap()
    /// #         .set_site_key(&fakeit::unique::uuid_v4())
    /// #         .unwrap();
    /// #     (user_ip, captcha)
    /// # }
    /// ```
    /// # Logging
    ///
    /// If the tracing feature is enabled a debug level span is set for the
    /// method.
    ///
    #[allow(dead_code)]
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(name = "Get user_ip field.", level = "debug")
    )]
    pub fn user_ip(&self) -> Option<HcaptchaUserIp> {
        self.user_ip.clone()
    }

    /// Get the value of the site_key field.
    ///
    /// # Output
    ///
    /// An [Option] enum containing the value of the site_key in the [Some]
    /// variant or a [None] variant if the value is not set.
    ///
    /// # Example
    ///
    /// ```no_run
    ///     use hcaptcha::HcaptchaCaptcha;
    /// # use claim::assert_some;
    /// # fn main() {
    ///     let (site_key, captcha) = get_captcha();
    ///
    ///     let value = captcha.site_key();
    ///     assert_some!(&value);
    ///
    ///     if let Some(v) = value {
    ///         assert_eq!(site_key, v.to_string());
    ///     };
    ///
    /// # }
    /// # use rand::distributions::Alphanumeric;
    /// # use rand::{thread_rng, Rng};
    /// # use std::iter;
    /// # fn random_response() -> String {
    /// #     let mut rng = thread_rng();
    /// #     iter::repeat(())
    /// #         .map(|()| rng.sample(Alphanumeric))
    /// #         .map(char::from)
    /// #         .take(100)
    /// #         .collect()
    /// # }
    /// #
    /// # fn get_captcha() -> (String, HcaptchaCaptcha) {
    /// #     let site_key = fakeit::unique::uuid_v4();
    /// #     let captcha = HcaptchaCaptcha::new(&random_response())
    /// #         .unwrap()
    /// #         .set_user_ip(&fakeit::internet::ipv4_address())
    /// #         .unwrap()
    /// #         .set_site_key(&site_key)
    /// #         .unwrap();
    /// #     (site_key, captcha)
    /// # }
    /// ```
    /// # Logging
    ///
    /// If the tracing feature is enabled a debug level span is set for the
    /// method.
    ///
    #[allow(dead_code)]
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(name = "Get site_key field.", level = "debug")
    )]
    pub fn site_key(&self) -> Option<HcaptchaSiteKey> {
        self.site_key.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Code;
    use claim::{assert_err, assert_none, assert_ok, assert_some};
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::iter;

    fn random_response() -> String {
        let mut rng = thread_rng();
        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(100)
            .collect()
    }

    fn dummy_captcha() -> HcaptchaCaptcha {
        HcaptchaCaptcha::new(&random_response())
            .unwrap()
            .set_user_ip(&fakeit::internet::ipv4_address())
            .unwrap()
            .set_site_key(&fakeit::unique::uuid_v4())
            .unwrap()
    }

    #[test]
    fn response_cannot_be_empty_or_blank() {
        let empty = "";
        assert_err!(HcaptchaCaptcha::new(empty));
        let blank = "   ";
        assert_err!(HcaptchaCaptcha::new(blank));
    }

    #[test]
    fn fail_if_user_ip_not_valid_v4_or_v6_address() {
        let captcha = HcaptchaCaptcha::new("response_string")
            .unwrap()
            .set_user_ip(&fakeit::words::word());
        assert_err!(&captcha);
        if let Err(HcaptchaError::Codes(hs)) = captcha {
            assert!(hs.contains(&Code::InvalidUserIp));
        }
    }
    #[test]
    fn user_ip_is_optional() {
        let captcha = HcaptchaCaptcha::new("response_string")
            .unwrap()
            .set_user_ip(&fakeit::internet::ipv4_address())
            .unwrap();

        assert_some!(captcha.user_ip);
    }

    #[test]
    fn valid_user_id_is_accepted() {
        assert_ok!(HcaptchaCaptcha::new("response_string")
            .unwrap()
            .set_user_ip(&fakeit::internet::ipv4_address()));
    }

    #[test]
    fn fail_if_site_key_not_valid_uuid() {
        let captcha = HcaptchaCaptcha::new("response_string")
            .unwrap()
            .set_site_key(&fakeit::words::word());

        assert_err!(&captcha);
        if let Err(HcaptchaError::Codes(hs)) = captcha {
            assert!(hs.contains(&Code::InvalidSiteKey));
        }
    }
    #[test]
    fn site_key_is_optional() {
        let captcha = HcaptchaCaptcha::new("response_string")
            .unwrap()
            .set_site_key(&fakeit::unique::uuid_v4())
            .unwrap();

        assert_some!(captcha.site_key);
    }

    #[test]
    fn valid_site_key_is_accepted() {
        let captcha = HcaptchaCaptcha::new("response_string")
            .unwrap()
            .set_site_key(&fakeit::unique::uuid_v4())
            .unwrap();

        assert_some!(captcha.site_key());
    }

    #[test]
    fn update_site_key_with_empty_string_yields_none() {
        let mut captcha = dummy_captcha();

        assert_some!(captcha.site_key());
        captcha.set_site_key("").unwrap();

        assert_none!(captcha.site_key());
    }

    #[test]
    fn update_user_ip_with_empty_string_yields_none() {
        let mut captcha = dummy_captcha();

        assert_some!(captcha.user_ip());
        captcha.set_user_ip("").unwrap();

        assert_none!(captcha.user_ip());
    }
}
