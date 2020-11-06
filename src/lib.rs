pub mod error;
mod request;
mod response;

use request::HcaptchaRequest;
use response::HcaptchaResponse;
use std::collections::HashSet;
use std::net::IpAddr;

pub use error::Error;

#[derive(Debug, Default)]
pub struct Hcaptcha {
    request: HcaptchaRequest,
    response: HcaptchaResponse,
}

impl Hcaptcha {
    /// Create a new Hcaptcha Request
    ///
    /// # Example
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() {
    /// # use hcaptcha::Hcaptcha;
    ///
    /// let secret = ""; // your secret key
    /// let token = "";  // user's token
    ///
    /// let hcaptcha = Hcaptcha::new(secret, token)
    ///                 .verify().await;
    ///
    /// assert!(hcaptcha.is_err());
    ///
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn new(secret: &str, token: &str) -> Hcaptcha {
        let request = HcaptchaRequest::new(secret, token);

        Hcaptcha {
            request,
            ..Hcaptcha::default()
        }
    }

    /// Specify the optional ip address value
    ///
    /// # Example
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() {
    /// # use hcaptcha::Hcaptcha;
    /// # use std::net::{IpAddr, Ipv4Addr};
    ///
    /// let secret = ""; // your secret key
    /// let token = "";  // user's token
    /// let user_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 17));
    ///
    /// let hcaptcha = Hcaptcha::new(secret, token)
    ///                 .set_user_ip(&user_ip)  
    ///                 .verify().await;
    ///
    /// assert!(hcaptcha.is_err());
    ///
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn set_user_ip(mut self, user_ip: &IpAddr) -> Hcaptcha {
        self.request.set_user_ip(user_ip);
        self
    }

    /// Specify the optional site key value
    ///
    /// # Example
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() {
    /// # use hcaptcha::Hcaptcha;
    ///
    /// let secret = ""; // your secret key
    /// let token = "";  // user's token
    /// let site_key = "10000000-ffff-ffff-ffff-000000000001";
    ///
    /// let hcaptcha = Hcaptcha::new(secret, token)
    ///                 .set_site_key(site_key)  
    ///                 .verify().await;
    ///
    /// assert!(hcaptcha.is_err());
    ///
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn set_site_key(mut self, site_key: &str) -> Hcaptcha {
        self.request.set_site_key(site_key);
        self
    }

    /// Verify a hcaptcha user response
    ///
    /// # Example
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), hcaptcha::Error> {
    /// # use hcaptcha::Hcaptcha;
    /// # use hcaptcha::error::Code::*;
    /// # use std::net::{IpAddr, Ipv4Addr};
    ///
    /// let secret = "0x0000000000000000000000000000000000000000";
    /// let token = "";
    /// let user_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 17));
    /// let site_key = "10000000-ffff-ffff-ffff-000000000001";
    ///
    /// let response = Hcaptcha::new(secret, token)
    ///                 .set_user_ip(&user_ip)
    ///                 .set_site_key(&site_key)
    ///                 .verify().await;
    ///
    /// assert!(response.is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn verify(&mut self) -> Result<(), Error> {
        self.response = self.request.verify().await?;

        match (self.response.success, self.response.error_codes.clone()) {
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
        let response = Hcaptcha::new("", "").verify().await;

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

        let response = Hcaptcha::new("", "").set_user_ip(&user_ip).verify().await;

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
        let response = Hcaptcha::new("", "")
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
