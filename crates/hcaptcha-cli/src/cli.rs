// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    pub logging: clap_verbosity_flag::Verbosity,
    /// The captcha token provided by the client for validation
    #[clap(short, long)]
    pub token: String,
    /// The sitekey for hcaptcha validation
    #[clap(short, long)]
    pub key: Option<String>,
    /// The secret key for hcaptcha validation
    #[clap(short, long)]
    pub secret: String,
    /// The ip address of the system generating the request
    #[clap(short, long)]
    pub ip: Option<String>,
}
