mod contact_form;
mod dynamo_items;
mod error;
mod param;
mod record;
mod send;

use contact_form::ContactForm;
use error::LambdaContactError;
use hcaptcha::Hcaptcha;
use lambda_runtime::Context;
use serde_derive::{Deserialize, Serialize};
use tokio::join;

const HCAPTCHA_SECRET: &str = "/website/hcaptcha/secret_key";

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ApiGatewayEvent {
    body: Option<String>,
}

impl ApiGatewayEvent {
    fn body_str(&self) -> &str {
        match &self.body {
            Some(s) => s,
            None => "",
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct ApiResponse {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: u16,
    body: String,
}

impl ApiResponse {
    fn new(status_code: u16, body: String) -> ApiResponse {
        ApiResponse {
            is_base64_encoded: false,
            status_code,
            body,
        }
    }
}

#[tracing::instrument(name = "API Gateway event handler", skip(e, c), fields(request_id = %c.request_id))]
pub async fn my_handler(e: ApiGatewayEvent, c: Context) -> Result<ApiResponse, LambdaContactError> {
    tracing::debug!("The event logged is: {:?}", e);

    tracing::debug!("About to validate the response code.");
    let contact_form: ContactForm = serde_json::from_str(e.body_str())?;
    tracing::info!(
        "Request {} is process for the contact {}.",
        c.request_id,
        contact_form.name
    );
    let secret = param::get_parameter(HCAPTCHA_SECRET, true).await?;

    let response = contact_form.valid_response(&secret, None).await?;

    tracing::debug!("Response code valid! Response: {:?}", response);

    if let Some(hostname) = response.hostname() {
        tracing::debug!("Response hostname: {:?}", hostname);
    };
    if let Some(credit) = response.credit() {
        tracing::debug!("Response credit: {:?}", credit);
    };
    if let Some(timestamp) = response.timestamp() {
        tracing::debug!("Response timestamp: {:?}", timestamp);
    };
    let success = response.success();
    tracing::debug!("Response success: {:?}", success);

    if success {
        let info_fut = send::notify(&contact_form);
        let notification_fut = send::acknowledge(&contact_form);
        let write_fut = record::write(&contact_form);

        let (info, notification, write) = join!(info_fut, notification_fut, write_fut);

        if let Err(e) = notification {
            tracing::error!("Notification not sent: {}", e);
            return Err(LambdaContactError::Processing("Notification not sent".into()));
        }

        if let Err(e) = info {
            tracing::error!("Info not sent to office: {}", e);
            return Err(LambdaContactError::Processing("Info not sent to office".into()));
        }

        if let Err(e) = write {
            tracing::error!("Contact information not written to database: {}", e);
        }

        Ok(ApiResponse::new(
            200,
            format!("{}, thank you for your contact request.", contact_form.name),
        ))
    } else {
        match response.error_codes() {
            Some(hs) => {
                let mut errors = String::new();
                for item in hs.iter() {
                    if errors.is_empty() {
                        errors = item.to_string();
                    } else {
                        errors = format!("{}, {}", errors, item);
                    }
                }
                Err(LambdaContactError::Processing(errors))
            }
            None => Err(LambdaContactError::Processing("no errors".to_string())),
        }
    }
}
