use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use hcaptcha::{HcaptchaCaptcha, HcaptchaClient, HcaptchaRequest};

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    println!("Args found: {:?}", args);

    let captcha = HcaptchaCaptcha::new(&args.token)?;

    let secret = args.secret;

    let client = HcaptchaClient::new();

    let request = HcaptchaRequest::new(&secret, captcha)?;

    let res = client.verify_client_response(request).await?;

    println!("{res}");
    Ok(())
}
