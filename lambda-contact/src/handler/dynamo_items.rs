use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;

pub struct DynamoItems {
    items: HashMap<String, AttributeValue>,
}

impl DynamoItems {
    pub fn new() -> DynamoItems {
        DynamoItems {
            items: HashMap::new(),
        }
    }

    pub fn insert_string(mut self, key: &str, value: &str) -> Self {
        self.items.insert(
            key.to_owned(),
            AttributeValue {
                s: Some(value.to_string()),
                ..AttributeValue::default()
            },
        );
        self
    }

    pub fn insert_number(mut self, key: &str, value: &str) -> Self {
        self.items.insert(
            key.to_owned(),
            AttributeValue {
                n: Some(value.to_string()),
                ..AttributeValue::default()
            },
        );
        self
    }

    pub fn items(self) -> HashMap<String, AttributeValue> {
        self.items
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rusoto_dynamodb::AttributeValue;

    #[test]
    fn insert_a_string() {
        let d_i = DynamoItems::new().insert_string("test", "test");

        let expected_attribute = AttributeValue {
            s: Some("test".to_owned()),
            ..AttributeValue::default()
        };

        assert!(d_i.items.contains_key("test"));
        assert_eq!(&expected_attribute, d_i.items.get("test").unwrap());
    }

    #[test]
    fn insert_a_number() {
        let d_i = DynamoItems::new().insert_number("test", "23");

        let expected_attribute = AttributeValue {
            n: Some("23".to_owned()),
            ..AttributeValue::default()
        };

        assert!(d_i.items.contains_key("test"));
        assert_eq!(&expected_attribute, d_i.items.get("test").unwrap());
    }
}
