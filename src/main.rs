mod app;
mod args;
mod capture;
mod error;
mod tui;

use args::Args;
use capture::{NullPacketSource, OsInterfaceProvider};
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
    let provider = OsInterfaceProvider;
    let source = NullPacketSource;
    let mut app = app::App::new(source, &provider, args.interface)?;
    let mut tui = tui::Tui::enter().map_err(error::InterfaceError::from)?;
    app.run(&mut tui)
}
