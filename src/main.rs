mod args;
mod error;

use args::Args;
use clap::Parser;
use error::AppError;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let args = Args::parse();

    match &args.interface {
        Some(name) => {
            println!("interface: {}", name);

            Ok(())
        }
        None => {
            println!("interface: None given");

            Ok(())
        }
    }
}
