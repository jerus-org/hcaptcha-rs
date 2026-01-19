// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use claims::assert_err;
use hcaptcha::Error;
use hcaptcha::Hcaptcha;

#[derive(Debug, Hcaptcha)]
pub struct Test {
    #[captcha]
    hcaptcha: String,
    #[sitekey]
    sitekey: String,
}

pub async fn hcaptcha_integration_test(
    response: &str,
    site_key: &str,
    secret_key: &str,
) -> Result<(), Error> {
    let form = Test {
        hcaptcha: response.to_string(),
        sitekey: site_key.to_string(),
    };

    let response = form.valid_response(secret_key, None).await;

    assert_err!(&response);

    // TODO: confirm the actual error source is
    //      source: hyper::Error(Connect, "invalid URL, scheme is not http")

    Ok(())
}
