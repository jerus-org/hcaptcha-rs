mod utils;

use claims::assert_ok;
use hcaptcha::Hcaptcha;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, test-wasm!");
}

#[derive(Hcaptcha)]
struct Test {
    #[captcha]
    hcaptcha: String,
}

#[wasm_bindgen]
pub async fn valid_integration_test() {
    let response = "10000000-aaaa-bbbb-cccc-000000000001";
    let secret = "0x0000000000000000000000000000000000000000";
    // let sitekey = "10000000-ffff-ffff-ffff-000000000001";

    let form = Test {
        hcaptcha: response.to_string(),
    };
    let response = form.valid_response(secret, None).await;

    assert_ok!(&response);
    let response = response.unwrap();
    assert!(&response.success());
}
