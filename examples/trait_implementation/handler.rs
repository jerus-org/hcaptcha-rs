mod error;
mod param;
mod record;
mod send;

use hcaptcha::Hcaptcha;
use lambda_runtime::{Context, Error};
use send::ContactForm;
use serde::{Deserialize, Serialize};
use tokio::join;
use tracing::{debug, error};

const HCAPTCHA_SECRET: &str = "/hcaptcha/secret";

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct CustomEvent {
    body: Option<String>,
}

impl CustomEvent {
    fn body_string(&self) -> &str {
        match &self.body {
            Some(s) => &s,
            None => "",
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Recaptcha {
    #[serde(rename = "reCaptchaResponse")]
    re_captcha_response: String,
}

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

pub async fn my_handler(e: CustomEvent, _c: Context) -> Result<CustomOutput, Error> {
    debug!("The event logged is: {:?}", e);

    let contact_form: ContactForm = serde_json::from_str(e.body_string())?;
    let secret = param::get_parameter(HCAPTCHA_SECRET).await?;
    contact_form.valid_response(&secret).await?;

    let notify_office_fut = send::notify_office(&contact_form);
    let notify_contact_fut = send::notify_contact(&contact_form);
    let write_fut = record::write(&contact_form);

    let (notify_office, notify_contact, write) =
        join!(notify_office_fut, notify_contact_fut, write_fut);

    if let Err(e) = notify_contact {
        error!("Notification to the contact not sent: {}", e);
        return Err("Notification not sent".into());
    }

    if let Err(e) = notify_office {
        error!("Notification to the office not sent: {}", e);
        return Err("Info not sent to office".into());
    }

    if let Err(e) = write {
        error!("Contact information not written to database: {}", e);
    }

    Ok(CustomOutput::new(
        200,
        format!("{}, thank you for your contact request.", contact_form.name),
    ))
}
