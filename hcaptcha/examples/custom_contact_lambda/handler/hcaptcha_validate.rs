use super::error::LambdaContactError;
use super::param;
use hcaptcha::{HcaptchaCaptcha, HcaptchaClient, HcaptchaRequest, HcaptchaResponse};

const HCAPTCHA_SECRET: &str = "/hcaptcha/secret";

pub async fn response_valid(
    captcha: HcaptchaCaptcha,
) -> Result<HcaptchaResponse, LambdaContactError> {
    let secret = param::get_parameter(HCAPTCHA_SECRET).await?;

    let client = HcaptchaClient::new();

    let request = HcaptchaRequest::new(&secret, captcha)?;

    let res = client.verify_client_response(request).await?;

    Ok(res)
}
