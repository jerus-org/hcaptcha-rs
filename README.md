# hcaptcha-rs 
[![Build Status](https://travis-ci.org/jerusdp/recaptcha-rs.svg?branch=main)](https://travis-ci.org/jerusdp/hcaptcha-rs) 
[![Rust 1.39+](https://img.shields.io/badge/rust-1.39+-orange.svg)](https://www.rust-lang.org)
hcaptcha-rs is a library to verify hcaptcha responses.

## Installation
To use hcaptcha-rs in your project you can add the following to your `Cargo.toml`:
```toml
[dependencies.hcaptcha]
version = "0.1.0"
```

## Usage
Verifying hcaptcha responses is very easy:
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

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits
Based on [recaptcha-rs](https://github.com/panicbit/recaptcha-rs) by panicbit