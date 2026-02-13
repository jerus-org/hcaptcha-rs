// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use hcaptcha::Captcha;
use rand::distr::Alphanumeric;
use rand::{rng, RngExt};

pub fn random_response() -> String {
    let mut rng = rng();
    (&mut rng)
        .sample_iter(Alphanumeric)
        .take(100)
        .map(char::from)
        .collect()
}

pub fn dummy_captcha() -> Captcha {
    Captcha::new(&random_response())
        .unwrap()
        .set_remoteip(&mockd::internet::ipv4_address())
        .unwrap()
        .set_sitekey(&mockd::unique::uuid_v4())
        .unwrap()
}

pub fn random_string(characters: usize) -> String {
    let mut rng = rng();
    (&mut rng)
        .sample_iter(Alphanumeric)
        .take(characters)
        .map(char::from)
        .collect()
}
