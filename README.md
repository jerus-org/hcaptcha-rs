# Rust library hcaptcha

[![Rust](https://github.com/jerusdp/hcaptcha-rs/actions/workflows/general.yml/badge.svg?branch=main)](https://github.com/jerusdp/hcaptcha-rs/actions/workflows/general.yml)
[![Rust 1.46+](https://img.shields.io/badge/rust-1.46+-orange.svg)](https://www.rust-lang.org)
[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B22707%2Fgithub.com%2Fjerusdp%2Fhcaptcha-rs.svg?type=shield)](https://app.fossa.com/projects/custom%2B22707%2Fgithub.com%2Fjerusdp%2Fhcaptcha-rs?ref=badge_shield)

The rust library hcaptcha is used with your backend service to verify the hcaptcha response provided from the client.

## Installation

To use hcaptcha-rs in your project you can add the following to your `Cargo.toml`:

```toml
[dependencies.hcaptcha]
version = "1.0.0"
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

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

Based on [recaptcha-rs](https://github.com/panicbit/recaptcha-rs) by panicbit.
