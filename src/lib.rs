//! Hcaptcha
//!
//! # Build the request and verify
//!
//! Build the request using the [Hcaptcha] builder.
//!
//! Execute [verify] on the request once to execute.
//!
//! Following a successful response the additional response in
//! [HcaptchaResponse] can be requested from the [Hcapthca] struct.
//!
//! [Hcaptcha]: ./struct.Hcaptcha.html
//! [HcaptchaResponse]: crate::response::HcaptchaResponse
//! [here]: https://docs.hcaptcha.com/#server
//!
//! # Examples
//!
//! ```
//! use hcaptcha::Hcaptcha;
//! use std::net::{IpAddr, Ipv4Addr};
//!
//! #[tokio::main]
//! async fn main() {
//!     let remote_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 17));
//!
//!     let res = Hcaptcha::new("your_private_key", "user_response")
//!                 .set_user_ip(&remote_ip)
//!                 .verify()
//!                 .await;
//!
//!     if res.is_ok() {
//!         println!("Success");
//!     } else {
//!         println!("Failure");
//!     }
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
mod response;

pub use error::Code;
pub use error::HcaptchaError;
pub use hcaptcha_builder::Hcaptcha;
