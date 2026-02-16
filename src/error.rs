use std::error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InterfaceNotFound(String),
    Interface(InterfaceError),
    #[allow(dead_code)]
    PermissionDenied,
    NoInterfaces,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InterfaceNotFound(name) => write!(f, "interface not found: {name}"),
            AppError::Interface(e) => write!(f, "{e}"),
            AppError::PermissionDenied => {
                write!(f, "permission denied (requires CAP_NET_RAW and sudo)")
            }
            AppError::NoInterfaces => write!(f, "no network interfaces found"),
        }
    }
}

impl error::Error for AppError {}

/// AppError tests
#[cfg(test)]
mod tests {

    use super::*;

    // Satisfies: R-01-09 — verifies InterfaceNotFound error message format
    #[test]
    fn test_interface_not_found_display() {
        let err = AppError::InterfaceNotFound("lo".to_string());

        let output = format!("{}", err);

        assert_eq!(output, "interface not found: lo");
    }

    // Satisfies: R-01-10 — verifies PermissionDenied message mentions sudo and CAP_NET_RAW
    #[test]
    fn test_permission_denied() {
        let err = AppError::PermissionDenied;

        let output = format!("{}", err);

        assert!(output.contains("sudo"), "{}", output);
        assert!(output.contains("CAP_NET_RAW"), "{}", output);
    }

    // Satisfies: R-01-11 — verifies NoInterfaces error message format
    #[test]
    fn test_no_interfaces() {
        let err = AppError::NoInterfaces;

        let output = format!("{}", err);

        assert_eq!(output, "no network interfaces found");
    }
}

#[derive(Debug)]
pub enum InterfaceError {
    Io(std::io::Error),
}

impl fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterfaceError::Io(e) => write!(f, "interface error: {}", e),
        }
    }
}

impl error::Error for InterfaceError {}
