use hcaptcha_derive::Hcaptcha;

#[derive(Hcaptcha)]
pub enum ContactEnum {
    Name,
    #[captcha]
    Token,
}

fn main() {
    println!("hello");
}
