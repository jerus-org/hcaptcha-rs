use super::error::ContactError;
use super::param;
use hcaptcha::{Captcha, Client, Request, Response};

const HCAPTCHA_SECRET: &str = "/hcaptcha/secret";

pub async fn response_valid(captcha: Captcha) -> Result<Response, ContactError> {
    let secret = param::get_parameter(HCAPTCHA_SECRET).await?;

    let client = Client::new();

    let request = Request::new(&secret, captcha)?;

    let res = client.verify_request(request).await?;

    Ok(res)
}
