use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use hcaptcha::{HcaptchaCaptcha, HcaptchaClient, HcaptchaRequest};

mod cli;

#[cfg(target_os = "wasi")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Cli::parse();
    eprintln!("Args found: {:?}", args);

    println!("{}", handle_cli(args).await?);

    Ok(())
}

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() {
    use std::process::exit;

    let args = Cli::parse();
    eprintln!("Args found: {:?}", args);

    let res = handle_cli(args).await;

    eprintln!("{:?}", res);

    match res {
        Ok(o) => println!("{o}"),
        Err(e) => {
            println!("{e}");
            exit(1)
        }
    };
}

async fn handle_cli(args: Cli) -> Result<String> {
    let captcha = HcaptchaCaptcha::new(&args.token)?;

    let secret = args.secret;

    let client = HcaptchaClient::new();

    let request = HcaptchaRequest::new(&secret, captcha)?;

    eprintln!("request: {:#?}", request);

    let res = client.verify_client_response(request).await?;

    // let res = res.success().to_string();

    // eprintln!("{:#?}", res);

    let res = res.to_json()?;

    Ok(res)
}
