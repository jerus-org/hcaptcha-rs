use rocket::{
    fairing::{Fairing, Info, Kind},
    get,
    http::Header,
    launch, routes, Request, Response,
};

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

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, siteverify, tc001])
        .attach(CORS {})
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
