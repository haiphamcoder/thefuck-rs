# Contributing to TheFuck-rs

Thank you for your interest in contributing to TheFuck-rs! This project is a Rust rewrite of [TheFuck](https://github.com/nvbn/thefuck) and we welcome contributions from the community.

## 📋 Table of Contents

- [How to Contribute](#-how-to-contribute)
- [Development Environment Setup](#-development-environment-setup)
- [Development Workflow](#-development-workflow)
- [CI/CD Pipeline](#-cicd-pipeline)
- [Code Guidelines](#-code-guidelines)
- [Testing](#-testing)
- [Bug Reports](#-bug-reports)
- [Feature Requests](#-feature-requests)
- [Documentation](#documentation)
- [Releases](#-releases)
- [Code of Conduct](#-code-of-conduct)
- [Contact](#-contact)

## 🚀 How to Contribute

### Types of Contributions We Welcome

- **Bug Reports**: Find and report bugs in the application
- **Feature Requests**: Suggest new features or improvements
- **Documentation**: Update README, docs, or comments
- **Performance Optimization**: Improve speed and memory usage
- **New Rules**: Implement new rules for different commands
- **UX Improvements**: Enhance user experience
- **Testing**: Write tests or improve test coverage

### Before You Start

1. **Check existing issues**: Browse [GitHub Issues](https://github.com/haiphamcoder/thefuck-rs/issues) to avoid duplicates
2. **Discuss**: If you have a big idea, create an issue to discuss it first
3. **Choose appropriate tasks**: Start with issues labeled "good first issue" or "help wanted"

## 🛠 Development Environment Setup

### System Requirements

- **Rust**: 1.88+ (stable channel)
- **Cargo**: Latest version
- **Git**: For source code management
- **Editor**: VS Code, Vim, Emacs, or any editor with Rust support

### Installing Rust

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or use package manager
# Ubuntu/Debian
sudo apt install rustc cargo

# macOS
brew install rust

# Windows
# Download from https://rustup.rs/
```

### Clone and Setup Project

```bash
# Fork repository on GitHub
# Then clone to local machine
git clone https://github.com/YOUR_USERNAME/thefuck-rs.git
cd thefuck-rs

# Add upstream remote
git remote add upstream https://github.com/haiphamcoder/thefuck-rs.git

# Install dependencies
cargo build

# Run tests to ensure everything works
cargo test
```

### Install Development Tools

```bash
# Rust analyzer (for IDE support)
rustup component add rust-analyzer

# Clippy (linter)
rustup component add clippy

# Rustfmt (code formatter)
rustup component add rustfmt

# Cargo-audit (security audit)
cargo install cargo-audit

# Cargo-tarpaulin (test coverage)
cargo install cargo-tarpaulin
```

## 🔄 Development Workflow

### 1. Create New Branch

```bash
# Ensure you're on main branch and up to date
git checkout main
git pull upstream main

# Create new branch for feature/fix
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 2. Development

- Write code following [code guidelines](#-code-guidelines)
- Commit frequently with clear messages
- Write tests for new code
- Update documentation if needed

### 3. Code Review

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test

# Run tests with coverage
cargo tarpaulin

# Check security
cargo audit

# Build release version
cargo build --release
```

### 4. Commit and Push

```bash
# Add files
git add .

# Commit with conventional commit message
git commit -m "feat: add new rule for docker commands"

# Push to your fork
git push origin feature/your-feature-name
```

### 5. Create Pull Request

1. Go to your GitHub repository
2. Click "New Pull Request"
3. Select your branch
4. Fill out PR template
5. Submit PR

## 🚀 CI/CD Pipeline

The project uses GitHub Actions for continuous integration and deployment. All workflows are located in the `.github/workflows/` directory.

### Automated Checks

When you create a pull request, the following checks will run automatically:

- **Code Formatting**: Ensures code follows Rust formatting standards
- **Linting**: Runs clippy to check for code quality issues
- **Tests**: Runs all unit and integration tests
- **Build**: Compiles the project for multiple platforms
- **Security Audit**: Checks for known vulnerabilities
- **Coverage**: Generates test coverage reports

### Local Development

To run CI checks locally before pushing:

```bash
# Format code
cargo fmt --all -- --check

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all-targets --all-features

# Check builds
cargo check --all-targets --all-features

# Security audit
cargo audit

# Generate documentation
cargo doc --no-deps --all-features
```

### Supported Platforms

The CI pipeline builds and tests on:

- **Linux**: x86_64, aarch64
- **macOS**: x86_64, aarch64  
- **Windows**: x86_64

### Release Process in CI/CD

Releases are automated and triggered by version tags:

- Push a tag like `v1.0.0` to trigger release workflow
- Multi-platform binaries are automatically built
- Release notes are generated from commits
- Assets are uploaded to GitHub releases

For detailed CI/CD documentation, see [`.github/CONTRIBUTING.md`](.github/CONTRIBUTING.md).

## 📝 Code Guidelines

### Project Structure

```text
thefuck-rs/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Library root
│   ├── app/             # Application core
│   ├── cli/             # Command line interface
│   ├── config/          # Configuration management
│   ├── rules/           # Rules implementation
│   ├── shell/           # Shell integration
│   ├── types/           # Core data types
│   └── utils/           # Utility functions
├── tests/               # Integration tests
├── examples/            # Example usage
├── docs/                # Documentation
└── benches/             # Benchmarks
```

### Naming Conventions

- **Files**: snake_case (e.g., `command_parser.rs`)
- **Modules**: snake_case (e.g., `mod command_parser`)
- **Structs**: PascalCase (e.g., `struct CommandParser`)
- **Functions**: snake_case (e.g., `fn parse_command()`)
- **Variables**: snake_case (e.g., `let command_text`)
- **Constants**: SCREAMING_SNAKE_CASE (e.g., `const MAX_RETRIES`)
- **Types**: PascalCase (e.g., `type CommandResult`)

### Code Style

#### Rust Code

```rust
// Use rustfmt to format code
use std::collections::HashMap;
use tokio::sync::mpsc;

/// Represents a shell command with its context
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    /// The original command text
    pub text: String,
    /// The shell that executed the command
    pub shell: Shell,
    /// Command execution timestamp
    pub timestamp: DateTime<Utc>,
}

impl Command {
    /// Creates a new command instance
    pub fn new(text: String, shell: Shell) -> Self {
        Self {
            text,
            shell,
            timestamp: Utc::now(),
        }
    }

    /// Parses the command and returns structured data
    pub fn parse(&self) -> Result<ParsedCommand, ParseError> {
        // Implementation here
    }
}
```

#### Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TheFuckError {
    #[error("Failed to parse command: {0}")]
    ParseError(String),
    
    #[error("Shell not supported: {0}")]
    UnsupportedShell(String),
    
    #[error("No matching rules found")]
    NoRulesFound,
    
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

// Use anyhow for application-level errors
use anyhow::{Result, Context};

pub fn process_command(cmd: &str) -> Result<String> {
    let parsed = parse_command(cmd)
        .context("Failed to parse command")?;
    
    let corrected = find_correction(&parsed)
        .context("Failed to find correction")?;
    
    Ok(corrected)
}
```

#### Async Code

```rust
use tokio::sync::mpsc;

pub async fn process_commands(
    mut rx: mpsc::Receiver<Command>,
) -> Result<()> {
    while let Some(cmd) = rx.recv().await {
        let result = process_single_command(cmd).await?;
        // Handle result
    }
    Ok(())
}
```

### Documentation

#### Code Comments

```rust
/// Calculates the similarity between two commands
/// 
/// Returns a value between 0.0 and 1.0, where 1.0 means
/// the commands are identical.
/// 
/// # Arguments
/// * `cmd1` - First command to compare
/// * `cmd2` - Second command to compare
/// 
/// # Examples
/// ```
/// use thefuck_rs::utils::similarity;
/// 
/// let sim = similarity("git push", "git pull");
/// assert!(sim > 0.5);
/// ```
pub fn similarity(cmd1: &str, cmd2: &str) -> f64 {
    // Implementation
}
```

#### Module Documentation

```rust
//! Command parsing and validation module
//! 
//! This module provides functionality for parsing shell commands
//! and validating their syntax. It supports multiple shell types
//! and provides error handling for malformed commands.
//! 
//! # Examples
//! 
//! ```rust
//! use thefuck_rs::parser::parse_command;
//! 
//! let result = parse_command("git push origin main");
//! assert!(result.is_ok());
//! ```

pub mod lexer;
pub mod parser;
pub mod validator;
```

## 🧪 Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parsing() {
        let cmd = Command::new("git push".to_string(), Shell::Bash);
        let parsed = cmd.parse().unwrap();
        
        assert_eq!(parsed.program, "git");
        assert_eq!(parsed.args, vec!["push"]);
    }

    #[test]
    fn test_invalid_command() {
        let cmd = Command::new("".to_string(), Shell::Bash);
        let result = cmd.parse();
        
        assert!(result.is_err());
    }
}
```

### Integration Tests

```rust
// tests/integration_test.rs
use thefuck_rs::{TheFuckApp, Config};

#[tokio::test]
async fn test_full_workflow() {
    let config = Config::default();
    let app = TheFuckApp::new(config);
    
    let result = app.process_command("git pus").await;
    assert!(result.is_ok());
    
    let corrected = result.unwrap();
    assert_eq!(corrected, "git push");
}
```

### Property-Based Tests

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_command_roundtrip(cmd in "[a-zA-Z0-9 ]{1,100}") {
        let command = Command::new(cmd.clone(), Shell::Bash);
        let parsed = command.parse();
        
        if let Ok(parsed_cmd) = parsed {
            assert_eq!(parsed_cmd.to_string(), cmd);
        }
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with detailed output
cargo test -- --nocapture

# Run specific tests
cargo test test_command_parsing

# Run tests with coverage
cargo tarpaulin

# Run benchmarks
cargo bench
```

## 🐛 Bug Reports

### Creating a Bug Report

When reporting a bug, please provide:

1. **Brief Description**: What error occurred?
2. **Steps to Reproduce**: How to reproduce the error?
3. **Expected Behavior**: What did you expect to happen?
4. **Actual Behavior**: What actually happened?
5. **System Information**:
   - OS: Linux/macOS/Windows
   - Rust version: `rustc --version`
   - TheFuck-rs version: `thefuck --version`
   - Shell: bash/zsh/fish/powershell

### Bug Report Template

```markdown
## Bug Report

### Description
[Brief description of the bug]

### Steps to Reproduce
1. Run command: `thefuck [command]`
2. [Additional steps...]

### Expected Behavior
[What you expected to happen]

### Actual Behavior
[What actually happened]

### Environment
- OS: [Your OS]
- Rust: [rustc --version]
- TheFuck-rs: [version]
- Shell: [Your shell]

### Additional Information
[Any other relevant information]
```

## 💡 Feature Requests

### Creating a Feature Request

When suggesting a new feature:

1. **Feature Description**: What does this feature do?
2. **Motivation**: Why is this feature useful?
3. **Use Cases**: Specific use cases
4. **Implementation Ideas**: Ideas for implementation (if any)

### Feature Request Template

```markdown
## Feature Request

### Description
[Description of the feature]

### Motivation
[Why this feature is needed]

### Use Cases
- [Use case 1]
- [Use case 2]

### Implementation Ideas
[Optional: ideas for implementation]

### Alternatives Considered
[Optional: other approaches you considered]
```

## 📚 Documentation {#documentation}

### Updating Documentation

When adding new features or changing APIs:

1. **README.md**: Update if necessary
2. **API docs**: Add documentation for public APIs
3. **Examples**: Add usage examples
4. **Changelog**: Update CHANGELOG.md

### Writing Documentation

```rust
/// Processes a command and returns possible corrections
/// 
/// This function takes a command string and returns a list of
/// possible corrections based on the available rules.
/// 
/// # Arguments
/// * `command` - The command to process
/// * `config` - Configuration options
/// 
/// # Returns
/// A list of possible corrections, sorted by relevance
/// 
/// # Examples
/// ```
/// use thefuck_rs::{process_command, Config};
/// 
/// let config = Config::default();
/// let corrections = process_command("git pus", &config).await?;
/// 
/// for correction in corrections {
///     println!("{}", correction);
/// }
/// ```
pub async fn process_command(
    command: &str,
    config: &Config,
) -> Result<Vec<Correction>> {
    // Implementation
}
```

## 🎯 Releases

### Release Process

1. **Version bump**: Update version in `Cargo.toml`
2. **Changelog**: Update `CHANGELOG.md`
3. **Tests**: Run all tests
4. **Documentation**: Update docs
5. **Tag**: Create git tag
6. **Publish**: Publish to crates.io

### Conventional Commits

Use [Conventional Commits](https://www.conventionalcommits.org/) for commit messages:

```text
feat: add new docker rule
fix: resolve issue with git push detection
docs: update README with installation instructions
style: format code with rustfmt
refactor: simplify command parsing logic
test: add tests for npm rules
chore: update dependencies
```

### Versioning

Use [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

## 🤝 Code of Conduct

### Expected Behavior

- **Respect**: Respect all contributors
- **Collaboration**: Work together to improve the project
- **Patience**: Everyone can make mistakes
- **Support**: Help others when possible

### Unacceptable Behavior

- Harassment or bullying
- Spam or irrelevant advertising
- Privacy violations
- Destructive behavior

## 📞 Contact

- **Issues**: [GitHub Issues](https://github.com/haiphamcoder/thefuck-rs/issues)
- **Discussions**: [GitHub Discussions](https://github.com/haiphamcoder/thefuck-rs/discussions)
- **Email**: [Your email]

## 🙏 Acknowledgments

Thank you for contributing to TheFuck-rs! Every contribution, big or small, is valuable and appreciated.

---
