//! Hcaptcha
//!
//! # Build the request and verify
//!
//! Build the request using the [Hcaptcha] builder.
//!
//! Execute [verify] on the request once to execute.
//!
//! Following a successful response the additional response in
//! [HcaptchaServerResponse] can be requested from the [Hcapthca] struct.
//!
//! [Hcaptcha]: ./struct.Hcaptcha.html
//! [HcaptchaServerResponse]: crate::response::HcaptchaServerResponse
//! [here]: https://docs.hcaptcha.com/#server
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
//!  - `logging:` Enbles debug logs for the request and response structs.
//!
//!
//!

mod error;
mod hcaptcha_builder;
mod request;

pub use error::Code;
pub use error::HcaptchaError;
pub use hcaptcha_builder::Hcaptcha;
