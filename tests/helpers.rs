pub fn random_hex_string(len: usize) -> String {
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len / 2)
        .collect();

    hex::encode(chars)
}

pub fn random_response() -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(100)
        .collect()
}

pub fn dummy_captcha() -> HcaptchaCaptcha {
    HcaptchaCaptcha::new(&random_response())
        .unwrap()
        .set_user_ip(&fakeit::internet::ipv4_address())
        .unwrap()
        .set_site_key(&fakeit::unique::uuid_v4())
        .unwrap()
}
