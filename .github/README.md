# CI/CD Pipeline

This directory contains the GitHub Actions workflows for the thefuck-rs project.

## Workflows

### 1. CI (`ci.yml`)

Main continuous integration workflow that runs on every push and pull request.

**Jobs:**

- **Check**: Linting, formatting, and build verification
- **Test**: Unit tests and test coverage
- **Build**: Multi-platform builds (Linux, macOS, Windows, ARM64)
- **Audit**: Security vulnerability scanning
- **Docs**: Documentation generation and deployment

**Triggers:**

- Push to `main` or `develop` branches
- Pull requests to `main` or `develop` branches

### 2. Release (`release.yml`)

Automated release workflow that creates GitHub releases with binaries.

**Features:**

- Multi-platform binary builds
- Automatic release notes generation
- Asset upload to GitHub releases

**Triggers:**

- Push of version tags (e.g., `v1.0.0`)

### 3. Dependabot (`dependabot.yml`)

Handles dependency updates from Dependabot.

**Features:**

- Automated testing of dependency updates
- PR commenting for successful updates
- Quality checks before merging

**Triggers:**

- Pull requests from Dependabot

### 4. Pages (`pages.yml`)

Deploys documentation to GitHub Pages.

**Features:**

- Automatic documentation deployment
- Rustdoc generation
- GitHub Pages integration

**Triggers:**

- Push to `main` branch
- Manual workflow dispatch

### 5. Coverage (`coverage.yml`)

Generates and reports code coverage.

**Features:**

- Tarpaulin coverage generation
- Codecov integration
- Coverage artifact storage

**Triggers:**

- Push to `main` or `develop` branches
- Pull requests to `main` or `develop` branches

## Configuration Files

### Dependabot (`dependabot.yml`)

Configures automatic dependency updates:

- Weekly updates for Rust dependencies
- Weekly updates for GitHub Actions
- Automatic PR creation with labels and assignees
- Ignore major version updates for critical dependencies

## Supported Platforms

- **Linux**: x86_64, aarch64
- **macOS**: x86_64, aarch64
- **Windows**: x86_64

## Environment Variables

The following secrets may be required:

- `GITHUB_TOKEN`: Automatically provided by GitHub
- `CODECOV_TOKEN`: For code coverage reporting (optional)

## Local Development

To run CI checks locally:

```bash
# Format code
cargo fmt --all -- --check

# Run clippy
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

## Troubleshooting

### Common Issues

1. **Build failures on specific platforms**: Check if all dependencies support the target platform
2. **Test failures**: Ensure all tests pass locally before pushing
3. **Coverage issues**: Verify tarpaulin is properly configured
4. **Release failures**: Check that version tags follow semantic versioning

### Getting Help

- Check the GitHub Actions logs for detailed error messages
- Review the workflow files for configuration issues
- Ensure all required secrets are properly configured
