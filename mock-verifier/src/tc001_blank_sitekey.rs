// mod helper;

// use chrono::{TimeDelta, Utc};
// use hcaptcha::{Code, Hcaptcha};
// use serde_json::json;
// use wiremock::matchers::{body_string, method, path};
// use wiremock::{Mock, MockServer, ResponseTemplate};

// #[derive(Debug, Hcaptcha)]
// struct Test {
//     #[captcha]
//     hcaptcha: String,
//     #[sitekey]
//     sitekey: String,
//     #[remoteip]
//     ip: String,
// }

use std::{collections::HashSet, fmt::Display};

use chrono::{TimeDelta, Utc};
use hcaptcha::Code;
use rocket::{form::Form, post, serde::json::Json, FromForm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SuccessResponse {
    success: bool,
    credit: bool,
    hostname: String,
    challenge_ts: String,
}

impl SuccessResponse {
    pub fn success(&self) -> bool {
        self.success
    }

    pub fn credit(&self) -> bool {
        self.credit
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    success: bool,
    #[serde(rename = "error-codes")]
    error_codes: Option<HashSet<Code>>,
}

#[derive(Debug, Serialize, Deserialize, Default, FromForm)]
pub struct RequestData {
    pub response: Option<String>,
    pub remoteip: Option<String>,
    pub sitekey: Option<String>,
    pub secret: Option<String>,
}

impl Display for RequestData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[post("/siteverify/tc001", data = "<token>")]
pub fn tc001(token: Form<RequestData>) -> Result<Json<SuccessResponse>, Json<ErrorResponse>> {
    let mut error_codes = HashSet::new();
    let mut early_exit = false;

    if token.response.is_none() {
        error_codes.insert(Code::MissingResponse);
        early_exit = true;
    };

    if token.secret.is_none() {
        error_codes.insert(Code::MissingSecret);
        early_exit = true;
    };

    if early_exit {
        return Err(ErrorResponse {
            success: false,
            error_codes: Some(error_codes),
        }
        .into());
    }
    let timestamp = Utc::now()
        .checked_sub_signed(TimeDelta::try_minutes(10).unwrap())
        .unwrap()
        .to_rfc3339();

    let response = SuccessResponse {
        success: true,
        challenge_ts: timestamp,
        hostname: String::from("dummy-key-pass"),
        credit: false,
    };
    Ok(response.into())
}

// #[tokio::main]
// async fn main() {
//     // Setup
//     let token = helper::random_string(100);
//     let remoteip = mockd::internet::ipv4_address();
//     // let sitekey = mockd::unique::uuid_v4();
//     let sitekey = "    ".to_string();
//     let secret = format!("0x{}", hex::encode(helper::random_string(20)));

//     let expected_body = format!(
//         "response={}&remoteip={}&sitekey={}&secret={}",
//         &token, &remoteip, &sitekey, &secret
//     );

//     let timestamp = Utc::now()
//         .checked_sub_signed(TimeDelta::try_minutes(10).unwrap())
//         .unwrap()
//         .to_rfc3339();

//     let response_template = ResponseTemplate::new(200).set_body_json(json!({
//         "success": true,
//         "challenge_ts": timestamp,
//         "hostname": "test-host",
//     }));

//     let mock_server = MockServer::start().await;
//     Mock::given(method("POST"))
//         .and(path("/siteverify"))
//         .and(body_string(&expected_body))
//         .respond_with(response_template)
//         .mount(&mock_server)
//         .await;

//     let uri = format!("{}{}", mock_server.uri(), "/siteverify");

//     let form = Test {
//         hcaptcha: token,
//         sitekey,
//         ip: remoteip,
//     };
//     let response = form.valid_response(&secret, Some(uri)).await;

//     claims::assert_err!(&response);

//     if let Err(codes) = response {
//         match codes {
//             hcaptcha::HcaptchaError::Codes(hash_set) => {
//                 assert_eq!(hash_set.len(), 1);
//                 assert!(hash_set.contains(&Code::MissingSiteKey));
//             }
//             _ => unreachable!(),
//         }
//     };
// }
