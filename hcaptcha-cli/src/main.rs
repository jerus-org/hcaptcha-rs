use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use hcaptcha::{HcaptchaCaptcha, HcaptchaClient, HcaptchaRequest, HcaptchaResponse};

mod cli;

#[cfg(target_family = "wasm")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Cli::parse();
    println!("Args found: {:?}", args);

    println!("{}", handle_cli(args).await?);

    Ok(())
}

// #[cfg(target_family = "linux")]
#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    println!("Args found: {:?}", args);

    println!("{}", handle_cli(args).await?);

    Ok(())
}

async fn handle_cli(args: Cli) -> Result<HcaptchaResponse> {
    let captcha = HcaptchaCaptcha::new(&args.token)?;

    let secret = args.secret;

    let client = HcaptchaClient::new();

    let request = HcaptchaRequest::new(&secret, captcha)?;

    let res = client.verify_client_response(request).await?;

    Ok(res)
}
