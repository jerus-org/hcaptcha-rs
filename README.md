# recaptcha-rs [![Build Status](https://travis-ci.org/panicbit/recaptcha-rs.svg)](https://travis-ci.org/panicbit/recaptcha-rs)
Recaptcha-rs is a very simple library to verify recaptcha responses.

## Installation
To use recaptcha-rs in your project you can add the following to your `Cargo.toml`:
```toml
[dependencies.recaptcha]
version = "0"
```

## Usage
Verifying recaptcha responses is very easy:
```rust
extern crate recaptcha;

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
