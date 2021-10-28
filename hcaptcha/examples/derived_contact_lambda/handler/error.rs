use thiserror::Error;
#[derive(Error, Debug)]
pub enum LambdaContactError {
    #[error("{0}")]
    Hcaptcha(#[from] hcaptcha::HcaptchaError),
    #[error("{0}")]
    RusotoSes(#[from] rusoto_core::RusotoError<rusoto_ses::SendEmailError>),
    #[error("{0}")]
    RusotoSesTemplate(#[from] rusoto_core::RusotoError<rusoto_ses::SendTemplatedEmailError>),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}
