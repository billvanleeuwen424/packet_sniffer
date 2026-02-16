mod args;
mod capture;
mod error;

use args::Args;
use capture::{InterfaceProvider, OsInterfaceProvider};
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

    let provider = OsInterfaceProvider {};
    let interfaces = provider.list_interfaces().map_err(AppError::Interface)?;

    println!("Available interfaces: {:?}", interfaces);

    validate_interface(&interfaces, args.interface.as_deref())?;

    if let Some(name) = &args.interface {
        println!("interface: {}", name);
    }

    Ok(())
}

fn validate_interface(interfaces: &[String], name: Option<&str>) -> Result<(), AppError> {
    if interfaces.is_empty() {
        return Err(AppError::NoInterfaces);
    }
    if let Some(name) = name {
        if !interfaces.contains(&name.to_string()) {
            return Err(AppError::InterfaceNotFound(name.to_string()));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Satisfies: R-01-02 — valid --interface flag, no error
    #[test]
    fn valid_interface_returns_ok() {
        let interfaces = vec!["eth0".to_string(), "lo".to_string()];
        assert!(validate_interface(&interfaces, Some("eth0")).is_ok());
    }

    // Satisfies: R-01-09 — unknown interface name returns InterfaceNotFound
    #[test]
    fn unknown_interface_returns_not_found() {
        let interfaces = vec!["eth0".to_string(), "lo".to_string()];
        let result = validate_interface(&interfaces, Some("fake0"));
        assert!(matches!(result, Err(AppError::InterfaceNotFound(ref n)) if n == "fake0"));
    }

    // Satisfies: R-01-11 — no interfaces available returns NoInterfaces error
    #[test]
    fn empty_list_returns_no_interfaces() {
        let result = validate_interface(&[], None);
        assert!(matches!(result, Err(AppError::NoInterfaces)));
    }
}
