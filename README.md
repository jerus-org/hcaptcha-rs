# Rust library hcaptcha

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][circleci-batch]][circleci-url]
[![Rust 1.51+][version-badge]][version-url]
[![FOSSA Status][fossa-badge]][fossa-url]
[![Docs][docs-badge]][docs-url]
[![BuyMeaCoffee][bmac-badge]][bmac-url]
[![GitHubSponsors][ghub-badge]][ghub-url]

[crates-badge]: https://img.shields.io/crates/v/hcaptcha.svg
[crates-url]: https://crates.io/crates/hcaptcha
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jerusdp/hcaptcha-rs/blob/main/LICENSE
[circleci-batch]:https://circleci.com/gh/jerusdp/hcaptcha-rs/tree/main.svg?style=svg
[circleci-url]: https://circleci.com/gh/jerusdp/hcaptcha-rs/tree/main
[actions-badge]: https://github.com/jerusdp/hcaptcha-rs/actions/workflows/general.yml/badge.svg?branch=main
[actions-url]: https://github.com/jerusdp/hcaptcha-rs/actions/workflows/general.yml
[version-badge]: https://img.shields.io/badge/rust-1.56+-orange.svg
[version-url]: https://www.rust-lang.org
[fossa-badge]: https://app.fossa.com/api/projects/custom%2B22707%2Fgithub.com%2Fjerusdp%2Fhcaptcha-rs.svg?type=shield
[fossa-url]: https://app.fossa.com/projects/custom%2B22707%2Fgithub.com%2Fjerusdp%2Fhcaptcha-rs?ref=badge_shield
[docs-badge]:  https://docs.rs/hcaptcha/badge.svg
[docs-url]:  https://docs.rs/hcapatcha
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: https://buymeacoffee.com/jerusdp
[ghub-badge]: https://img.shields.io/badge/sponsor-30363D?logo=GitHub-Sponsors&logoColor=#white
[ghub-url]: https://github.com/sponsors/jerusdp

The rust library hcaptcha is used with your backend service to verify the hcaptcha response provided from the client.

## Installation

To use hcaptcha-rs in your project you can add the following to your `Cargo.toml`:

```toml
[dependencies.hcaptcha]
version = "2.1.0"
```

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

## License

Licensed under either of

* Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

### Third Party Licenses

A summary of third party licenses can be found [here][fossa-report-url]

[fossa-report-url]: https://app.fossa.com/attribution/524389e4-8ef2-4dd1-9453-d07c39efa929

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

Initial version based on [recaptcha-rs](https://github.com/panicbit/recaptcha-rs) by panicbit.
