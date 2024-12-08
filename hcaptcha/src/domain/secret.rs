use crate::{Code, Error};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct Secret(String);

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Secret {
    #[cfg_attr(
        feature = "trace",
        tracing::instrument(name = "Simple check of secret.", skip(s), level = "debug")
    )]
    pub fn parse(s: String) -> Result<Self, Error> {
        if s.trim().is_empty() {
            let mut codes = HashSet::new();
            codes.insert(Code::MissingSecret);

            #[cfg(feature = "trace")]
            tracing::debug!("Secret string is missing");
            Err(Error::Codes(codes))
        } else {
            Ok(Secret(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Secret;
    use crate::Code;
    use crate::Error;
    use claims::{assert_err, assert_ok};

    #[test]
    fn whitespace_only_secrets_are_rejected() {
        let secret = " ".to_string();
        assert_err!(Secret::parse(secret));
    }
    #[test]
    fn empty_string_is_rejected() {
        let secret = "".to_string();
        assert_err!(Secret::parse(secret));
    }
    #[cfg(feature = "ext")]
    #[test]
    fn secret_longer_than_secret_len_is_rejected() {
        let secret = "1234567890123456789012345678901234567890123".to_string();
        assert_err!(Secret::parse(secret));
    }

    #[cfg(feature = "ext")]
    #[test]
    fn secret_shorter_than_secret_len_is_rejected() {
        let secret = "12345678901234567890123456789012345678901".to_string();
        assert_err!(Secret::parse(secret));
    }

    #[test]
    fn error_set_contains_missing_secret_error() {
        let secret = "".to_string();
        if let Err(Error::Codes(hs)) = Secret::parse(secret) {
            assert!(hs.contains(&Code::MissingSecret));
        }
    }

    #[test]
    fn test_v1_secret_key_is_valid() {
        let secret = "0x0000000123456789abcdefABCDEF000000000000".to_string();
        assert_ok!(Secret::parse(secret));
    }

    // A second format of secret is being issued since September 2023
    #[test]
    fn test_v2_secret_key_is_valid() {
        let secret = "ES_a5e0b5406e2b4c939ce48946389463894638473b1c".to_string();
        assert_ok!(Secret::parse(secret));
    }
}
