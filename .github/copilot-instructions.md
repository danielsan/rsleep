# Copilot Coding Agent Instructions

## Repository Overview

This repository contains **rsleep**, a modern Rust-based alternative to the classic `sleep` command with a beautiful progress bar visualization. It's a command-line utility that provides visual feedback during wait times, making it perfect for scripts, demonstrations, and interactive use cases.

**Key Technologies:**
- Language: Rust (Edition 2021, requires Rust 1.70+)
- Main Dependency: `indicatif` v0.18.1 for progress bar functionality
- Build Tool: Cargo
- License: MIT

## Project Structure

```
rsleep/
├── src/
│   └── main.rs           # Main application logic and CLI entry point
├── Cargo.toml            # Project manifest and dependencies
├── Cargo.lock            # Locked dependency versions
├── .github/
│   └── workflows/        # CI/CD workflows (release and publish)
└── README.md             # User-facing documentation
```

## Building and Testing

### Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run the application (development)
cargo run -- <seconds>

# Run the application (release)
./target/release/rsleep <seconds>
```

### Testing

Currently, this project does not have automated tests. When adding tests:
- Place unit tests in the same file using `#[cfg(test)]` modules
- Place integration tests in a `tests/` directory at the project root
- Run tests with: `cargo test`
- Ensure all tests pass before submitting PRs

### Linting and Formatting

```bash
# Check code formatting
cargo fmt -- --check

# Format code automatically
cargo fmt

# Run Clippy for lints
cargo clippy -- -D warnings

# Run Clippy with fixes
cargo clippy --fix
```

## Coding Standards and Guidelines

### Code Style
- Follow standard Rust formatting (enforced by `rustfmt`)
- Use meaningful variable names and clear function signatures
- Keep functions focused and single-purpose
- Add comments for complex logic, but prefer self-documenting code

### Error Handling
- Use proper error messages that help users understand what went wrong
- Exit with appropriate status codes (1 for errors)
- Validate input before processing (e.g., check for negative durations)

### Dependencies
- Minimize external dependencies to keep the binary lightweight
- Justify any new dependencies in PRs
- Check for security vulnerabilities before adding new crates

### Performance Considerations
- The release profile is optimized for size (`opt-level = "s"`)
- LTO (Link Time Optimization) is enabled for smaller binaries
- Debug symbols are stripped in release builds
- Maintain fast startup times (this is a CLI tool)

## Contribution Guidelines

### Pull Request Requirements
1. **Code Quality:**
   - Code must compile without warnings
   - Follow Rust best practices and idioms
   - Pass `cargo clippy` without warnings

2. **Documentation:**
   - Update README.md if adding features or changing behavior
   - Add inline comments for complex logic
   - Update usage examples if CLI interface changes

3. **Testing:**
   - Manually test the application with various inputs
   - Test edge cases (0 seconds, very large values, decimal values)
   - Verify the progress bar renders correctly

4. **Commits:**
   - Write clear, descriptive commit messages
   - Keep commits focused and atomic
   - Use conventional commit format when appropriate (feat:, fix:, docs:, etc.)

## Application Behavior

### Core Functionality
- Accepts a single argument: duration in seconds (supports decimals)
- Displays an animated progress bar during the sleep period
- Shows: spinner, elapsed time, progress bar, position/total steps, and ETA
- Uses 0.25-second intervals (4 steps per second) for smooth updates
- Completes with a "Done" message

### Input Validation
- Must accept exactly one argument (the duration)
- Duration must be a valid number (integer or decimal)
- Duration must be positive (>= 0)
- Invalid input should show helpful error messages and usage information

### Progress Bar Features
- Green spinner animation
- Cyan/blue progress bar with `#>-` characters
- Precise elapsed time display
- Estimated time remaining (ETA)
- Position tracking (current step / total steps)

## Common Tasks

### Adding a New Feature
1. Consider if the feature aligns with the tool's simple, focused purpose
2. Update the CLI argument parsing if needed
3. Implement the feature with minimal complexity
4. Update README.md with usage examples
5. Test thoroughly with various inputs

### Fixing a Bug
1. Reproduce the bug and understand the root cause
2. Add a test case if possible to prevent regression
3. Fix the issue with minimal changes
4. Verify the fix doesn't break existing functionality
5. Update documentation if the bug fix changes behavior

### Updating Dependencies
1. Check for security advisories before updating
2. Test thoroughly after updates (especially `indicatif`)
3. Update Cargo.lock by running `cargo update`
4. Document any breaking changes in the PR

## Release Process

Releases are managed through GitHub Actions:
- **release.yml**: Handles building and releasing binaries
- **publish.yml**: Publishes to crates.io

When preparing a release:
1. Update version in Cargo.toml
2. Update CHANGELOG if it exists
3. Tag the release following semantic versioning (vX.Y.Z)
4. CI will handle building and publishing

## Security and Safety

- Never add dependencies with known vulnerabilities
- Validate all user input before processing
- Avoid panics in production code (use proper error handling)
- Keep the binary minimal to reduce attack surface
- Review all dependency updates for security implications

## Areas to Be Cautious With

- **Progress Bar Rendering**: Changes to the progress bar template or update logic should be tested thoroughly
- **Time Calculations**: Floating-point arithmetic for duration calculations needs careful handling
- **Thread Sleep**: The sleep intervals affect visual smoothness and accuracy
- **Binary Size**: Keep the release binary small and fast-loading

## Acceptance Criteria for Copilot

When creating pull requests:
- All code must compile without errors or warnings
- The application must function correctly with various inputs (0.5s, 1s, 10s, 100s, etc.)
- Error messages must be clear and helpful
- Changes must not break existing functionality
- Documentation must be updated to reflect any changes
- Keep changes minimal and focused on the specific issue or feature
- Consider backwards compatibility for published crate users

## Questions or Unclear Requirements?

If you encounter ambiguity or need clarification:
- Leave a comment in the PR describing the uncertainty
- Propose multiple solutions if applicable
- Ask for human review before making significant architectural changes
- Err on the side of simplicity and minimal changes
