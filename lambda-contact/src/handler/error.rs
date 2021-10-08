// use dynomite::dynamodb::PutItemError;
// use hcaptcha::HcaptchaError;
// use rusoto_core::RusotoError;
// use rusoto_ses::SendEmailError;
// use rusoto_ses::SendTemplatedEmailError;
// use rusoto_ssm::GetParameterError;
use thiserror::Error;
// use std::collections::HashSet;
#[derive(Error, Debug)]
pub enum LambdaContactError {
    #[error("{0}")]
    Hcaptcha(#[from] hcaptcha::HcaptchaError),
    #[error("{0}")]
    RusotoSsm(#[from] rusoto_core::RusotoError<rusoto_ssm::GetParameterError>),
    #[error("{0}")]
    RusotoSes(#[from] rusoto_core::RusotoError<rusoto_ses::SendEmailError>),
    #[error("{0}")]
    RusotoSesTemplate(#[from] rusoto_core::RusotoError<rusoto_ses::SendTemplatedEmailError>),
    #[error("{0}")]
    DynomoDb(#[from] rusoto_core::RusotoError<rusoto_dynamodb::PutItemError>),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

// impl From<LambdaContactError> for lambda_runtime::Error {
//     fn from(err: LambdaContactError) -> lambda_runtime::Error {
//         lambda_runtime::Error::from(err.to_string().as_str())
//     }
// }
