const VERIFY_URL: &str = "https://hcaptcha.com/siteverify";
pub mod error;
mod response;

use log::debug;
use reqwest::{Client, Url};
use response::HcaptchaResponse;
use serde_derive::Serialize;
use std::collections::HashSet;
use std::net::IpAddr;

pub use error::Error;

#[derive(Debug, Default, Serialize)]
pub struct HcaptchaRequest {
    secret: String,
    token: String,
    user_ip: Option<String>,
    site_key: Option<String>,
}

impl HcaptchaRequest {
    /// Create a new HcaptchaRequest
    #[allow(dead_code)]
    pub fn new(secret: &str, token: &str) -> HcaptchaRequest {
        HcaptchaRequest {
            secret: secret.to_owned(),
            token: token.to_owned(),
            ..HcaptchaRequest::default()
        }
    }

    /// Specify the optional ip address value
    #[allow(dead_code)]
    pub fn set_user_ip(mut self, user_ip: &IpAddr) -> HcaptchaRequest {
        self.user_ip = Some(user_ip.to_string());
        self
    }

    /// Specify the optional site key value
    #[allow(dead_code)]
    pub fn set_site_key(mut self, site_key: &str) -> HcaptchaRequest {
        self.site_key = Some(site_key.to_owned());
        self
    }

    /// Verify a hcaptcha user response
    ///
    /// # Example
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::Error> {
    /// # use hcaptcha::HcaptchaRequest;
    /// # use hcaptcha::error::Code::*;
    /// # use std::net::{IpAddr, Ipv4Addr};
    ///
    /// let secret = "0x0000000000000000000000000000000000000000";
    /// let token = "";
    /// let user_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 17));
    /// let site_key = "10000000-ffff-ffff-ffff-000000000001";
    ///
    /// let response = HcaptchaRequest::new(secret, token)
    ///                 .set_user_ip(&user_ip)
    ///                 .set_site_key(&site_key)
    ///                 .verify().await;
    ///
    /// assert!(response.is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn verify(self) -> Result<(), Error> {
        let url = Url::parse(VERIFY_URL).unwrap();

        let body = serde_json::to_string(&self)?;
        debug!("Url {} and body {}", url, body);

        let response = Client::new().post(url).body(body).send().await?;
        let response = response.json::<HcaptchaResponse>().await?;
        println!("The response is: {:?}", response);
        match (response.success, response.error_codes) {
            (true, _) => Ok(()),
            (false, Some(errors)) => Err(Error::Codes(errors)),
            (false, _) => Err(Error::Codes(HashSet::new())),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_secret_missing_response() {
        use error::Code::*;
        use error::Error::*;
        let response = HcaptchaRequest::new("", "").verify().await;

        match response {
            Ok(()) => panic!("unexpected response: Ok(())"),
            Err(Codes(ref errors)) => {
                println!("Errors found {:?}", errors);
                assert!(errors.contains(&MissingSecret));
                assert!(errors.contains(&MissingResponse));
            }
            Err(e) => panic!("unexpected error: {}", e),
        };

        println!("{:?}", response);
    }

    #[tokio::test]
    async fn test_invalid_secret_missing_response_with_ip() {
        use error::Code::*;
        use error::Error::*;
        use std::net::Ipv4Addr;

        let user_ip = IpAddr::V4(Ipv4Addr::new(18, 197, 23, 227));

        let response = HcaptchaRequest::new("", "")
            .set_user_ip(&user_ip)
            .verify()
            .await;

        match response {
            Ok(()) => panic!("unexpected response: Ok(())"),
            Err(Codes(ref errors)) => {
                assert!(errors.contains(&MissingSecret));
                assert!(errors.contains(&MissingResponse));
            }
            Err(e) => panic!("unexpected error: {}", e),
        };

        println!("{:?}", response);
    }

    #[tokio::test]
    async fn test_invalid_secret_missing_response_with_site_key() {
        use error::Code::*;
        use error::Error::*;
        let response = HcaptchaRequest::new("", "")
            .set_site_key("10000000-ffff-ffff-ffff-000000000001")
            .verify()
            .await;

        match response {
            Ok(()) => panic!("unexpected response: Ok(())"),
            Err(Codes(ref errors)) => {
                assert!(errors.contains(&MissingSecret));
                assert!(errors.contains(&MissingResponse));
            }
            Err(e) => panic!("unexpected error: {}", e),
        };

        println!("{:?}", response);
    }
}
