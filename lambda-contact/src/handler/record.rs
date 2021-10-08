use crate::handler::contact_form::ContactForm;
use crate::handler::dynamo_items::DynamoItems;
use crate::handler::error::LambdaContactError;
use chrono::{Duration, Utc};
use regex::Regex;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput};
use std::borrow::Cow;
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

// The structure for the data to add to the table
#[derive(Debug, Clone)]
pub struct Contact {
    items: HashMap<String, AttributeValue>,
}

impl From<&ContactForm> for Contact {
    #[tracing::instrument(name = "convert contact form to contact record", skip(item))]
    fn from(item: &ContactForm) -> Self {
        let contact_id = Uuid::new_v4();
        let now = Utc::now();
        let expiry = if item.site.contains("jerus") {
            now.checked_add_signed(Duration::weeks(52)).unwrap()
        } else {
            now.checked_add_signed(Duration::weeks(1)).unwrap()
        };
        let clean_page = item.page.replace("/", "");

        let mut clean_site = String::from(&item.site);

        if let Ok(re) = Regex::new(r"\w+://([\w.]+)") {
            for cap in re.captures_iter(&item.site) {
                clean_site = String::from(&cap[1]);
            }
        }
        tracing::debug!("The site is {:?}", clean_site);

        tracing::debug!("The full sites is: {:?}", &item.site);
        let (event, referer, reference) = check_trackers(&item.site);

        Contact {
            items: DynamoItems::new()
                .insert_string("contact_id", &contact_id.to_string())
                .insert_string("name", &item.name)
                .insert_string("email", &item.email.clone())
                .insert_string("phone", &item.phone.clone())
                .insert_string("message", &item.message.clone())
                .insert_string("site", &clean_site)
                .insert_string("page", &clean_page)
                .insert_number("exp_date", &expiry.timestamp().to_string())
                .insert_string("event", &event)
                .insert_string("referer", &referer)
                .insert_string("reference", &reference)
                .items(),
        }
    }
}

#[tracing::instrument(
    name = "Write record to database"
    skip(form)
    fields(email = %form.email)
)]
pub async fn write(form: &ContactForm) -> Result<(), LambdaContactError> {
    let rec: Contact = form.into();

    let client = DynamoDbClient::new(Region::default());

    let table_name = "contacts".to_string();

    let input = PutItemInput {
        table_name: table_name.clone(),
        item: rec.items.clone(),
        ..PutItemInput::default()
    };

    tracing::debug!("The contact record: {:?}", rec);

    client.put_item(input).await?;
    Ok(())
}

#[tracing::instrument(name = "Check for trackers")]
fn check_trackers(site: &str) -> (String, String, String) {
    let parsed_url = Url::parse(site).unwrap();
    let mut pairs = parsed_url.query_pairs();

    let mut closure = |s: &str| -> String {
        match pairs.find(|(k, _v)| *k == Cow::Borrowed(s)) {
            Some((_k, v)) => {
                if let Cow::Borrowed(e) = v {
                    e.to_owned()
                } else {
                    "".to_owned()
                }
            }
            None => "".to_owned(),
        }
    };

    let event = closure("event");
    let referer = closure("referer");
    let reference = closure("ref");

    (event, referer, reference)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_all_tracker_data() {
        let site = "https://www.jerusdataprotect.ie/contact?event=meeting&referer=john&ref=great";

        let (event, referer, reference) = check_trackers(site);

        assert_eq!("meeting", event);
        assert_eq!("john", referer);
        assert_eq!("great", reference);
    }
}
