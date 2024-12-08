use thiserror::Error;
#[derive(Error, Debug)]
pub enum ContactError {
    #[error("{0}")]
    Hcaptcha(#[from] hcaptcha::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}
