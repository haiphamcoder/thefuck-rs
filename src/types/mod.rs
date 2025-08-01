use crate::{TheFuckError, TheFuckResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Represents a shell command with its context
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Command {
    /// The original command text
    pub text: String,
    /// The shell that executed the command
    pub shell: Shell,
    /// Command execution timestamp
    #[allow(clippy::type_complexity)]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Environment variables at execution time
    #[allow(clippy::type_complexity)]
    pub env: HashMap<String, String>,
    /// Working directory at execution time
    pub cwd: String,
}

impl Command {
    /// Creates a new command instance
    pub fn new(text: String, shell: Shell) -> Self {
        Self {
            text,
            shell,
            timestamp: chrono::Utc::now(),
            env: HashMap::new(),
            cwd: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| "unknown".to_string()),
        }
    }

    /// Parses the command and returns structured data
    #[allow(clippy::type_complexity)]
    pub fn parse(&self) -> TheFuckResult<ParsedCommand> {
        // TODO: Implement command parsing logic
        Ok(ParsedCommand {
            program: self
                .text
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string(),
            arguments: self
                .text
                .split_whitespace()
                .skip(1)
                .map(|s| s.to_string())
                .collect(),
            original: self.text.clone(),
        })
    }

    /// Validates the command
    #[allow(clippy::type_complexity)]
    pub fn validate(&self) -> TheFuckResult<()> {
        if self.text.trim().is_empty() {
            return Err(TheFuckError::validation_error(
                "Command text cannot be empty",
            ));
        }
        Ok(())
    }
}

/// Represents a parsed command with program and arguments
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedCommand {
    /// The program name
    pub program: String,
    /// The command arguments
    #[allow(clippy::type_complexity)]
    pub arguments: Vec<String>,
    /// The original command text
    pub original: String,
}

impl ParsedCommand {
    /// Creates a new parsed command
    #[allow(clippy::type_complexity)]
    pub fn new(program: String, arguments: Vec<String>, original: String) -> Self {
        Self {
            program,
            arguments,
            original,
        }
    }

    /// Gets the full command as a string
    pub fn as_string(&self) -> String {
        if self.arguments.is_empty() {
            self.program.clone()
        } else {
            format!("{} {}", self.program, self.arguments.join(" "))
        }
    }
}

impl fmt::Display for ParsedCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

/// Represents a corrected command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorrectedCommand {
    /// The corrected command text
    pub text: String,
    /// The original command
    pub original: Command,
    /// Priority of this correction (higher = more likely to be correct)
    pub priority: u32,
    /// Whether this correction requires confirmation
    pub requires_confirmation: bool,
    /// Side effects of this correction
    #[allow(clippy::type_complexity)]
    pub side_effects: Vec<String>,
}

impl CorrectedCommand {
    /// Creates a new corrected command
    pub fn new(text: String, original: Command, priority: u32) -> Self {
        Self {
            text,
            original,
            priority,
            requires_confirmation: true,
            side_effects: Vec::new(),
        }
    }

    /// Sets whether confirmation is required
    pub fn with_confirmation(mut self, requires_confirmation: bool) -> Self {
        self.requires_confirmation = requires_confirmation;
        self
    }

    /// Adds a side effect
    pub fn with_side_effect(mut self, side_effect: String) -> Self {
        self.side_effects.push(side_effect);
        self
    }
}

/// Supported shell types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Cmd,
    Unknown(String),
}

impl Shell {
    /// Creates a shell from string
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            "fish" => Shell::Fish,
            "powershell" | "pwsh" => Shell::PowerShell,
            "cmd" | "cmd.exe" => Shell::Cmd,
            _ => Shell::Unknown(s.to_string()),
        }
    }

    /// Gets the shell name as string
    pub fn as_string(&self) -> String {
        match self {
            Shell::Bash => "bash".to_string(),
            Shell::Zsh => "zsh".to_string(),
            Shell::Fish => "fish".to_string(),
            Shell::PowerShell => "powershell".to_string(),
            Shell::Cmd => "cmd".to_string(),
            Shell::Unknown(name) => name.clone(),
        }
    }

    /// Checks if this shell is supported
    pub fn is_supported(&self) -> bool {
        !matches!(self, Shell::Unknown(_))
    }
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

/// Command execution result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandResult {
    /// Whether the command was successful
    pub success: bool,
    /// Exit code
    pub exit_code: i32,
    /// Standard output
    pub stdout: String,
    /// Standard error
    pub stderr: String,
    /// Execution time in milliseconds
    pub execution_time: u64,
}

impl CommandResult {
    /// Creates a new command result
    pub fn new(
        success: bool,
        exit_code: i32,
        stdout: String,
        stderr: String,
        execution_time: u64,
    ) -> Self {
        Self {
            success,
            exit_code,
            stdout,
            stderr,
            execution_time,
        }
    }

    /// Creates a successful result
    pub fn success(stdout: String) -> Self {
        Self::new(true, 0, stdout, String::new(), 0)
    }

    /// Creates a failed result
    pub fn failure(exit_code: i32, stderr: String) -> Self {
        Self::new(false, exit_code, String::new(), stderr, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let cmd = Command::new("git status".to_string(), Shell::Bash);
        assert_eq!(cmd.text, "git status");
        assert_eq!(cmd.shell, Shell::Bash);
        assert!(!cmd.cwd.is_empty());
    }

    #[test]
    fn test_command_validation() {
        let cmd = Command::new("".to_string(), Shell::Bash);
        assert!(cmd.validate().is_err());

        let cmd = Command::new("git status".to_string(), Shell::Bash);
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn test_shell_parsing() {
        assert_eq!(Shell::from_string("bash"), Shell::Bash);
        assert_eq!(Shell::from_string("ZSH"), Shell::Zsh);
        assert_eq!(
            Shell::from_string("unknown"),
            Shell::Unknown("unknown".to_string())
        );
    }

    #[test]
    fn test_parsed_command() {
        let parsed = ParsedCommand::new(
            "git".to_string(),
            vec!["status".to_string()],
            "git status".to_string(),
        );
        assert_eq!(parsed.as_string(), "git status");
    }

    #[test]
    fn test_corrected_command() {
        let original = Command::new("git psh".to_string(), Shell::Bash);
        let corrected = CorrectedCommand::new("git push".to_string(), original, 100);
        assert_eq!(corrected.text, "git push");
        assert_eq!(corrected.priority, 100);
        assert!(corrected.requires_confirmation);
    }
}
