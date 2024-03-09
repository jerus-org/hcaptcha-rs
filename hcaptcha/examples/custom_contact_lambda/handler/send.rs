use super::error::ContactError;
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
}

#[instrument(name = "send notification to info mailbox", skip(_contact_form))]
pub async fn notify_office(_contact_form: &ContactForm) -> Result<(), ContactError> {
    // Construct email and send message to the office info mailbox

    Ok(())
}

#[instrument(name = "Send notification to the contact", skip(_contact_form))]
pub async fn notify_contact(_contact_form: &ContactForm) -> Result<(), ContactError> {
    // Construct and send email to the contact

    Ok(())
}
