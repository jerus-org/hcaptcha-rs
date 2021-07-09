use super::error::LambdaContactError;
use tracing::instrument;

#[instrument(name = "get the secret key from parameter store")]
pub async fn get_paramater(key: &str) -> Result<String, LambdaContactError> {
    // Extract the secret key from your parameter store
    Ok("0x123456789abcedf0123456789abcedf012345678".to_owned())
}
