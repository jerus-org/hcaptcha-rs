//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn valid_integration_test() {
    hcaptcha_wasm::validate_standard().await;
}
