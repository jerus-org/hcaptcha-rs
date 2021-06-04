#![cfg_attr(docsrs, feature(doc_cfg))]
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
//!     let res = Hcaptcha::new(secret, token)?
//!                 .set_user_ip(&remote_ip)
//!                 .verify()
//!                 .await;
//!
//!     if res.is_ok() {
//!         println!("Success");
//!     } else {
//!         println!("Failure"); //  <- result as token is arbitrary
//!     }
//!     # Ok(())
//! }
//! ```
//!
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

mod error;
mod hcaptcha_builder;
mod request;

pub use error::Code;
pub use error::HcaptchaError;
pub use hcaptcha_builder::*;
