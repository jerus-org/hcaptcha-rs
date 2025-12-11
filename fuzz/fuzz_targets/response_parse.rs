// SPDX-FileCopyrightText: 2025 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#![no_main]
use libfuzzer_sys::fuzz_target;
use serde_json::from_slice;

// Parse potential hCaptcha API responses; parser should not panic.
fuzz_target!(|data: &[u8]| {
    let _ = from_slice::<hcaptcha::Response>(data);
});