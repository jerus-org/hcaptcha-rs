# hcaptcha-rs [![Build Status](https://travis-ci.org/jerusdp/hcaptcha-rs.svg)](https://travis-ci.org/jerusdp/hcaptcha-rs) [![Rust 1.38+](https://img.shields.io/badge/rust-1.38+-orange.svg)](https://www.rust-lang.org)
hcaptcha-rs is a very simple library to verify hcaptcha responses.

## Installation
To use hcaptcha-rs in your project you can add the following to your `Cargo.toml`:
```toml
[dependencies.hcaptcha]
version = "0.1.0"
```

## Usage
Verifying hcaptcha responses is very easy:
```rust

fn main() {
    let remote_ip = "123.123.123.123".parse().ok();
    let res = recaptcha::verify("your_private_key", "user_response", remote_ip);

    if res.is_ok() {
        println!("Success");
    } else {
        println!("Failure");
    }
}

```

## Credits
Based on [recaptcha-rs](https://github.com/panicbit/recaptcha-rs) by panicbit