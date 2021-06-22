use crate::domain::{HcaptchaClientResponse, HcaptchaSiteKey, HcaptchaUserIp};
use crate::HcaptchaError;

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct HcaptchaCaptcha {
    pub(crate) response: HcaptchaClientResponse,
    pub(crate) user_ip: Option<HcaptchaUserIp>,
    pub(crate) site_key: Option<HcaptchaSiteKey>,
}

impl HcaptchaCaptcha {
    pub fn new(response: &str) -> Result<Self, HcaptchaError> {
        Ok(HcaptchaCaptcha {
            response: HcaptchaClientResponse::parse(response.to_owned())?,
            user_ip: None,
            site_key: None,
        })
    }
    pub fn set_user_ip(&mut self, user_ip: &str) -> Result<Self, HcaptchaError> {
        if user_ip.is_empty() {
            self.user_ip = None;
        } else {
            self.user_ip = Some(HcaptchaUserIp::parse(user_ip.to_owned())?);
        };

        Ok(self.clone())
    }

    pub fn set_site_key(&mut self, site_key: &str) -> Result<Self, HcaptchaError> {
        if site_key.is_empty() {
            self.site_key = None;
        } else {
            self.site_key = Some(HcaptchaSiteKey::parse(site_key.to_owned())?);
        };

        Ok(self.clone())
    }

    pub fn response(self) -> HcaptchaClientResponse {
        self.response
    }

    pub fn user_ip(&self) -> Option<HcaptchaUserIp> {
        self.user_ip.clone()
    }

    pub fn site_key(&self) -> Option<HcaptchaSiteKey> {
        self.site_key.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Code;
    use claim::{assert_err, assert_none, assert_ok, assert_some};
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::iter;

    fn random_response() -> String {
        let mut rng = thread_rng();
        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(100)
            .collect()
    }
    fn dummy_captcha() -> HcaptchaCaptcha {
        HcaptchaCaptcha::new(&random_response())
            .unwrap()
            .set_user_ip(&fakeit::internet::ipv4_address())
            .unwrap()
            .set_site_key(&fakeit::unique::uuid_v4())
            .unwrap()
    }
    #[test]
    fn response_cannot_be_empty_or_blank() {
        let empty = "";
        assert_err!(HcaptchaCaptcha::new(empty));
        let blank = "   ";
        assert_err!(HcaptchaCaptcha::new(blank));
    }

    #[test]
    fn fail_if_user_ip_not_valid_v4_or_v6_address() {
        let captcha = HcaptchaCaptcha::new("response_string")
            .unwrap()
            .set_user_ip(&fakeit::words::word());
        assert_err!(&captcha);
        if let Err(HcaptchaError::Codes(hs)) = captcha {
            assert!(hs.contains(&Code::InvalidUserIp));
        }
    }
    #[test]
    fn user_ip_is_optional() {
        let captcha = HcaptchaCaptcha::new("response_string")
            .unwrap()
            .set_user_ip(&fakeit::internet::ipv4_address())
            .unwrap();

        assert_some!(captcha.user_ip);
    }

    #[test]
    fn valid_user_id_is_accepted() {
        assert_ok!(HcaptchaCaptcha::new("response_string")
            .unwrap()
            .set_user_ip(&fakeit::internet::ipv4_address()));
    }

    #[test]
    fn fail_if_site_key_not_valid_uuid() {
        let captcha = HcaptchaCaptcha::new("response_string")
            .unwrap()
            .set_site_key(&fakeit::words::word());

        assert_err!(&captcha);
        if let Err(HcaptchaError::Codes(hs)) = captcha {
            assert!(hs.contains(&Code::InvalidSiteKey));
        }
    }
    #[test]
    fn site_key_is_optional() {
        let captcha = HcaptchaCaptcha::new("response_string")
            .unwrap()
            .set_site_key(&fakeit::unique::uuid_v4())
            .unwrap();

        assert_some!(captcha.site_key);
    }

    #[test]
    fn valid_site_key_is_accepted() {
        let captcha = HcaptchaCaptcha::new("response_string")
            .unwrap()
            .set_site_key(&fakeit::unique::uuid_v4())
            .unwrap();

        assert_some!(captcha.site_key());
    }

    #[test]
    fn update_site_key_with_empty_string_yields_none() {
        let mut captcha = dummy_captcha();

        assert_some!(captcha.site_key());
        captcha.set_site_key("").unwrap();

        assert_none!(captcha.site_key());
    }

    #[test]
    fn update_user_ip_with_empty_string_yields_none() {
        let mut captcha = dummy_captcha();

        assert_some!(captcha.user_ip());
        captcha.set_user_ip("").unwrap();

        assert_none!(captcha.user_ip());
    }
}
