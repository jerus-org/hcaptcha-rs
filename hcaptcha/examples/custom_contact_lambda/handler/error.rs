use thiserror::Error;
#[derive(Error, Debug)]
pub enum ContactError {
    #[error("{0}")]
    Hcaptcha(#[from] hcaptcha::HcaptchaError),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}
