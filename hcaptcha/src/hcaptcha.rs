//! Hcaptcha Trait
use crate::{Error, Response};
// use async_trait::async_trait;
use std::{future::Future, pin::Pin};

/// Hcaptcha trait
pub trait Hcaptcha {
    /// valid response function
    fn valid_response(
        &self,
        secret: &str,
        uri: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, Error>>>>;
}
