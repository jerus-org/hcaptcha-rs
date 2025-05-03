# Rust library hcaptcha

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][circleci-batch]][circleci-url]
[![Rust 1.81+][version-badge]][version-url]
[![Docs][docs-badge]][docs-url]
[![BuyMeaCoffee][bmac-badge]][bmac-url]
[![GitHubSponsors][ghub-badge]][ghub-url]
[![codecov][codecov-badge]][codecov-url]

[crates-badge]: https://img.shields.io/crates/v/hcaptcha.svg
[crates-url]: https://crates.io/crates/hcaptcha
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jerusdp/hcaptcha-rs/blob/main/LICENSE
[circleci-batch]: https://dl.circleci.com/status-badge/img/gh/jerus-org/hcaptcha-rs/tree/main.svg?style=svg
[circleci-url]: https://dl.circleci.com/status-badge/redirect/gh/jerus-org/hcaptcha-rs/tree/main
[version-badge]: https://img.shields.io/badge/rust-1.81+-orange.svg
[version-url]: https://www.rust-lang.org
[docs-badge]:  https://docs.rs/hcaptcha/badge.svg
[docs-url]:  https://docs.rs/hcapatcha
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: https://buymeacoffee.com/jerusdp
[ghub-badge]: https://img.shields.io/badge/sponsor-30363D?logo=GitHub-Sponsors&logoColor=#white
[ghub-url]: https://github.com/sponsors/jerusdp
[codecov-badge]: https://codecov.io/gh/jerus-org/hcaptcha-rs/graph/badge.svg?token=V4N745YPR3
[codecov-url]: https://codecov.io/gh/jerus-org/hcaptcha-rs

The rust library hcaptcha is used with your backend service to verify the hcaptcha response provided from the client.

## Installation

To use hcaptcha, add the following to your `Cargo.toml`:

```toml
[dependencies]
hcaptcha = "3.0.26"

```

## Breaking changes with version 3.0.0

- The `Hcaptcha` prefix has been removed from all types.
- The default feature now uses `rustls-backend` and not the `nativetls-backend`.
- The `verify_client_response` method has been deprecated in favour or the `verify` method.

## Usage

Derive a validation method on the data structure representing your data, marking the captcha components in the data structure.

``` rust
# use hcaptcha::Hcaptcha;

#[derive(Debug, Deserialize, Hcaptcha)]
pub struct ContactForm {
    name: String,
    phone: String,
    email: String,
    message: String,
    #[captcha]
    token: String,
}

```

Validate the captcha data.

``` rust
    # #[tokio::main]
    # async main() -> Result<(), Box<dyn std::error::Error>> {
    let contact_form: ContactForm = serde_json::from_str(e.body_string())?;
    contact_form.valid_response(&secret, None).await?;
    # }
    # fn get_your_secret() -> String {
    #   "0x123456789abcde0f123456789abcdef012345678".to_string()
    # }

```

See the examples folder for an AWS Lambda contact form example.

## Web Assembly

Hcaptcha has been tested in a web assembly project using wasm-bindgen and node.

See the `hcaptcha-wasm` example for a sample project which can be run using `wasm-pack test --node`.

## License

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

Initial version based on [recaptcha-rs](https://github.com/panicbit/recaptcha-rs) by panicbit.
