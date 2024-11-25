#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]

//! Hcaptcha
//!
//! # Build the request and verify
//!
//! Initialise a client using the [`Client`] builder to submit requests to the hcaptcha service validation.
//!
//! For each request build the request using the [`Request`] builder.
//!
//! Submit the request using the [`Client`] struct's [`Client::verify`] method.
//!
//! A [`Response`] is returned if the validation was successful or the method fails with a set of [`Error`] [`Code`]s if the validation failed.
//!
//! ## Examples
//!
//! ### Enterprise example (requires `enterprise` feature)
//!
//! Token needs to be supplied by the client.
//! This example will fail as a client-provided token is not used.
//! ```no_run
//!     use hcaptcha::{HcaptchaClient, HcaptchaRequest};
//!
//!     let request = HcaptchaRequest::new(&secret, captcha)?
//!         .set_remoteip(&remoteip)?;
//!
//!     let client = HcaptchaClient::new();
//!
//!     let response = client.verify_client_response(request).await?;
//!
//!     let score = match &response.score() {
//!         Some(v) => *v,
//!         None => 0.0,
//!     };
//!     let score_reasons = match &response.score_reason() {
//!         Some(v) => v.iter().join(", "),
//!         None => "".to_owned(),
//!     };
//!
//!     println!("\tScore: {:?}\n\tReasons: {:?}", score, score_reasons);
//!     # Ok(())
//! ```
//!
//! ### Lambda backend implementation.
//!
//! See examples for more detail.
//!
//!
//! ``` no_run
//!
//!         let body_str = e.body.unwrap_or_else(|| "".to_owned());
//!         let captcha: HcaptchaCaptcha = serde_json::from_str(&body_str)?;
//!
//!         let hcaptcha_secret = param::get_parameter(HCAPTCHA_SECRET).await?;
//!
//!         let request = HcaptchaRequest::new(&hcaptcha_secret,
//!             captcha)?;
//!         
//!         let client = HcaptchaClient::new();
//!         let _response = client.verify_client_response(request).await?;
//!
//!         let contact_form: ContactForm = serde_json::from_str(&body_str)?;
//!
//!         let notify_office_fut = send::notify_office(&contact_form);
//!         let notify_contact_fut = send::notify_contact(&contact_form);
//!         let write_fut = record::write(&contact_form);
//!
//!         let (notify_office, notify_contact, write) =
//!             join!(notify_office_fut, notify_contact_fut, write_fut);
//!
//!         if let Err(e) = notify_contact {
//!             error!("Notification to the contact not sent: {}", e);
//!             return Err("Notification not sent".into());
//!         }
//!
//!         if let Err(e) = notify_office {
//!             error!("Notification to the office not sent: {}", e);
//!             return Err("Info not sent to office".into());
//!         }
//!
//!         if let Err(e) = write {
//!             error!("Contact information not written to database: {}", e);
//!         }
//!
//!         Ok(CustomOutput::new(
//!             200,
//!             format!("{}, thank you for your contact request.", contact_form.name),
//!         ))
//!
//!
//! ```
//! ## Feature Flags
//!
//! The default library includes extended validation for the secret field and use of rustls TLS as the TLS backend
//! Disable this validation by setting default-features = false and to enable nativetls features=["nativetls-backend"]
//!
//! ```toml
//! [dependency]
//! hcaptcha = { version = "2.8.9", default-features = false }
//! ```
//!
//! The following feature flags are available:
//! * `enterprise` - Enable methods to access enterprise service fields in the  `HcaptchaResponse`
//! * `ext` - Enables extended validation of secret
//! * `trace` - Enables tracing instrumentation on all functions. Traces are logged at the debug level. The value of the secret is not logged.
//! * `nativetls-backend` - Enables native-tls backend in reqwests
//! * `rustls-backend` - Enables rustls backend in reqwests
//!
//! # Rust Version
//!
//! This version of hcaptcha requires Rust v1.71 or later.

#[doc(hidden)]
pub(crate) mod domain;
mod hcaptcha;
mod hcaptcha_captcha;
mod hcaptcha_client;
mod hcaptcha_error;
mod hcaptcha_request;
mod hcaptcha_response;

pub use hcaptcha_captcha::HcaptchaCaptcha;
pub use hcaptcha_client::HcaptchaClient;
pub use hcaptcha_client::VERIFY_URL;
pub use hcaptcha_error::Code;
pub use hcaptcha_error::HcaptchaError;
pub use hcaptcha_request::HcaptchaRequest;
pub use hcaptcha_response::HcaptchaResponse;

pub use crate::hcaptcha::Hcaptcha;
pub use hcaptcha_derive::*;
