// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use hcaptcha::{Captcha, Client, Request, Response};

mod cli;

#[cfg(target_os = "wasi")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Cli::parse();
    println!("Args found: {:?}", args);

    println!("{}", handle_cli(args).await?);

    Ok(())
}

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    println!("Args found: {args:?}");

    println!("{}", handle_cli(args).await?);

    Ok(())
}

async fn handle_cli(args: Cli) -> Result<Response> {
    let captcha = Captcha::new(&args.token)?;

    let secret = args.secret;

    let client = Client::new();

    let request = Request::new(&secret, captcha)?;

    let res = client.verify(request).await?;

    Ok(res)
}
