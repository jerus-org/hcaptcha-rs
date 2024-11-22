use super::error::ContactError;
use super::param;
use hcaptcha::{Client, HcaptchaCaptcha, HcaptchaRequest, HcaptchaResponse};

const HCAPTCHA_SECRET: &str = "/hcaptcha/secret";

pub async fn response_valid(captcha: HcaptchaCaptcha) -> Result<HcaptchaResponse, ContactError> {
    let secret = param::get_parameter(HCAPTCHA_SECRET).await?;

    let client = Client::new();

    let request = HcaptchaRequest::new(&secret, captcha)?;

    let res = client.verify_client_response(request).await?;

    Ok(res)
}
