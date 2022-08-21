mod error;
mod hcaptcha_validate;
mod param;
mod record;
mod send;

use hcaptcha::HcaptchaCaptcha;
use lambda_runtime::{Error, LambdaEvent};
use send::ContactForm;
use serde::{Deserialize, Serialize};
use tokio::join;
use tracing::{debug, error};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ApiGatewayEvent {
    body: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Recaptcha {
    #[serde(rename = "reCaptchaResponse")]
    re_captcha_response: String,
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct GatewayResponse {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: u16,
    body: String,
}

impl GatewayResponse {
    fn new(status_code: u16, body: String) -> GatewayResponse {
        GatewayResponse {
            is_base64_encoded: false,
            status_code,
            body,
        }
    }
}

#[tracing::instrument (name = "Handle submitted contact form", skip(event) fields(request_id = %event.context.request_id))]
pub async fn my_handler(event: LambdaEvent<ApiGatewayEvent>) -> Result<GatewayResponse, Error> {
    debug!("The event logged is: {:?}", event);
    let (e, c) = event.into_parts();
    let body_str = e.body.unwrap_or_else(|| "".to_owned());
    let captcha: HcaptchaCaptcha = serde_json::from_str(&body_str)?;

    hcaptcha_validate::response_valid(captcha).await?;

    let contact_form: ContactForm = serde_json::from_str(&body_str)?;

    tracing::info!(
        "Request {} is process for the contact {}.",
        c.request_id,
        contact_form.name
    );

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

    Ok(GatewayResponse::new(
        200,
        format!("{}, thank you for your contact request.", contact_form.name),
    ))
}
