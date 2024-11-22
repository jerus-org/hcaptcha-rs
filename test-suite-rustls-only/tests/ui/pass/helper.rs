use hcaptcha::Captcha;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

pub fn random_response() -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(100)
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
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(characters)
        .collect()
}
