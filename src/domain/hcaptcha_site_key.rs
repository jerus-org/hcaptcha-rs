use crate::{Code, HcaptchaError};
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct HcaptchaSiteKey(String);

impl fmt::Display for HcaptchaSiteKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl HcaptchaSiteKey {
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(name = "Validate Site Key.", skip(s), level = "debug")
    )]
    pub fn parse(s: String) -> Result<Self, HcaptchaError> {
        empty_site_key(&s)?;
        invalid_site_key(&s)?;

        Ok(HcaptchaSiteKey(s))
    }
}

#[cfg_attr(
    feature = "trace",
    tracing::instrument(name = "Return error on empty string.", skip(s), level = "debug")
)]
fn empty_site_key(s: &str) -> Result<(), HcaptchaError> {
    if s.trim().is_empty() {
        let mut codes = HashSet::new();
        codes.insert(Code::MissingSiteKey);

        #[cfg(feature = "trace")]
        tracing::debug!("{}", Code::MissingSiteKey);
        Err(HcaptchaError::Codes(codes))
    } else {
        Ok(())
    }
}

#[cfg_attr(
    feature = "trace",
    tracing::instrument(name = "Return error if not an ip string.", skip(s), level = "debug")
)]
fn invalid_site_key(s: &str) -> Result<(), HcaptchaError> {
    if Uuid::from_str(s).is_err() {
        let mut codes = HashSet::new();
        codes.insert(Code::InvalidSiteKey);

        #[cfg(feature = "trace")]
        tracing::debug!("{}", Code::InvalidSiteKey);
        Err(HcaptchaError::Codes(codes))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::HcaptchaSiteKey;
    use crate::Code;
    use crate::HcaptchaError;
    use claim::{assert_err, assert_ok};

    // const CYAN: &str = "\u{001b}[36m";
    // const RESET: &str = "\u{001b}[0m";

    #[test]
    fn whitespace_only_site_keys_are_rejected() {
        let site_key = " ".to_string();
        assert_err!(HcaptchaSiteKey::parse(site_key));
    }

    #[test]
    fn empty_string_is_rejected() {
        let site_key = "".to_string();
        assert_err!(HcaptchaSiteKey::parse(site_key));
    }

    #[test]
    fn error_set_contains_missing_site_key_error() {
        let site_key = "".to_string();
        if let Err(HcaptchaError::Codes(hs)) = HcaptchaSiteKey::parse(site_key) {
            assert!(hs.contains(&Code::MissingSiteKey));
        }
    }

    #[test]
    fn error_set_contains_invalid_site_key_error() {
        let site_key = "1922.20".to_string();
        let res = HcaptchaSiteKey::parse(site_key);
        assert_err!(&res);

        if let Err(HcaptchaError::Codes(hs)) = res {
            assert!(hs.contains(&Code::InvalidSiteKey));
        }
    }

    #[test]
    fn valid_site_key_key_is_valid() {
        let site_key = fakeit::unique::uuid_v4();

        assert_ok!(HcaptchaSiteKey::parse(site_key));
    }
}
