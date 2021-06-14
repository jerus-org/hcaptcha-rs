use crate::HcaptchaError;
use crate::HcaptchaRequest;
use crate::HcaptchaResponse;
use async_trait::async_trait;

/// Trait representing the capabilities of the Hcaptcha api.
/// This trait is implemented by the HcaptchaClient struct.
#[async_trait]
pub trait Hcaptcha {
    async fn verify_client_response(
        &self,
        client_response: HcaptchaRequest,
    ) -> Result<HcaptchaResponse, HcaptchaError>;
}
