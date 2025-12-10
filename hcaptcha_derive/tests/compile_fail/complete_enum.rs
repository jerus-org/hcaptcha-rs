// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use hcaptcha_derive::Hcaptcha;

#[derive(Hcaptcha)]
pub enum ContactEnum {
    Name,
    #[captcha]
    Token,
}

fn main() {
    println!("hello");
}
