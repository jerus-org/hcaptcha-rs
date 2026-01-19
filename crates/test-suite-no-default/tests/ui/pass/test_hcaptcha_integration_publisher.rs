// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

const SITE_KEY: &str = "10000000-ffff-ffff-ffff-000000000001";
const SECRET_KEY: &str = "0x0000000000000000000000000000000000000000";
const RESPONSE: &str = "10000000-aaaa-bbbb-cccc-000000000001";

mod hcaptcha_integration;


#[tokio::main]
async 
fn main() {
    hcaptcha_integration::hcaptcha_integration_test(RESPONSE,SITE_KEY,SECRET_KEY).await.unwrap();
}
