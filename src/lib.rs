#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![warn(private_doc_tests)]
#![warn(invalid_codeblock_attributes)]

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
//! [`verify`]: ./function.hcapthca_request.verify.html
//!
//! # Examples
//! Token needs to be supplied by the client.
//! This example will fail as a client-provided token is not used.
//! ```should_panic no_run
//! use hcaptcha::Hcaptcha;
//! use std::net::{IpAddr, Ipv4Addr};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), hcaptcha::HcaptchaError> {
//!     let secret = "0x0000000000000000000000000000000000000000";
//!     let token = "client_response";
//!     let remote_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 17));
//!
//!     let mut hc = Hcaptcha::new(secret, token)?
//!         .set_user_ip(&remote_ip);
//!
//!     hc.verify().await?;
//!
//!     let score = match hc.score() {
//!         Some(v) => v,
//!         None => 0.0,
//!     };
//!     let score_reasons = match &hc.score_reasons() {
//!         Some(v) => v.iter().join(", "),
//!         None => "".to_owned(),
//!     };
//!     println!("\tScore: {:?}\n\tReasons: {:?}", score, score_reasons);
//!     # Ok(())
//! }
//! ```
//!
//! Lambda backend implemetation. See examples for more detail.
//! ``` no_run
//! # use lambda_runtime::Error;
//! # use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
//! # use tracing_log::LogTracer;
//! # use tracing_subscriber::layer::SubscriberExt;
//! # use tracing_subscriber::{EnvFilter, Registry};
//! #
//! # mod handler {
//! #     mod error {
//! #         use thiserror::Error;
//! #         #[derive(Error, Debug)]
//! #         pub enum LambdaContactError {
//! #             #[error("{0}")]
//! #             Hcaptcha(#[from] hcaptcha::HcaptchaError),
//! #             #[error("{0}")]
//! #             RusotoSes(#[from] rusoto_core::RusotoError<rusoto_ses::SendEmailError>),
//! #             #[error("{0}")]
//! #             RusotoSesTemplate(
//! #                 #[from] rusoto_core::RusotoError<rusoto_ses::SendTemplatedEmailError>,
//! #             ),
//! #             #[error("{0}")]
//! #             Json(#[from] serde_json::Error),
//! #         }
//! #     }
//! #
//! #     mod param {
//! #         use super::error::LambdaContactError;
//! #         use tracing::instrument;
//! #         #[instrument(name = "get the secret key from parameter store")]
//! #         pub async fn get_paramater(key: &str) -> Result<String, LambdaContactError> {
//! #             // Extract the secret key from your parameter store
//! #             Ok("0x123456789abcedf0123456789abcedf012345678".to_owned())
//! #         }
//! #     }
//! #
//! #     mod record {
//! #         use super::error::LambdaContactError;
//! #         use super::send::ContactForm;
//! #         use tracing::instrument;
//! #
//! #         #[instrument(
//! #     name = "Write record to database"
//! #     skip(form)
//! #     fields(email = %form.email)
//! # )]
//! #         pub async fn write(form: &ContactForm) -> Result<(), LambdaContactError> {
//! #             // Write the contact form data to dynamodb
//! #             Ok(())
//! #         }
//! #     }
//! #
//! #     mod send {
//! #         use super::error::LambdaContactError;
//! #         use rusoto_ses::{SendEmailResponse, SendTemplatedEmailResponse};
//! #         use serde_derive::{Deserialize, Serialize};
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
//! #         ) -> Result<SendEmailResponse, LambdaContactError> {
//! #             // Constuct email and send message to the office info mailbox
//! #
//! #             let res = SendEmailResponse {
//! #                 message_id: "generated_message_id".to_owned(),
//! #             };
//! #
//! #             Ok(res)
//! #         }
//! #
//! #         #[instrument(name = "Send notification to the contact", skip(_contact_form))]
//! #         pub async fn notify_contact(
//! #             _contact_form: &ContactForm,
//! #         ) -> Result<SendTemplatedEmailResponse, LambdaContactError> {
//! #             // Construct and send email to the contact
//! #             let res = SendTemplatedEmailResponse {
//! #                 message_id: "generated_message_id".to_owned(),
//! #             };
//! #
//! #             Ok(res)
//! #         }
//! #     }
//!
//! #     const HCAPTCHA_SECRET: &str = "/hcaptcha/secret";
//! #
//! #     use hcaptcha::Hcaptcha;
//! #     use lambda_runtime::{Context, Error};
//! #     use send::ContactForm;
//! #     use serde_derive::{Deserialize, Serialize};
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
//! #     #[derive(Deserialize, Serialize, Clone, Debug, Default)]
//! #     struct Captcha {
//! #         #[serde(rename = "captchaResponse")]
//! #         captcha_response: String,
//! #     }
//! #
//!     pub async fn my_handler(e: CustomEvent, _c: Context) -> Result<CustomOutput,  Error> {
//!         debug!("The event logged is: {:?}", e);
//!
//!         let body_str = e.body.unwrap_or_else(|| "".to_owned());
//!         let captcha: Captcha = serde_json::from_str(&body_str)?;
//!
//!         let hcaptcha_secret = param::get_paramater(HCAPTCHA_SECRET).await?;
//!
//!         Hcaptcha::new(&hcaptcha_secret, &captcha.captcha_response)?
//!             .verify()
//!             .await?;
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
//!             return Err("Notifcation not sent".into());
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
//! The default library includes the extended validation for the secret
//! field. You can disable this validation by setting default-features = false.
//!
//! ```toml
//! [dependency]
//! hcaptcha = { version = "2.0.0", default-features = false }
//! ```
//!
//!  - 'enterprise'
//!     Enable methods to access enterprise service fields in the response
//!     from the Hcaptcha API.
//!  - `ext`
//!     Enables additional validation that the secret conforms to a 40 byte
//!     hexadecimal string.
//! - 'trace'
//!     Enables instrumentation of all functions by tracing.
//!

mod hcaptcha_client;
mod hcaptcha_error;
mod hcaptcha_request;
mod hcaptcha_response;

pub use hcaptcha_client::HcaptchaClient;
pub use hcaptcha_error::Code;
pub use hcaptcha_error::HcaptchaError;
pub use hcaptcha_request::HcaptchaRequest;
pub use hcaptcha_response::HcaptchaResponse;
