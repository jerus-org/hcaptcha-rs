use hcaptcha_derive::Hcaptcha;

#[derive(Hcaptcha)]
pub enum ContactEnum {
    Name,
    Token,
}

fn main() {
    println!("hello");
}
