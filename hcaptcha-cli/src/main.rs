use clap::Parser;
use cli::Cli;

mod cli;

fn main() {
    let args = Cli::parse();
    println!("Args found: {:?}", args);
}
