use crate::error::InterfaceError;
use std::fs;

pub trait InterfaceProvider {
    fn list_interfaces(&self) -> Result<Vec<String>, InterfaceError>;
}

pub struct OsInterfaceProvider;

impl InterfaceProvider for OsInterfaceProvider {
    fn list_interfaces(&self) -> Result<Vec<String>, InterfaceError> {
        let content = fs::read_to_string("/proc/net/dev").map_err(InterfaceError::Io)?;
        let interfaces = content
            .lines()
            .skip(2)
            .filter_map(|line| line.split(':').next())
            .map(|s| s.trim().to_string())
            .collect();
        Ok(interfaces)
    }
}

#[cfg(test)]
pub mod test_helpers {
    use super::*;

    pub struct MockInterfaceProvider {
        interfaces: Vec<String>,
        should_fail: bool,
    }

    impl MockInterfaceProvider {
        pub fn new(interfaces: Vec<String>) -> Self {
            Self {
                interfaces,
                should_fail: false,
            }
        }

        pub fn failing() -> Self {
            Self {
                interfaces: vec![],
                should_fail: true,
            }
        }
    }

    impl InterfaceProvider for MockInterfaceProvider {
        fn list_interfaces(&self) -> Result<Vec<String>, InterfaceError> {
            if self.should_fail {
                Err(InterfaceError::Io(std::io::Error::other("mock failure")))
            } else {
                Ok(self.interfaces.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers::MockInterfaceProvider;

    // No specific requirement — tests the MockInterfaceProvider testability seam
    #[test]
    fn mock_returns_exact_interfaces() {
        let provider = MockInterfaceProvider::new(vec!["eth0".to_string(), "lo".to_string()]);
        let result = provider.list_interfaces().unwrap();
        assert_eq!(result, vec!["eth0", "lo"]);
    }

    // Satisfies: R-01-03 — enumerates available interfaces; lo must appear in the list
    #[test]
    fn os_provider_returns_nonempty_list_with_lo() {
        let provider = OsInterfaceProvider {};
        let result = provider.list_interfaces().unwrap();
        assert!(!result.is_empty());
        assert!(result.contains(&"lo".to_string()));
    }
}
