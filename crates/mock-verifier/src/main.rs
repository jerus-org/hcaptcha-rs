// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use rocket::{get, launch, routes};

mod tc001_blank_sitekey;
use tc001_blank_sitekey::tc001;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/siteverify")]
fn siteverify() -> &'static str {
    r#"tc001 - Blank Sitekey"#
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, siteverify, tc001])
}

#[cfg(test)]
mod test {
    use crate::tc001_blank_sitekey::{RequestData, SuccessResponse};

    use super::rocket;
    use rocket::{http::ContentType, local::blocking::Client};
    use rocket::{http::Status, uri};

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::hello)).dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Hello, world!".into()));
    }

    #[test]
    fn tc001() {
        let request = RequestData {
            response: Some("10000000-aaaa-bbbb-cccc-000000000001".to_string()),
            secret: Some("0x0000000000000000000000000000000000000000".to_string()),
            sitekey: Some("10000000-ffff-ffff-ffff-000000000001".to_string()),
            ..Default::default()
        };
        // urlencode request
        let request = serde_urlencoded::to_string(&request).unwrap();

        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!(super::tc001_blank_sitekey::tc001))
            .header(ContentType::Form)
            .body(request)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        // convert rsponse body to success response struct
        let response = response.into_json::<SuccessResponse>().unwrap();

        assert!(response.success());
        assert!(!response.credit());
        assert_eq!(response.hostname(), "dummy-key-pass");
    }
}
