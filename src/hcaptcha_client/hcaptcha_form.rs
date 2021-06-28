use crate::HcaptchaRequest;

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct HcaptchaForm {
    response: String,
    remoteip: Option<String>,
    sitekey: Option<String>,
    secret: String,
}

impl From<HcaptchaRequest> for HcaptchaForm {
    fn from(request: HcaptchaRequest) -> HcaptchaForm {
        let remoteip = request.captcha().remoteip.map(|v| v.to_string());
        let sitekey = request.captcha().sitekey.map(|v| v.to_string());

        HcaptchaForm {
            response: request.captcha().response.to_string(),
            remoteip,
            sitekey,
            secret: request.secret().to_string(),
        }
    }
}
