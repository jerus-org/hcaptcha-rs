use crate::{Code, Error};
use std::collections::HashSet;
use std::fmt;

const SECRET_LEN_V1: usize = 42;
const SECRET_LEN_V2: usize = 35;

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
        tracing::instrument(name = "Extended check of secret.", skip(s), level = "debug")
    )]

    pub fn parse(s: String) -> Result<Self, Error> {
        match SecretVersions::parse(s)? {
            SecretVersions::V1(s) => Secret::parse_v1(s),
            SecretVersions::V2(s) => Secret::parse_v2(s),
        }
    }

    pub fn parse_v1(s: String) -> Result<Self, Error> {
        let is_wrong_length = s.len() != SECRET_LEN_V1;
        let is_not_a_hex_string = !is_hex_string(&s);
        let mut codes = HashSet::new();
        if is_wrong_length {
            codes.insert(Code::InvalidSecretExtWrongLen);
        };
        if is_not_a_hex_string {
            codes.insert(Code::InvalidSecretExtNotHex);
        };
        if codes.is_empty() {
            Ok(Secret(s))
        } else {
            #[cfg(feature = "trace")]
            tracing::debug!("Extended check found errors in secret string: {:?}", &codes);
            Err(Error::Codes(codes))
        }
    }

    pub fn parse_v2(s: String) -> Result<Self, Error> {
        let is_wrong_length = s.len() != SECRET_LEN_V2;
        let hex_portion = s.replace("ES_", "0x");
        let is_not_a_hex_string = !is_hex_string(&hex_portion);
        let mut codes = HashSet::new();
        if is_wrong_length {
            codes.insert(Code::InvalidSecretExtWrongLen);
        };
        if is_not_a_hex_string {
            codes.insert(Code::InvalidSecretExtNotHex);
        };
        if codes.is_empty() {
            Ok(Secret(s))
        } else {
            #[cfg(feature = "trace")]
            tracing::debug!("Extended check found errors in secret string: {:?}", &codes);
            Err(Error::Codes(codes))
        }
    }
}

#[cfg_attr(
    feature = "trace",
    tracing::instrument(name = "Check for hex string.", skip(s), level = "debug")
)]
fn is_hex_string(s: &str) -> bool {
    let start_is_valid = &s[0..2] == "0x";
    let string_is_valid = hex::decode(s.trim_start_matches("0x")).is_ok();

    start_is_valid && string_is_valid
}

#[derive(Debug)]
enum SecretVersions {
    V1(String),
    V2(String),
}

impl SecretVersions {
    pub fn parse(s: String) -> Result<Self, Error> {
        let mut codes = HashSet::new();
        let is_empty_or_whitespace = s.trim().is_empty();
        if is_empty_or_whitespace {
            codes.insert(Code::MissingSecret);
            #[cfg(feature = "trace")]
            tracing::debug!("Extended check found errors in secret string: {:?}", &codes);
            return Err(Error::Codes(codes));
        }
        let start = &s[0..2];
        match start {
            "0x" => Ok(SecretVersions::V1(s)),
            "ES" => Ok(SecretVersions::V2(s)),
            _ => {
                codes.insert(Code::SecretVersionUnknown);
                #[cfg(feature = "trace")]
                tracing::debug!("Extended check found errors in secret string: {:?}", &codes);
                Err(Error::Codes(codes))
            }
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

    #[cfg(feature = "ext")]
    #[test]
    fn secret_that_is_not_a_valid_hex_string_is_rejected() {
        let secret = "abcdefghijklmnopqrstuv".to_string();
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
    fn error_set_contains_invalid_secret_error() {
        let secret = "0xcdefghijklmnopqrstuvxyzabcdefghijk".to_string();
        if let Err(Error::Codes(hs)) = Secret::parse(secret) {
            assert!(hs.contains(&Code::InvalidSecretExtNotHex));
            assert!(hs.contains(&Code::InvalidSecretExtWrongLen));
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
        let secret = "ES_215963ca0f4b4d5e80d2ae736ce35d1d".to_string();
        assert_ok!(Secret::parse(secret));
    }

    #[test]
    fn test_parse_v2_wrong_length() {
        let s = "ES_12345678901234567890123456789012345"; // incorrect length
        let result = Secret::parse_v2(s.to_string());
        assert!(result.is_err());
        if let Err(Error::Codes(codes)) = result {
            assert!(codes.contains(&Code::InvalidSecretExtWrongLen));
        }
    }
}
