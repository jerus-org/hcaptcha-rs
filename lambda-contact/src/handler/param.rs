use super::error::LambdaContactError;
use rusoto_core::Region;
use rusoto_ssm::{GetParameterRequest, Ssm, SsmClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
struct Captcha {
    #[serde(rename = "captchaResponse")]
    captcha_response: String,
}

#[tracing::instrument(name = "validate the captcha")]
pub async fn get_parameter(
    parameter_key: &str,
    decrypt: bool,
) -> Result<String, LambdaContactError> {
    let request = GetParameterRequest {
        name: parameter_key.to_owned(),
        with_decryption: Some(decrypt),
    };

    tracing::debug!("Request for parameter value: {:?}", request);

    let client = SsmClient::new(Region::EuWest1);

    let result = client.get_parameter(request).await?;

    let hcaptcha_secret = match result.parameter {
        Some(param) => match param.value {
            Some(value) => value,
            None => "".to_owned(),
        },
        None => "".to_owned(),
    };

    Ok(hcaptcha_secret)
}
