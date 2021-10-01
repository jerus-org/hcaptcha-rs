use super::error::LambdaContactError;
use async_trait::async_trait;
use hcaptcha::Hcaptcha;
use hcaptcha::{HcaptchaCaptcha, HcaptchaClient, HcaptchaError, HcaptchaRequest, HcaptchaResponse};
use rusoto_ses::{SendEmailResponse, SendTemplatedEmailResponse};
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
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
    pub page: String,
    #[serde(default)]
    pub site: String,
    pub hcaptcha: String,
    pub remoteip: String,
    pub sitekey: String,
}

#[async_trait]
impl Hcaptcha for ContactForm {
    // impl ContactForm {
    async fn valid_response(&self, secret: &str) -> Result<HcaptchaResponse, HcaptchaError> {
        let client = HcaptchaClient::new();

        let captcha = HcaptchaCaptcha::new(&self.hcaptcha)?
            .set_remoteip(&self.remoteip)?
            .set_sitekey(&self.sitekey)?;

        let request = HcaptchaRequest::new(&secret, captcha)?;

        client.verify_client_response(request).await
    }
}

#[instrument(name = "send notification to info mailbox", skip(_contact_form))]
pub async fn notify_office(
    _contact_form: &ContactForm,
) -> Result<SendEmailResponse, LambdaContactError> {
    // Construct email and send message to the office info mailbox

    let res = SendEmailResponse {
        message_id: "generated_message_id".to_owned(),
    };

    Ok(res)
}

#[instrument(name = "Send notification to the contact", skip(_contact_form))]
pub async fn notify_contact(
    _contact_form: &ContactForm,
) -> Result<SendTemplatedEmailResponse, LambdaContactError> {
    // Construct and send email to the contact
    let res = SendTemplatedEmailResponse {
        message_id: "generated_message_id".to_owned(),
    };

    Ok(res)
}
