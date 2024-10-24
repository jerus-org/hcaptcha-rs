#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]

//! Hcaptcha
//!
//! # Build the request and verify
//!
//! Build the request using the [`Hcaptcha`] builder.
//!
//! Execute [`verify`] on the request once to execute.
//!
//! Following a successful response the additional response in can be
//! requested from the [`Hcaptcha`] struct.
//!
//! [`Hcaptcha`]: ./struct.hcaptcha_request.Hcaptcha.html
//! [`verify`]: ./function.hcaptcha_request.verify.html
//!
//! # Examples
//!
//! Token needs to be supplied by the client.
//! This example will fail as a client-provided token is not used.
//! ```no_run
//!     use hcaptcha::{HcaptchaClient, HcaptchaRequest};
//! # use itertools::Itertools;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), hcaptcha::HcaptchaError> {
//! #   let secret = get_your_secret();
//! #   let captcha = get_captcha();
//! #   let remoteip = get_remoteip_address();
//!
//!     let request = HcaptchaRequest::new(&secret, captcha)?
//!         .set_remoteip(&remoteip)?;
//!
//!     let client = HcaptchaClient::new();
//!
//!     let response = client.verify_client_response(request).await?;
//!
//! # #[cfg(feature = "enterprise")]
//!     let score = match &response.score() {
//!         Some(v) => *v,
//!         None => 0.0,
//!     };
//! # #[cfg(feature = "enterprise")]
//!     let score_reasons = match &response.score_reason() {
//!         Some(v) => v.iter().join(", "),
//!         None => "".to_owned(),
//!     };
//! # #[cfg(feature = "enterprise")]
//!     println!("\tScore: {:?}\n\tReasons: {:?}", score, score_reasons);
//!     # Ok(())
//! # }
//! # fn get_your_secret() -> String {
//! #   "0x123456789abcde0f123456789abcdef012345678".to_string()
//! # }
//! # use hcaptcha::HcaptchaCaptcha;
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
//! # fn get_captcha() -> HcaptchaCaptcha {
//! #    HcaptchaCaptcha::new(&random_response())
//! #       .unwrap()
//! #       .set_remoteip(&mockd::internet::ipv4_address())
//! #       .unwrap()
//! #       .set_sitekey(&mockd::unique::uuid_v4())
//! #       .unwrap()
//! #       }
//! # fn get_remoteip_address() -> String {
//! #    "192.168.0.17".to_string()
//! # }
//! # use uuid::Uuid;
//! # fn get_your_sitekey() -> Uuid {
//! #    Uuid::new_v4()
//! # }
//! ```
//!
//! Lambda backend implementation. See examples for more detail.
//!
//! ``` no_run
//! # use lambda_runtime::Error;
//! # use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
//! # use tracing_log::LogTracer;
//! # use tracing_subscriber::layer::SubscriberExt;
//! # use tracing_subscriber::{EnvFilter, Registry};
//! #
//! mod handler {
//! #     mod error {
//! #         use thiserror::Error;
//! #         #[derive(Error, Debug)]
//! #         pub enum ContactError {
//! #             #[error("{0}")]
//! #             Hcaptcha(#[from] hcaptcha::HcaptchaError),
//! #             #[error("{0}")]
//! #             Json(#[from] serde_json::Error),
//! #         }
//! #     }
//! #
//! #     mod param {
//! #         use super::error::ContactError;
//! #         use tracing::instrument;
//! #         #[instrument(name = "get the secret key from parameter store")]
//! #         pub async fn get_parameter(key: &str) -> Result<String, ContactError> {
//! #             // Extract the secret key from your parameter store
//! #             Ok("0x123456789abcedf0123456789abcedf012345678".to_owned())
//! #         }
//! #     }
//! #
//! #     mod record {
//! #         use super::error::ContactError;
//! #         use super::send::ContactForm;
//! #         use tracing::instrument;
//! #
//! #         #[instrument(
//! #     name = "Write record to database"
//! #     skip(form)
//! #     fields(email = %form.email)
//! # )]
//! #         pub async fn write(form: &ContactForm) -> Result<(), ContactError> {
//! #             // Write the contact form data to dynamodb
//! #             Ok(())
//! #         }
//! #     }
//! #
//! #     mod send {
//! #         use super::error::ContactError;
//! #         use serde::{Deserialize, Serialize};
//! #         use tracing::instrument;
//! #
//! #         #[derive(Deserialize, Serialize, Clone, Debug, Default)]
//! #         pub struct ContactForm {
//! #             #[serde(default)]
//! #             pub name: String,
//! #             #[serde(default)]
//! #             pub phone: String,
//! #             #[serde(default)]
//! #             pub email: String,
//! #             #[serde(default)]
//! #             pub message: String,
//! #             #[serde(default)]
//! #             pub page: String,
//! #             #[serde(default)]
//! #             pub site: String,
//! #         }
//! #
//! #         #[instrument(name = "send notification to info mailbox", skip(_contact_form))]
//! #         pub async fn notify_office(
//! #             _contact_form: &ContactForm,
//! #         ) -> Result<(), ContactError> {
//! #             // Construct email and send message to the office info mailbox
//! #
//! #             Ok(())
//! #         }
//! #
//! #         #[instrument(name = "Send notification to the contact", skip(_contact_form))]
//! #         pub async fn notify_contact(
//! #             _contact_form: &ContactForm,
//! #         ) -> Result<(), ContactError> {
//! #             // Construct and send email to the contact
//! #
//! #             Ok(())
//! #         }
//! #     }
//!
//! #     const HCAPTCHA_SECRET: &str = "/hcaptcha/secret";
//! #
//! #     use hcaptcha::{HcaptchaCaptcha, HcaptchaClient, HcaptchaRequest};
//! #     use lambda_runtime::{Context, Error};
//! #     use send::ContactForm;
//! #     use serde::{Deserialize, Serialize};
//! #     use tokio::join;
//! #     use tracing::{debug, error};
//! #
//! #     #[derive(Deserialize, Serialize, Clone, Debug, Default)]
//! #     pub struct CustomEvent {
//! #         body: Option<String>,
//! #     }
//! #
//! #     #[derive(Deserialize, Serialize, Clone, Default)]
//! #     pub struct Recaptcha {
//! #         #[serde(rename = "reCaptchaResponse")]
//! #         re_captcha_response: String,
//! #     }
//! #
//! #     #[derive(Serialize, Clone, Debug, PartialEq)]
//! #     pub struct CustomOutput {
//! #         #[serde(rename = "isBase64Encoded")]
//! #         is_base64_encoded: bool,
//! #         #[serde(rename = "statusCode")]
//! #         status_code: u16,
//! #         body: String,
//! #     }
//! #
//! #     impl CustomOutput {
//! #         fn new(status_code: u16, body: String) -> CustomOutput {
//! #             CustomOutput {
//! #                 is_base64_encoded: false,
//! #                 status_code,
//! #                 body,
//! #             }
//! #         }
//! #     }
//! #
//! #
//!     pub async fn my_handler(e: CustomEvent, _c: Context) -> Result<CustomOutput,  Error> {
//!         debug!("The event logged is: {:?}", e);
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
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//! #    LogTracer::init()?;
//! #
//! #    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
//! #    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
//! #    let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking_writer);
//! #    let subscriber = Registry::default()
//! #        .with(EnvFilter::new(
//! #            std::env::var("RUST_LOG").unwrap_or_else(|_| "INFO".to_owned()),
//! #        ))
//! #        .with(JsonStorageLayer)
//! #        .with(bunyan_formatting_layer);
//! #    tracing::subscriber::set_global_default(subscriber)?;
//!
//!     lambda_runtime::run(lambda_runtime::handler_fn(handler::my_handler)).await?;
//!     Ok(())
//! }
//!
//! ```
//! # Feature Flags
//!
//! The default library includes extended validation for the secret field and use of native TLS as the TLS backend
//! Disable this validation by setting default-features = false and to enable rustls features=["rustls"]
//!
//! ```toml
//! [dependency]
//! hcaptcha = { version = "2.7.7", default-features = false }
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
