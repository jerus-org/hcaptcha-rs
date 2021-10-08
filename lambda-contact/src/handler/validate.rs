use super::error::LambdaContactError;
use hcaptcha::{HcaptchaCaptcha, HcaptchaClient, HcaptchaRequest, HcaptchaResponse};
use rusoto_core::Region;
use rusoto_ssm::{GetParameterRequest, Ssm, SsmClient};
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
struct Captcha {
    #[serde(rename = "captchaResponse")]
    captcha_response: String,
}

#[instrument(name = "validate the captcha", skip(body_str))]
pub async fn valid_captcha(body_str: &str) -> Result<HcaptchaResponse, LambdaContactError> {
    let captcha: HcaptchaCaptcha = serde_json::from_str(&body_str)?;

    debug!("The captcha is: {:?}", captcha);

    let parameter_key = "/website/hcaptcha/secret_key".to_owned();
    let decrypt = true;

    let request = GetParameterRequest {
        name: parameter_key,
        with_decryption: Some(decrypt),
    };

    debug!("Request for parameter value: {:?}", request);

    let client = SsmClient::new(Region::EuWest1);

    let result = client.get_parameter(request).await?;

    let hcaptcha_secret = match result.parameter {
        Some(param) => match param.value {
            Some(value) => value,
            None => "".to_owned(),
        },
        None => "".to_owned(),
    };

    let request = HcaptchaRequest::new(&hcaptcha_secret, captcha)?;
    debug!("Request for hcaptcha: {:?}", request);
    let client = HcaptchaClient::new();
    debug!("Client for hcaptcha: {:?}", client);

    let res = client.verify_client_response(request).await?;
    Ok(res)
}
