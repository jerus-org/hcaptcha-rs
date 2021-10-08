mod dynamo_items;
mod error;
mod record;
mod send;
mod validate;
mod contact_form;

use lambda_runtime::{Context, Error};
use send::ContactForm;
use serde_derive::{Deserialize, Serialize};
use tokio::join;
use tracing::{debug, error, info, instrument};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ApiGatewayEvent {
    body: Option<String>,
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

#[instrument(name = "API Gateway event handler", skip(e, _c), fields(request_id = %c.request_id))]
pub async fn my_handler(e: ApiGatewayEvent, c: Context) -> Result<ApiResponse, Error> {
    debug!("The event logged is: {:?}", e);

    let body_str = e.body.unwrap_or_else(|| "".to_owned());
    info!("About to validate the response code.");
    

    let response = validate::valid_captcha(&body_str).await?;
    info!("Response code valid! Response: {:?}", response);

    if let Some(hostname) = response.hostname() {
        info!("Response hostname: {:?}", hostname);
    };
    if let Some(credit) = response.credit() {
        info!("Response credit: {:?}", credit);
    };
    if let Some(timestamp) = response.timestamp() {
        info!("Response timestamp: {:?}", timestamp);
    };
    let success = response.success();
    info!("Response success: {:?}", success);

    let contact_form: ContactForm = serde_json::from_str(&body_str)?;

    let info_fut = send::notify(&contact_form);
    let notification_fut = send::acknowledge(&contact_form);
    let write_fut = record::write(&contact_form);

    let (info, notification, write) = join!(info_fut, notification_fut, write_fut);

    if let Err(e) = notification {
        error!("Notification not sent: {}", e);
        return Err("Notification not sent".into());
    }

    if let Err(e) = info {
        error!("Info not sent to office: {}", e);
        return Err("Info not sent to office".into());
    }

    if let Err(e) = write {
        error!("Contact information not written to database: {}", e);
    }

    Ok(ApiResponse::new(
        200,
        format!("{}, thank you for your contact request.", contact_form.name),
    ))
}
