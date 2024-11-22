use crate::{Code, Error};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ClientResponse(String);

impl ClientResponse {
    pub fn parse(s: String) -> Result<ClientResponse, Error> {
        if s.trim().is_empty() {
            let mut codes = HashSet::new();
            codes.insert(Code::MissingResponse);

            Err(Error::Codes(codes))
        } else {
            Ok(ClientResponse(s))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ClientResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::ClientResponse;
    use crate::Code;
    use crate::Error;
    use claims::assert_err;

    #[test]
    fn whitespace_only_names_are_rejected() {
        let response = " ".to_string();
        assert_err!(ClientResponse::parse(response));
    }

    #[test]
    fn empty_string_is_rejected() {
        let response = "".to_string();
        assert_err!(ClientResponse::parse(response));
    }

    #[test]
    fn error_set_contains_missing_response_error() {
        let response = "".to_string();

        if let Err(Error::Codes(hs)) = ClientResponse::parse(response) {
            assert!(hs.contains(&Code::MissingResponse));
        }
    }

    #[test]
    fn test_as_str() {
        let response = ClientResponse("test_response".to_string());
        assert_eq!(response.as_str(), "test_response");
    }
}
