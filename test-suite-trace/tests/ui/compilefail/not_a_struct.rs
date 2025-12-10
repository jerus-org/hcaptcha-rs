// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use hcaptcha::Hcaptcha;
// use hcaptcha_derive::Hcaptcha;

#[derive(Hcaptcha)]
enum Test {
    #[captcha]
    Hcaptcha(String),
}

fn main() {
    println!("Super!");
}