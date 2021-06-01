use crate::{Code, HcaptchaError};
use std::collections::HashSet;

#[derive(Debug, Default, serde::Serialize)]
pub struct HcaptchaSecret(String);

impl HcaptchaSecret {
    pub fn parse(s: String) -> Result<Self, HcaptchaError> {
        if s.trim().is_empty() {
            let mut codes = HashSet::new();
            codes.insert(Code::MissingSecret);

            Err(HcaptchaError::Codes(codes))
        } else {
            Ok(HcaptchaSecret(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HcaptchaSecret;
    use crate::Code;
    use crate::HcaptchaError;
    use claim::assert_err;

    #[test]
    fn whitespace_only_names_are_rejected() {
        let secret = " ".to_string();
        assert_err!(HcaptchaSecret::parse(secret));
    }

    #[test]
    fn empty_string_is_rejected() {
        let secret = "".to_string();
        assert_err!(HcaptchaSecret::parse(secret));
    }

    #[test]
    fn error_set_contains_missing_secret_error() {
        let secret = "".to_string();

        if let Err(HcaptchaError::Codes(hs)) = HcaptchaSecret::parse(secret) {
            assert!(hs.contains(&Code::MissingSecret));
        }
    }
}
