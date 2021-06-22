use crate::domain::{HcaptchaClientResponse, HcaptchaSiteKey, HcaptchaUserIp};
use crate::HcaptchaError;

#[derive(Debug, Default, serde::Serialize)]
pub struct HcaptchaCaptcha {
    response: HcaptchaClientResponse,
    user_ip: Option<HcaptchaUserIp>,
    site_key: Option<HcaptchaSiteKey>,
}

impl HcaptchaCaptcha {
    pub fn new(response: &str) -> Result<Self, HcaptchaError> {
        Ok(HcaptchaCaptcha {
            response: HcaptchaClientResponse::parse(response.to_owned())?,
            user_ip: None,
            site_key: None,
        })
    }

    pub fn set_user_ip(mut self, user_ip: &str) -> Result<Self, HcaptchaError> {
        self.user_ip = Some(HcaptchaUserIp::parse(user_ip.to_owned())?);
        Ok(self)
    }

    pub fn set_site_key(mut self, site_key: &str) -> Result<Self, HcaptchaError> {
        self.site_key = Some(HcaptchaSiteKey::parse(site_key.to_owned())?);
        Ok(self)
    }

    pub fn response(self) -> HcaptchaClientResponse {
        self.response
    }

    pub fn user_ip(self) -> Option<HcaptchaUserIp> {
        self.user_ip
    }

    pub fn site_key(self) -> Option<HcaptchaSiteKey> {
        self.site_key
    }
}
