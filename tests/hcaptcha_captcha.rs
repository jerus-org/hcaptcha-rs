mod helpers;

use crate::Code;
use crate::HcaptchaSecret;
use claim::{assert_err, assert_none, assert_ok, assert_some};

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
        .set_user_ip(&fakeit::internet::ipv4_address().to_string())
        .unwrap();

    assert_some!(captcha.user_ip);
}

#[test]
fn valid_user_id_is_accepted() {
    assert_ok!(HcaptchaCaptcha::new("response_string")
        .unwrap()
        .set_user_ip(&fakeit::internet::ipv4_address().to_string()));
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
        .set_site_key(&fakeit::unique::uuid_v4().to_string())
        .unwrap();

    assert_some!(captcha.site_key);
}

#[test]
fn valid_site_key_is_accepted() {
    let captcha = HcaptchaCaptcha::new("response_string")
        .unwrap()
        .set_site_key(&fakeit::unique::uuid_v4().to_string())
        .unwrap();

    assert_some!(captcha.site_key);
}

#[test]
fn update_site_key_with_empty_string_yields_none() {
    let captcha = helpers::dummy_captcha();

    assert_some!(captcha.site_key());
    captcha.set_site_key("");

    assert_none!(captcha.site_key());
}
