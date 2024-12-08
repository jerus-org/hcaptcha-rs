use crate::{Code, Error};
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Sitekey(String);

impl fmt::Display for Sitekey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sitekey {
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(name = "Validate Site Key.", skip(s), level = "debug")
    )]
    pub fn parse(s: String) -> Result<Self, Error> {
        empty_sitekey(&s)?;
        invalid_sitekey(&s)?;

        Ok(Sitekey(s))
    }
}

#[cfg_attr(
    feature = "trace",
    tracing::instrument(name = "Return error on empty string.", skip(s), level = "debug")
)]
fn empty_sitekey(s: &str) -> Result<(), Error> {
    if s.trim().is_empty() {
        let mut codes = HashSet::new();
        codes.insert(Code::MissingSiteKey);

        #[cfg(feature = "trace")]
        tracing::debug!("{}", Code::MissingSiteKey);
        Err(Error::Codes(codes))
    } else {
        Ok(())
    }
}

#[cfg_attr(
    feature = "trace",
    tracing::instrument(name = "Return error if not an ip string.", skip(s), level = "debug")
)]
fn invalid_sitekey(s: &str) -> Result<(), Error> {
    if Uuid::from_str(s).is_err() {
        let mut codes = HashSet::new();
        codes.insert(Code::InvalidSiteKey);

        #[cfg(feature = "trace")]
        tracing::debug!("{}", Code::InvalidSiteKey);
        Err(Error::Codes(codes))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Sitekey;
    use crate::Code;
    use crate::Error;
    use claims::{assert_err, assert_ok};

    // const CYAN: &str = "\u{001b}[36m";
    // const RESET: &str = "\u{001b}[0m";

    #[test]
    fn whitespace_only_sitekeys_are_rejected() {
        let sitekey = " ".to_string();
        assert_err!(Sitekey::parse(sitekey));
    }

    #[test]
    fn empty_string_is_rejected() {
        let sitekey = "".to_string();
        assert_err!(Sitekey::parse(sitekey));
    }

    #[test]
    fn error_set_contains_missing_sitekey_error() {
        let sitekey = "".to_string();
        if let Err(Error::Codes(hs)) = Sitekey::parse(sitekey) {
            assert!(hs.contains(&Code::MissingSiteKey));
        }
    }

    #[test]
    fn error_set_contains_invalid_sitekey_error() {
        let sitekey = "1922.20".to_string();
        let res = Sitekey::parse(sitekey);
        assert_err!(&res);

        if let Err(Error::Codes(hs)) = res {
            assert!(hs.contains(&Code::InvalidSiteKey));
        }
    }

    #[test]
    fn valid_sitekey_key_is_valid() {
        let sitekey = mockd::unique::uuid_v4();

        assert_ok!(Sitekey::parse(sitekey));
    }
}
