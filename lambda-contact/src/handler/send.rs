use super::error::LambdaContactError;
use rusoto_core::Region;
use rusoto_ses::{
    Body, Content, Destination, Message, SendEmailRequest, SendEmailResponse,
    SendTemplatedEmailRequest, SendTemplatedEmailResponse, Ses, SesClient,
};
use serde_derive::{Deserialize, Serialize};
use tracing::{debug, instrument};

#[instrument(name = "notify contact details", skip(contact_form))]
pub async fn notify(contact_form: &ContactForm) -> Result<SendEmailResponse, LambdaContactError> {
    const RECIEVER: &str = "info@jerus.ie";
    const SENDER: &str = "info@jerus.ie";
    const CONFIG_SET: &str = "website";

    let client = SesClient::new(Region::EuWest1);

    let destinations = vec![RECIEVER.to_string()];

    let send_to = Destination {
        bcc_addresses: None,
        cc_addresses: None,
        to_addresses: Some(destinations),
    };
    let message = create_message(&contact_form);
    debug!("The prepared message is: {:#?}", message);
    let send_mail_request = SendEmailRequest {
        configuration_set_name: Some(CONFIG_SET.to_string()),
        destination: send_to,
        message,
        source: SENDER.to_string(),
        ..Default::default()
    };
    debug!("The request: {:#?}", send_mail_request);
    let res = client.send_email(send_mail_request).await?;
    debug!("Successful result?: {:#?}", res);

    Ok(res)
}

#[derive(Deserialize, Serialize, Clone, Default)]
struct NotificationTemplate {
    name: String,
}

impl From<&ContactForm> for NotificationTemplate {
    fn from(form: &ContactForm) -> NotificationTemplate {
        NotificationTemplate {
            name: form.name.clone(),
        }
    }
}

#[instrument(name = "Send acknowledgement to contact", skip(contact_form))]
pub async fn acknowledge(
    contact_form: &ContactForm,
) -> Result<SendTemplatedEmailResponse, LambdaContactError> {
    const SENDER: &str = "info@jerus.ie";
    const CONFIG_SET: &str = "website";
    const TEMPLATE: &str = "contact-response";
    let client = SesClient::new(Region::EuWest1);
    let destinations = vec![contact_form.email.clone()];

    let send_to = Destination {
        bcc_addresses: None,
        cc_addresses: None,
        to_addresses: Some(destinations),
    };
    let template_data: NotificationTemplate = contact_form.into();
    let template_data = serde_json::to_string(&template_data).unwrap_or_default();
    let send_templated_mail_request = SendTemplatedEmailRequest {
        configuration_set_name: Some(CONFIG_SET.to_string()),
        destination: send_to,
        template: TEMPLATE.to_string(),
        template_data,
        source: SENDER.to_string(),
        ..Default::default()
    };

    debug!("Request: {:#?}", send_templated_mail_request);
    let res = client
        .send_templated_email(send_templated_mail_request)
        .await?;
    debug!("Successful result?: {:#?}", res);
    Ok(res)
}

#[instrument(name = "create the message", skip(contact_form))]
fn create_message(contact_form: &ContactForm) -> Message {
    let body_text = format!(
        "Name:   {}\nEmail:  {}\nPhone:  {}\nMessage\n{}",
        contact_form.name, contact_form.email, contact_form.phone, contact_form.message,
    );

    let html_body_text = format!(
        "<html>
            <head></head>
            <body>
                <h2>Contact Details</h2>
                <p><b>Name:</b>   {}</p>
                <p><b>Email:</b>  {}</p>
                <p><b>Phone:</b>  {}</p>
                <h2>Message</h2>
                <p>{}</p>
            </body>
        </html>",
        contact_form.name, contact_form.email, contact_form.phone, contact_form.message,
    );

    Message {
        body: Body {
            html: Some(Content {
                charset: None,
                data: html_body_text,
            }),
            text: Some(Content {
                charset: None,
                data: body_text,
            }),
        },
        subject: Content {
            charset: None,
            data: format!(
                "Message from \"{}\" page on {}",
                contact_form.page_name(),
                contact_form.site_name(),
            ),
        },
    }
}
