//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
async fn testing_run_test() {
    let expected = JsValue::from_str("200 OK");
    println!("Test value: {:?}", expected);

    println!();
    println!("THE TEST");
    println!("--------");
    println!();
    // let form: HcaptchaForm = request.into();
    // eprintln!(
    //     "The form to submit to Hcaptcha API: {:?}",
    //     serde_urlenc oded::to_string(&form).unwrap_or_else(|_| "form corrupted".to_owned())
    // );

    let client_builder = reqwest::Client::builder();
    let client = client_builder.build().unwrap();
    println!("The client is: {:?}", client);

    let url = "http://httpbin.org/post";
    // let url = "https://api.hcaptcha.com/siteverify";
    println!("The url is: {}", url);

    let response_res = client
        .post(url)
        // .form(&form)
        .send()
        .await;
    println!("The response from the Hcaptcha API: {:#?}", response_res);
    let response = response_res.expect("response result unwrapped");

    let status = response.status();
    println!("The status code is: {}", status);

    let test_value = status.to_string().into();

    assert_eq!(test_value, expected);
}
