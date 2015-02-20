#![feature(core, io, hash)]
extern crate "rustc-serialize" as rustc_serialize;
extern crate hyper;

mod error;
mod errorcode;
mod response;

pub use error::RecaptchaError;
pub use errorcode::RecaptchaErrorCode;

use std::collections::HashSet;
use rustc_serialize::json;
use response::RecaptchaResponse;

/// Verify a recaptcha user response
pub fn verify(key: &str, response: &str, user_ip: Option<&str>) -> Result<(), RecaptchaError> {
    use hyper::{Client, Url};

    let mut query = vec![
        ("secret", key),
        ("response", response),
    ];

    if let Some(user_ip) = user_ip {
        query.push(("remoteip", user_ip));
    }

    let mut url = Url::parse("https://www.google.com/recaptcha/api/siteverify").unwrap();

    url.set_query_from_pairs(query.into_iter());

    let mut client = Client::new();

    let mut response = try!(client.get(url).send());
    let body = try!(response.read_to_string());
    let recaptcha_response = try!(json::decode::<RecaptchaResponse>(&body));
    
    match (recaptcha_response.success, recaptcha_response.error_codes) {
        (true, _) => Ok(()),
        (false, Some(errors)) => Err(RecaptchaError::Codes(errors)),
        (false, _) => Err(RecaptchaError::Codes(HashSet::new()))
    }
}

#[test]
fn test_invalid_secret_missing_response() {
    use RecaptchaError::*;
    use RecaptchaErrorCode::*;
    let resp = verify("", "", None);

    match resp {
        Ok(()) => panic!("unexpected response: Ok(())"),
        Err(Wrapped(ref e)) => panic!("unexpected response: {}", e),
        Err(Codes(ref errors)) => {
            assert!(errors.contains(&InvalidSecret));
            assert!(errors.contains(&MissingResponse));
        }
    };

    println!("{:?}", resp);
}
