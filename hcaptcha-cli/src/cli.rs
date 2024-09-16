use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    pub logging: clap_verbosity_flag::Verbosity,
    /// The captcha token proivded by the client for validation
    #[clap(short, long)]
    pub token: String,
    /// The sitekey for hcapthca validation
    #[clap(short, long)]
    pub key: Option<String>,
    /// The secret key for hcaptcha validation
    #[clap(short, long)]
    pub secret: String,
    /// The ip address of the system generating the request
    #[clap(short, long)]
    pub ip: Option<String>,
}
