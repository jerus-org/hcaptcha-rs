// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use test_wasm::validate_standard;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn valid_integration_test() {
    validate_standard().await;
}
