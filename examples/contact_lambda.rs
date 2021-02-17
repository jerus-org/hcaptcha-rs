//! # Contact Lambda Example
//!
//! Backend lambda to process the data submitted from a contact form.
//! The form is protected by hcaptcha and the hcaptcha is verified before
//! further processsing of the form data.
//! Once verified the form data is sent to the website owner, a noticication
//! is sent to the submitter and a record of the form saved. The three tasks
//! are completed concurrently by using async functions.
//!

use lambda_runtime::lambda;
#[cfg(feature = "logging")]
use log::LevelFilter;
#[cfg(feature = "logging")]
use simple_logger::SimpleLogger;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "logging")]
    let level = get_environment_level();
    #[cfg(feature = "logging")]
    println!("The module level will be {}", level);
    #[cfg(feature = "logging")]
    SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .with_module_level("handler", level)
        .init()?;

    lambda!(handler::my_handler);

    Ok(())
}
#[cfg(feature = "logging")]
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
    #[cfg(feature = "logging")]
    use log::{debug, error};
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
        pub name: String,
        pub phone: String,
        pub email: String,
        pub message: String,
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

    impl From<MyError> for HandlerError {
        fn from(err: MyError) -> HandlerError {
            HandlerError::from(err.to_string().as_str())
        }
    }

    #[tokio::main]
    pub async fn my_handler(e: CustomEvent, _c: Context) -> Result<CustomOutput, HandlerError> {
        #[cfg(feature = "logging")]
        debug!("The event logged is: {:?}", e);

        let body_str = e.body.unwrap_or_else(|| "".to_owned());

        let contact_form: ContactForm = serde_json::from_str(&body_str)?;

        // Validate that the formm is submitted from a human
        valid_captcha(&contact_form.captcha_response).await?;

        // Create async futures work with the contact form data
        let info_fut = info(&contact_form);
        let notification_fut = notification(&contact_form);
        let write_fut = write(&contact_form);

        // Execute the futures concurrently
        let (info, notification, write) = join!(info_fut, notification_fut, write_fut);

        #[allow(unused_variables)]
        if let Err(e) = notification {
            // Log the error and return error for rework at the client
            #[cfg(feature = "logging")]
            error!("Notification not sent: {}", e);
            return Err("Notifcation not sent".into());
        }
        #[allow(unused_variables)]
        if let Err(e) = info {
            // Log the error and return error for rework at the client
            #[cfg(feature = "logging")]
            error!("Info not sent to office: {}", e);
            return Err("Info not sent to office".into());
        }
        #[allow(unused_variables)]
        if let Err(e) = write {
            // Log the error but client will expect success
            #[cfg(feature = "logging")]
            error!("Contact information not written to database: {}", e);
        }

        Ok(CustomOutput::new(
            200,
            format!("{}, thank you for your contact request.", contact_form.name),
        ))
    }

    use tokio::join;

    async fn valid_captcha(captcha_response: &str) -> Result<(), MyError> {
        let name = "/hcaptcha/secret_key".to_owned();
        let parameter_request = GetParameterRequest {
            name,
            with_decryption: Some(true),
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

        Hcaptcha::new(&hcaptcha_secret, captcha_response)
            .verify()
            .await?;
        Ok(())
    }

    async fn info(_contact_form: &ContactForm) -> Result<(), MyError> {
        // Implement SES code to send email to the company info address
        todo!()
    }

    async fn notification(_contact_form: &ContactForm) -> Result<(), MyError> {
        // Implement SES code to send email to the customer
        todo!()
    }

    async fn write(_contact_form: &ContactForm) -> Result<(), MyError> {
        // Implement code to record contact
        todo!()
    }
}
