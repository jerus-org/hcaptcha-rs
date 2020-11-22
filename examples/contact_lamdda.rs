//! # Contact Lambda Example
//!
//! Backend lambda to process the data submitted from a contact form.
//! The form is protected by hcaptcha and the hcaptcha is verified before
//! further processsing of the form data. Once verified the form data is
//! saved and email to the website owner. An acknowledgement is emailed to
//! the submitter.
//!

use lambda_runtime::lambda;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let level = get_environment_level();
    println!("The module level will be {}", level);
    SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .with_module_level("handler", level)
        .init()?;

    lambda!(handler::my_handler);

    Ok(())
}

fn get_environment_level() -> LevelFilter {
    match std::env::var("LOGGING") {
        Ok(level) => match level.to_uppercase().as_str() {
            "OFF" => LevelFilter::Off,
            "ERROR" => LevelFilter::Error,
            "WARM" => LevelFilter::Warn,
            "INFO" => LevelFilter::Info,
            "DEBUG" => LevelFilter::Debug,
            "TRACE" => LevelFilter::Trace,
            _ => LevelFilter::Error,
        },
        Err(_e) => LevelFilter::Error,
    }
}

mod handler {
    use hcaptcha::Hcaptcha;
    use lambda_runtime::{error::HandlerError, Context};
    use rusoto_core::Region;
    use rusoto_ssm::{GetParameterRequest, Ssm, SsmClient};
    use serde_derive::{Deserialize, Serialize};

    /// Capture the event input from the body of the http request
    #[derive(Deserialize, Serialize, Clone, Debug, Default)]
    pub struct CustomEvent {
        body: Option<String>,
    }

    /// Provide the output for the request
    #[derive(Serialize, Clone, Debug, PartialEq)]
    pub struct CustomOutput {
        #[serde(rename = "isBase64Encoded")]
        is_base64_encoded: bool,
        #[serde(rename = "statusCode")]
        status_code: u16,
        body: String,
    }

    impl CustomOutput {
        fn new(status_code: u16, body: String) -> CustomOutput {
            CustomOutput {
                is_base64_encoded: false,
                status_code,
                body,
            }
        }
    }

    #[derive(Deserialize, Serialize, Clone, Default)]
    pub struct ContactForm {
        #[serde(default)]
        pub name: String,
        #[serde(default)]
        pub phone: String,
        #[serde(default)]
        pub email: String,
        #[serde(default)]
        pub message: String,
        #[serde(default)]
        #[serde(rename = "captchaResponse")]
        pub captcha_response: String,
    }

    use rusoto_ssm::GetParameterError;
    use thiserror::Error;

    #[derive(Debug, Error)]
    enum MyError {
        #[error("{0}")]
        SsmGetParameter(#[from] rusoto_core::RusotoError<GetParameterError>),
        // #[error("{0}")]
        // LambdaError(#[from] lambda_runtime::error::HandlerError),
        #[error("{0}")]
        Hcaptcha(#[from] hcaptcha::HcaptchaError),
    }

    impl ContactForm {
        async fn valid_captcha(&self) -> Result<(), MyError> {
            // secret key is stored encrypted in SSM Parameter store
            let name = "/hcaptcha/secret_key".to_owned();
            let decrypt = true;
            let parameter_request = GetParameterRequest {
                name,
                with_decryption: Some(decrypt),
            };

            let client = SsmClient::new(Region::EuWest1);

            let result = client.get_parameter(parameter_request).await?;

            let hcaptcha_secret;
            match result.parameter {
                Some(param) => match param.value {
                    Some(value) => hcaptcha_secret = value,
                    None => hcaptcha_secret = "".to_owned(),
                },
                None => hcaptcha_secret = "".to_owned(),
            }

            Hcaptcha::new(&hcaptcha_secret, &self.captcha_response)
                .verify()
                .await?;
            Ok(())
        }

        async fn save(&self) {
            todo!()
        }

        async fn send(&self) {
            todo!()
        }

        async fn acknowledge(&self) {
            todo!()
        }

        async fn check_errors(&self) -> Result<CustomOutput, MyError> {
            todo!()
        }
    }

    #[tokio::main]
    pub async fn my_handler(e: CustomEvent, _c: Context) -> Result<CustomOutput, HandlerError> {
        let body_str = e.body.unwrap_or_else(|| "".to_owned());

        let contact_form: ContactForm = serde_json::from_str(&body_str).unwrap_or_default();

        contact_form.valid_captcha().await?;

        let save = contact_form.save().await;
        let send = contact_form.send().await;
        let ack = contact_form.acknowledge().await;

        let result = contact_form.check_errors().await?;
        Ok(result)
    }
}
