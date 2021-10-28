mod error;
mod param;
mod record;
mod send;

use hcaptcha::Hcaptcha;
use lambda_runtime::{Context, Error};
use serde::{Deserialize, Serialize};
use tokio::join;

const HCAPTCHA_SECRET: &str = "/hcaptcha/secret";

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ApiGatewayEvent {
    body: Option<String>,
}

impl ApiGatewayEvent {
    fn body_string(&self) -> &str {
        match &self.body {
            Some(s) => s,
            None => "",
        }
    }
}

#[derive(Debug, Deserialize, Hcaptcha)]
pub struct ContactForm {
    name: String,
    phone: String,
    email: String,
    message: String,
    #[captcha]
    token: String,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
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

#[tracing::instrument (name = "Handle submitted contact form", skip(e,c) fields(request_id = %c.request_id))]
pub async fn my_handler(e: ApiGatewayEvent, c: Context) -> Result<GatewayResponse, Error> {
    tracing::debug!("The event logged is: {:?}", e);

    let contact_form: ContactForm = serde_json::from_str(e.body_string())?;
    tracing::info!(
        "Request {} is process for the contact {}.",
        c.request_id,
        contact_form.name
    );
    let secret = param::get_parameter(HCAPTCHA_SECRET).await?;

    contact_form.valid_response(&secret, None).await?;

    let notify_office_fut = send::notify_office(&contact_form);
    let notify_contact_fut = send::notify_contact(&contact_form);
    let write_fut = record::write(&contact_form);

    let (notify_office, notify_contact, write) =
        join!(notify_office_fut, notify_contact_fut, write_fut);

    if let Err(e) = notify_contact {
        tracing::error!("Notification to the contact not sent: {}", e);
        return Err("Notification not sent".into());
    }

    if let Err(e) = notify_office {
        tracing::error!("Notification to the office not sent: {}", e);
        return Err("Info not sent to office".into());
    }

    if let Err(e) = write {
        tracing::error!("Contact information not written to database: {}", e);
    }

    Ok(GatewayResponse::new(
        200,
        format!("{}, thank you for your contact request.", contact_form.name),
    ))
}
