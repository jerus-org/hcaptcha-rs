//! Hcaptcha Trait
use crate::{HcaptchaError, HcaptchaResponse};
use async_trait::async_trait;

/// Hcaptcha trait
#[async_trait]
pub trait Hcaptcha {
    /// valid response function
    async fn valid_response(&self, secret: &str) -> Result<HcaptchaResponse, HcaptchaError>;
}
