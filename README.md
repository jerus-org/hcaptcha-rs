# recaptcha-rs
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
    let res = recaptcha::verify("your_private_key", "user_response", Some("user_ip"));

    if res.is_ok() {
        println!("Success");
    } else {
        println!("Failure");
    }
}

```
