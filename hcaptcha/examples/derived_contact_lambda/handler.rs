// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod error;
mod param;
mod record;
mod send;

use hcaptcha::Hcaptcha;
use lambda_runtime::{Error, LambdaEvent};
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

/// This struct represents the payload provided by the client
/// in the body of the event.
///
/// Hcaptcha derive will implement valid_reponse use the field
/// identified with #[captcha] as the field containing the token
/// from the client.
#[derive(Debug, Deserialize, Hcaptcha)]
pub struct ContactForm {
    name: String,
    #[allow(dead_code)]
    phone: String,
    email: String,
    #[allow(dead_code)]
    message: String,
    #[captcha]
    token: String,
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
    tracing::debug!("The event logged is: {:?}", event);
    let (e, c) = event.into_parts();

    // Get the contact form (payload) from the body string in the event
    // The payload includes the hcapthca response
    let contact_form: ContactForm = serde_json::from_str(e.body_string())?;
    tracing::info!(
        "Request {} is process for the contact {}.",
        c.request_id,
        contact_form.name
    );

    // Get the secret string required to validate with hcaptcha.com
    let secret = param::get_parameter(HCAPTCHA_SECRET).await?;

    // Validate the hcaptcha response before processing the form.
    contact_form.valid_response(&secret, None).await?;

    let notify_office_fut = send::notify_office(&contact_form);
    let notify_contact_fut = send::notify_contact(&contact_form);
    let write_fut = record::write(&contact_form);

    let (notify_office, notify_contact, write) =
        join!(notify_office_fut, notify_contact_fut, write_fut);

    // This is an error condition that needs to be handled by the client
    if let Err(e) = notify_contact {
        tracing::error!("Notification to the contact not sent: {}", e);
        return Err("Notification not sent".into());
    }

    // This is an error condition that needs to be handled by the client
    if let Err(e) = notify_office {
        tracing::error!("Notification to the office not sent: {}", e);
        return Err("Info not sent to office".into());
    }

    // This is an error condition that is logged but does not need to be handled by the client
    // The error is only logged because:
    //   1. the customer has been notified
    //   2. the customers details are captured in the office notification
    if let Err(e) = write {
        tracing::error!("Contact information not written to database: {}", e);
    }

    Ok(GatewayResponse::new(
        200,
        format!("{}, thank you for your contact request.", contact_form.name),
    ))
}
