# Rust library hcaptcha

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]
[![Rust 1.46+][version-badge]][version-url]
[![FOSSA Status][fossa-badge]][fossa-url]
[![Docs][docs-badge]][docs-url]

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

use hcaptcha::Hcaptcha;
use std::net::{IpAddr, Ipv4Addr};
#[tokio::main]
async fn main() {
    let remote_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 17));

    let res = Hcaptcha::new("your_private_key", "user_response")
                .set_user_ip(&remote_ip)
                .verify()
                .await;
    if res.is_ok() {
        println!("Success");
    } else {
        println!("Failure");
    }
}

```

See the examples folder for an AWS Lambda contact form example.

## License

* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

Based on [recaptcha-rs](https://github.com/panicbit/recaptcha-rs) by panicbit.
