use crate::{Code, HcaptchaError};
use std::collections::HashSet;

#[derive(Debug, Default, serde::Serialize)]
pub struct HcaptchaSecret(String);

impl HcaptchaSecret {
    pub fn parse(s: String) -> Result<Self, HcaptchaError> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.len() != 22;
        let is_not_a_hex_string = !is_hex_string(&s);

        let mut codes = HashSet::new();

        if is_empty_or_whitespace {
            codes.insert(Code::MissingSecret);
        };
        if is_too_long || is_not_a_hex_string {
            codes.insert(Code::InvalidSecret);
        };

        if codes.is_empty() {
            Ok(HcaptchaSecret(s))
        } else {
            Err(HcaptchaError::Codes(codes))
        }
    }
}

fn is_hex_string(s: &str) -> bool {
    if s.len() != 22 {
        return false;
    };
    let start = &s[0..2];
    let number = &s[2..];
    let is_not_valid_start = start != "0x";
    let is_not_valid_number = number.parse::<usize>().is_err();
    !(is_not_valid_start || is_not_valid_number)
}

#[cfg(test)]
mod tests {
    use super::HcaptchaSecret;
    use crate::Code;
    use crate::HcaptchaError;
    use claim::assert_err;

    #[test]
    fn whitespace_only_secrets_are_rejected() {
        let secret = " ".to_string();
        assert_err!(HcaptchaSecret::parse(secret));
    }

    #[test]
    fn empty_string_is_rejected() {
        let secret = "".to_string();
        assert_err!(HcaptchaSecret::parse(secret));
    }

    #[test]
    fn secret_longer_than_22_bytes_is_rejected() {
        let secret = "12345678901234567890123".to_string();
        assert_err!(HcaptchaSecret::parse(secret));
    }

    #[test]
    fn secret_shorter_than_22_bytes_is_rejected() {
        let secret = "123456789012345678901".to_string();
        assert_err!(HcaptchaSecret::parse(secret));
    }
    #[test]
    fn secret_that_is_not_a_valid_hex_string_is_rejected() {
        let secret = "abcdefghijklmnopqrstuv".to_string();
        assert_err!(HcaptchaSecret::parse(secret));
    }

    #[test]
    fn error_set_contains_missing_secret_error() {
        let secret = "".to_string();
        if let Err(HcaptchaError::Codes(hs)) = HcaptchaSecret::parse(secret) {
            assert!(hs.contains(&Code::MissingSecret));
        }
    }

    #[test]
    fn error_set_contains_invalid_secret_error() {
        let secret = "abcdefghijklmnopqrstuv".to_string();
        if let Err(HcaptchaError::Codes(hs)) = HcaptchaSecret::parse(secret) {
            assert!(hs.contains(&Code::InvalidSecret));
        }
    }
}
