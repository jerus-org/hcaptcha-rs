# Rust library hcaptcha

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]
[![Rust 1.46+][version-badge]][version-url]
[![FOSSA Status][fossa-badge]][fossa-url]
[![Docs][docs-badge]][docs-url]
[![BuyMeaCoffee][bmac-badge]][bmacs-url]

[crates-badge]: https://img.shields.io/crates/v/hcaptcha.svg
[crates-url]: https://crates.io/crates/hcaptcha
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jerusdp/hcaptcha-rs/blob/main/LICENSE
[actions-badge]: https://github.com/jerusdp/hcaptcha-rs/actions/workflows/general.yml/badge.svg?branch=main
[actions-url]: https://github.com/jerusdp/hcaptcha-rs/actions/workflows/general.yml
[version-badge]: https://img.shields.io/badge/rust-1.46+-orange.svg
[version-url]: https://www.rust-lang.org
[fossa-badge]: https://app.fossa.com/api/projects/custom%2B22707%2Fgithub.com%2Fjerusdp%2Fhcaptcha-rs.svg?type=shield
[fossa-url]: https://app.fossa.com/projects/custom%2B22707%2Fgithub.com%2Fjerusdp%2Fhcaptcha-rs?ref=badge_shield
[docs-badge]:  https://docs.rs/hcaptcha/badge.svg
[docs-url]:  https://docs.rs/hcapatcha
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: buymeacoffee.com/jerusdp

The rust library hcaptcha is used with your backend service to verify the hcaptcha response provided from the client.

## Installation

To use hcaptcha-rs in your project you can add the following to your `Cargo.toml`:

```toml
[dependencies.hcaptcha]
version = "2.0.0"
```

## Usage

Verifying hcaptcha responses is easy:

```rust
use hcaptcha::{HcaptchaClient, HcaptchaRequest};
# use itertools::Itertools;
# #[tokio::main]
# async fn main() -> Result<(), hcaptcha::HcaptchaError> {
    let secret = get_your_secret();
    let token = get_user_token();
    let remote_ip = get_user_ip_address();
    let request = HcaptchaRequest::new(&secret, &token)?
        .set_user_ip(remote_ip);
    let client = HcaptchaClient::new();
    let response = client.verify_client_response(request).await?;
    let score = match &response.score() {
        Some(v) => *v,
        None => 0.0,
    };
    let score_reasons = match &response.score_reason() {
        Some(v) => v.iter().join(", "),
        None => "".to_owned(),
    };
    println!("\tScore: {:?}\n\tReasons: {:?}", score, score_reasons);
    # Ok(())
# }
# fn get_your_secret() -> String {
#   "0x123456789abcde0f123456789abcdef012345678".to_string()
# }
# fn get_user_token() -> String {
#    "thisisnotapropertoken".to_string()
# }
# use std::net::{IpAddr, Ipv4Addr};
# fn get_user_ip_address() -> IpAddr {
#    IpAddr::V4(Ipv4Addr::new(192, 168, 0, 17))
# }
```

See the examples folder for an AWS Lambda contact form example.

## License

Licensed under either of

* Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

Initial version based on [recaptcha-rs](https://github.com/panicbit/recaptcha-rs) by panicbit.
