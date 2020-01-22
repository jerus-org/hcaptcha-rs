extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;

pub mod error;
mod response;

use std::collections::HashSet;
use std::net::IpAddr;

use reqwest::Url;

use response::RecaptchaResponse;

pub use error::Error;


/// Verify a recaptcha user response
pub async fn verify(key: &str, response: &str, user_ip: Option<&IpAddr>) -> Result<(), Error> {
    let user_ip = user_ip.map(ToString::to_string);

    let mut url = Url::parse("https://www.google.com/recaptcha/api/siteverify").unwrap();

    url.query_pairs_mut().extend_pairs(&[
        ("secret", key),
        ("response", response),
    ]);

    if let Some(user_ip) = user_ip {
        url.query_pairs_mut().append_pair("remoteip", &user_ip);
    }

    let response = reqwest::get(url).await?;
    let recaptcha_response = response.json::<RecaptchaResponse>().await?;
    
    match (recaptcha_response.success, recaptcha_response.error_codes) {
        (true, _) => Ok(()),
        (false, Some(errors)) => Err(Error::Codes(errors)),
        (false, _) => Err(Error::Codes(HashSet::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_secret_missing_response() {
        use error::Error::*;
        use error::Code::*;
        let response = verify("", "", None).await;

        match response {
            Ok(()) => panic!("unexpected response: Ok(())"),
            Err(Codes(ref errors)) => {
                assert!(errors.contains(&InvalidSecret));
                assert!(errors.contains(&MissingResponse));
            }
            Err(e) => panic!("unexpected error: {}", e),
        };

        println!("{:?}", response);
    }
}
