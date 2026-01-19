// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use hcaptcha_derive::Hcaptcha;

#[derive(Debug, Hcaptcha)]
pub struct ContactForm {
    name: String,
    #[allow(dead_code)]
    phone: String,
    email: String,
    #[allow(dead_code)]
    message: String,
    #[captcha]
    token: String,
}
