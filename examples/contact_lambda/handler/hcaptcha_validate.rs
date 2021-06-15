use super::error::LambdaContactError;
use super::param;
use hcaptcha::{Hcaptcha, HcaptchaClient, HcaptchaRequest, HcaptchaResponse};

const HCAPTCHA_SECRET: &str = "/hcaptcha/secret";

pub async fn response_valid(response: &str) -> Result<HcaptchaResponse, LambdaContactError> {
    let secret = param::get_paramater(HCAPTCHA_SECRET).await?;

    let client = HcaptchaClient::new();

    let request = HcaptchaRequest::new(&secret, response)?;

    let res = client.verify_client_response(request).await?;

    Ok(res)
}
