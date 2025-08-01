use thiserror::Error;

/// Main error type for the thefuck-rs application
#[derive(Error, Debug)]
pub enum TheFuckError {
    /// Command parsing errors
    #[error("Failed to parse command: {0}")]
    ParseError(String),

    /// Shell-related errors
    #[error("Shell not supported: {0}")]
    UnsupportedShell(String),

    #[error("Shell execution failed: {0}")]
    ShellExecutionError(String),

    /// Rule-related errors
    #[error("No matching rules found for command: {0}")]
    NoRulesFound(String),

    #[error("Rule execution failed: {0}")]
    RuleExecutionError(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Failed to load configuration file: {0}")]
    ConfigLoadError(String),

    /// File system errors
    #[error("File system error: {0}")]
    FileSystemError(String),

    #[error("Failed to read file: {0}")]
    FileReadError(String),

    #[error("Failed to write file: {0}")]
    FileWriteError(String),

    /// Process and execution errors
    #[error("Process execution failed: {0}")]
    ProcessError(String),

    #[error("Command execution failed: {0}")]
    CommandExecutionError(String),

    /// History-related errors
    #[error("Failed to access command history: {0}")]
    HistoryError(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Network errors (for future use)
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Path conversion errors
    #[error("Path conversion error: {0}")]
    PathError(String),

    /// Generic I/O errors
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Serde serialization/deserialization errors
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    /// TOML parsing errors
    #[error(transparent)]
    TomlError(#[from] toml::de::Error),

    /// UTF-8 encoding errors
    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

/// Result type alias for thefuck-rs operations
pub type TheFuckResult<T> = Result<T, TheFuckError>;

/// Error context for better error messages
pub trait ErrorContext<T> {
    /// Add context to an error
    #[allow(clippy::type_complexity)]
    fn with_context<C>(self, context: C) -> TheFuckResult<T>
    where
        C: std::fmt::Display;
}

impl<T> ErrorContext<T> for TheFuckResult<T> {
    fn with_context<C>(self, context: C) -> TheFuckResult<T>
    where
        C: std::fmt::Display,
    {
        self.map_err(|e| match e {
            TheFuckError::ParseError(msg) => TheFuckError::ParseError(format!("{context}: {msg}")),
            TheFuckError::ShellExecutionError(msg) => {
                TheFuckError::ShellExecutionError(format!("{context}: {msg}"))
            }
            TheFuckError::RuleExecutionError(msg) => {
                TheFuckError::RuleExecutionError(format!("{context}: {msg}"))
            }
            TheFuckError::ConfigError(msg) => {
                TheFuckError::ConfigError(format!("{context}: {msg}"))
            }
            TheFuckError::FileSystemError(msg) => {
                TheFuckError::FileSystemError(format!("{context}: {msg}"))
            }
            TheFuckError::ProcessError(msg) => {
                TheFuckError::ProcessError(format!("{context}: {msg}"))
            }
            TheFuckError::CommandExecutionError(msg) => {
                TheFuckError::CommandExecutionError(format!("{context}: {msg}"))
            }
            TheFuckError::HistoryError(msg) => {
                TheFuckError::HistoryError(format!("{context}: {msg}"))
            }
            TheFuckError::ValidationError(msg) => {
                TheFuckError::ValidationError(format!("{context}: {msg}"))
            }
            TheFuckError::NetworkError(msg) => {
                TheFuckError::NetworkError(format!("{context}: {msg}"))
            }
            TheFuckError::PathError(msg) => TheFuckError::PathError(format!("{context}: {msg}")),
            _ => e,
        })
    }
}

/// Helper functions for creating common errors
impl TheFuckError {
    /// Create a parse error
    pub fn parse_error<S: Into<String>>(message: S) -> Self {
        TheFuckError::ParseError(message.into())
    }

    /// Create a shell error
    pub fn unsupported_shell<S: Into<String>>(shell: S) -> Self {
        TheFuckError::UnsupportedShell(shell.into())
    }

    /// Create a no rules found error
    pub fn no_rules_found<S: Into<String>>(command: S) -> Self {
        TheFuckError::NoRulesFound(command.into())
    }

    /// Create a configuration error
    pub fn config_error<S: Into<String>>(message: S) -> Self {
        TheFuckError::ConfigError(message.into())
    }

    /// Create a file system error
    pub fn file_system_error<S: Into<String>>(message: S) -> Self {
        TheFuckError::FileSystemError(message.into())
    }

    /// Create a process error
    pub fn process_error<S: Into<String>>(message: S) -> Self {
        TheFuckError::ProcessError(message.into())
    }

    /// Create a validation error
    pub fn validation_error<S: Into<String>>(message: S) -> Self {
        TheFuckError::ValidationError(message.into())
    }

    /// Create a path error
    pub fn path_error<S: Into<String>>(message: S) -> Self {
        TheFuckError::PathError(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let parse_err = TheFuckError::parse_error("Invalid command syntax");
        assert!(matches!(parse_err, TheFuckError::ParseError(_)));

        let shell_err = TheFuckError::unsupported_shell("cmd.exe");
        assert!(matches!(shell_err, TheFuckError::UnsupportedShell(_)));

        let no_rules_err = TheFuckError::no_rules_found("git psh");
        assert!(matches!(no_rules_err, TheFuckError::NoRulesFound(_)));
    }

    #[test]
    fn test_error_context() {
        #[allow(clippy::type_complexity)]
        let result: TheFuckResult<()> = Err(TheFuckError::parse_error("test"));
        let result_with_context = result.with_context("parsing command");

        match result_with_context {
            Err(TheFuckError::ParseError(msg)) => {
                assert!(msg.contains("parsing command"));
                assert!(msg.contains("test"));
            }
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_error_display() {
        let error = TheFuckError::parse_error("test error");
        let display = format!("{error}");
        assert_eq!(display, "Failed to parse command: test error");
    }
}
