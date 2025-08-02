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

    /// Creates a new command with environment variables
    #[allow(clippy::type_complexity)]
    pub fn with_env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }

    /// Creates a new command with a specific working directory
    pub fn with_cwd(mut self, cwd: String) -> Self {
        self.cwd = cwd;
        self
    }

    /// Creates a new command with a specific timestamp
    #[allow(clippy::type_complexity)]
    pub fn with_timestamp(mut self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Gets the command text as a string slice
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Gets the command text trimmed of whitespace
    pub fn trimmed(&self) -> &str {
        self.text.trim()
    }

    /// Checks if the command is empty (after trimming)
    pub fn is_empty(&self) -> bool {
        self.trimmed().is_empty()
    }

    /// Gets the first word of the command (the program name)
    #[allow(clippy::type_complexity)]
    pub fn program(&self) -> Option<&str> {
        self.trimmed().split_whitespace().next()
    }

    /// Gets all arguments as a vector of strings
    #[allow(clippy::type_complexity)]
    pub fn arguments(&self) -> Vec<&str> {
        self.trimmed().split_whitespace().skip(1).collect()
    }

    /// Gets the number of arguments
    pub fn argument_count(&self) -> usize {
        self.arguments().len()
    }

    /// Checks if the command has any arguments
    pub fn has_arguments(&self) -> bool {
        self.argument_count() > 0
    }

    /// Gets a specific argument by index
    #[allow(clippy::type_complexity)]
    pub fn argument(&self, index: usize) -> Option<&str> {
        self.arguments().get(index).copied()
    }

    /// Checks if the command starts with a specific program
    pub fn starts_with(&self, program: &str) -> bool {
        self.program()
            .map(|p| p.eq_ignore_ascii_case(program))
            .unwrap_or(false)
    }

    /// Checks if the command contains a specific argument
    pub fn contains_argument(&self, arg: &str) -> bool {
        self.arguments().iter().any(|a| a.eq_ignore_ascii_case(arg))
    }

    /// Parses the command and returns structured data
    #[allow(clippy::type_complexity)]
    pub fn parse(&self) -> TheFuckResult<ParsedCommand> {
        let trimmed = self.trimmed();
        if trimmed.is_empty() {
            return Err(TheFuckError::parse_error("Command text is empty"));
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let program = parts.first().unwrap_or(&"").to_string();
        let arguments = parts.iter().skip(1).map(|s| s.to_string()).collect();

        Ok(ParsedCommand {
            program,
            arguments,
            original: self.text.clone(),
        })
    }

    /// Validates the command
    #[allow(clippy::type_complexity)]
    pub fn validate(&self) -> TheFuckResult<()> {
        if self.is_empty() {
            return Err(TheFuckError::validation_error(
                "Command text cannot be empty",
            ));
        }
        Ok(())
    }

    /// Creates a new command with modified text
    pub fn with_text(&self, text: String) -> Self {
        Self {
            text,
            shell: self.shell.clone(),
            timestamp: self.timestamp,
            env: self.env.clone(),
            cwd: self.cwd.clone(),
        }
    }

    /// Gets the command age in seconds
    pub fn age_seconds(&self) -> i64 {
        let now = chrono::Utc::now();
        (now - self.timestamp).num_seconds()
    }

    /// Gets the command age in minutes
    pub fn age_minutes(&self) -> i64 {
        self.age_seconds() / 60
    }

    /// Checks if the command is recent (within the last hour)
    pub fn is_recent(&self) -> bool {
        self.age_minutes() < 60
    }

    /// Gets an environment variable value
    #[allow(clippy::type_complexity)]
    pub fn get_env(&self, key: &str) -> Option<&String> {
        self.env.get(key)
    }

    /// Sets an environment variable
    pub fn set_env(&mut self, key: String, value: String) {
        self.env.insert(key, value);
    }

    /// Removes an environment variable
    #[allow(clippy::type_complexity)]
    pub fn remove_env(&mut self, key: &str) -> Option<String> {
        self.env.remove(key)
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
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
    fn test_command_builder_methods() {
        let mut env = HashMap::new();
        env.insert("PATH".to_string(), "/usr/bin".to_string());

        let cmd = Command::new("git status".to_string(), Shell::Bash)
            .with_env(env.clone())
            .with_cwd("/home/user".to_string());

        assert_eq!(cmd.env, env);
        assert_eq!(cmd.cwd, "/home/user");
    }

    #[test]
    fn test_command_text_methods() {
        let cmd = Command::new("  git status  ".to_string(), Shell::Bash);

        assert_eq!(cmd.as_str(), "  git status  ");
        assert_eq!(cmd.trimmed(), "git status");
        assert!(!cmd.is_empty());

        let empty_cmd = Command::new("   ".to_string(), Shell::Bash);
        assert!(empty_cmd.is_empty());
    }

    #[test]
    fn test_command_parsing_methods() {
        let cmd = Command::new("git status --porcelain".to_string(), Shell::Bash);

        assert_eq!(cmd.program(), Some("git"));
        assert_eq!(cmd.arguments(), vec!["status", "--porcelain"]);
        assert_eq!(cmd.argument_count(), 2);
        assert!(cmd.has_arguments());
        assert_eq!(cmd.argument(0), Some("status"));
        assert_eq!(cmd.argument(1), Some("--porcelain"));
        assert_eq!(cmd.argument(2), None);
    }

    #[test]
    fn test_command_matching_methods() {
        let cmd = Command::new("git push origin main".to_string(), Shell::Bash);

        assert!(cmd.starts_with("git"));
        assert!(cmd.starts_with("GIT")); // case insensitive
        assert!(!cmd.starts_with("ls"));

        assert!(cmd.contains_argument("push"));
        assert!(cmd.contains_argument("PUSH")); // case insensitive
        assert!(!cmd.contains_argument("pull"));
    }

    #[test]
    fn test_command_parse() {
        let cmd = Command::new("git status --porcelain".to_string(), Shell::Bash);
        let parsed = cmd.parse().unwrap();

        assert_eq!(parsed.program, "git");
        assert_eq!(parsed.arguments, vec!["status", "--porcelain"]);
        assert_eq!(parsed.original, "git status --porcelain");
    }

    #[test]
    fn test_command_parse_empty() {
        let cmd = Command::new("   ".to_string(), Shell::Bash);
        assert!(cmd.parse().is_err());
    }

    #[test]
    fn test_command_with_text() {
        let original = Command::new("git status".to_string(), Shell::Bash);
        let modified = original.with_text("git add .".to_string());

        assert_eq!(modified.text, "git add .");
        assert_eq!(modified.shell, original.shell);
        assert_eq!(modified.cwd, original.cwd);
    }

    #[test]
    fn test_command_age() {
        let cmd = Command::new("git status".to_string(), Shell::Bash);

        // Command should be very recent (just created)
        assert!(cmd.age_seconds() < 5);
        assert!(cmd.age_minutes() < 1);
        assert!(cmd.is_recent());
    }

    #[test]
    fn test_command_env_methods() {
        let mut cmd = Command::new("git status".to_string(), Shell::Bash);

        // Test setting and getting env vars
        cmd.set_env("PATH".to_string(), "/usr/bin".to_string());
        assert_eq!(cmd.get_env("PATH"), Some(&"/usr/bin".to_string()));
        assert_eq!(cmd.get_env("NONEXISTENT"), None);

        // Test removing env vars
        let removed = cmd.remove_env("PATH");
        assert_eq!(removed, Some("/usr/bin".to_string()));
        assert_eq!(cmd.get_env("PATH"), None);
    }

    #[test]
    fn test_command_display() {
        let cmd = Command::new("git status".to_string(), Shell::Bash);
        assert_eq!(format!("{}", cmd), "git status");
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
