use claims::assert_ok;
use hcaptcha::Error;
use hcaptcha::Hcaptcha;

const BOT: &str = "30000000-ffff-ffff-ffff-000000000003";
const SAFE: &str = "20000000-ffff-ffff-ffff-000000000002";

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

    assert_ok!(&response);
    let response = response.unwrap();
    assert!(&response.success());

    // match site_key {
    //     SAFE => {
    //         assert!(response.score_reason().is_some());
    //         if let Some(hash_set) = response.score_reason() {
    //             assert!(!hash_set.is_empty());
    //             assert!(hash_set.contains("safe"));
    //         }
    //     }
    //     BOT => {
    //         assert!(response.score_reason().is_some());
    //         if let Some(hash_set) = response.score_reason() {
    //             assert!(!hash_set.is_empty());
    //             assert!(hash_set.contains("bot"));
    //         }
    //     }
    //     _ => {}
    // }
    Ok(())
}
