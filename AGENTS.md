# AGENTS.md - AI Agent Guide for rsleep

## Overview

**rsleep** is a modern Rust-based alternative to the classic `sleep` command with a beautiful progress bar visualization. This command-line utility provides visual feedback during wait times, making it perfect for scripts, demonstrations, and interactive use cases.

**Key Information:**
- **Language**: Rust (Edition 2021, requires Rust 1.70+)
- **Main Dependency**: `indicatif` v0.18.1 for progress bar functionality
- **Build Tool**: Cargo (Rust's package manager)
- **License**: MIT
- **Repository**: https://github.com/danielsan/rsleep

## Quick Start for Agents

### Essential Commands

```bash
# Navigate to repository
cd /home/runner/work/rsleep/rsleep

# Build (development)
cargo build

# Build (release - optimized)
cargo build --release

# Run the application
cargo run -- <seconds>
./target/release/rsleep <seconds>

# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings

# Fix linting issues automatically
cargo clippy --fix
```

### Testing the Application

Currently, this project does **NOT** have automated tests. To manually test:

```bash
# Test basic functionality
./target/debug/rsleep 1        # 1 second
./target/debug/rsleep 2.5      # 2.5 seconds with decimal support
./target/debug/rsleep 0        # Edge case: 0 seconds

# Test error handling
./target/debug/rsleep          # Missing argument (should error)
./target/debug/rsleep -1       # Negative duration (should error)
./target/debug/rsleep abc      # Invalid input (should error)
```

## Project Structure

```
rsleep/
├── src/
│   └── main.rs           # Main application logic (CLI entry point)
├── Cargo.toml            # Project manifest and dependencies
├── Cargo.lock            # Locked dependency versions
├── .github/
│   ├── workflows/        # CI/CD workflows (release, publish, and security)
│   └── copilot-instructions.md  # Detailed development guidelines
├── README.md             # User-facing documentation
├── LICENSE               # MIT License
└── AGENTS.md             # This file
```

## Code Architecture

### Main Application Flow (src/main.rs)

1. **Argument Parsing**: Accepts exactly one argument (duration in seconds)
2. **Input Validation**: 
   - Checks argument count
   - Parses as f64 (supports decimals)
   - Validates non-negative value
3. **Progress Bar Setup**:
   - Calculates total steps (4 steps per second = 0.25s intervals)
   - Creates progress bar with custom styling
   - Template: `{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})`
   - Progress characters: `#>-`
4. **Execution Loop**:
   - Sleeps in 0.25-second intervals
   - Updates progress bar after each interval
   - Breaks when elapsed time reaches target duration
5. **Completion**: Shows "Done" message

### Key Implementation Details

- **Time Precision**: Uses 0.25-second intervals for smooth visual updates
- **Step Calculation**: `total_steps = (seconds * 4.0).ceil() as u64`
- **Progress Bar**: Uses `indicatif` crate for rendering
- **Error Handling**: Exits with status code 1 on errors with descriptive messages

## Common Tasks for Agents

### Adding a New Feature

1. **Assess Alignment**: Ensure the feature aligns with rsleep's simple, focused purpose
2. **Minimal Changes**: Keep changes surgical and focused
3. **Update CLI Parsing**: Modify argument handling in `main()` if needed
4. **Test Manually**: Run with various inputs to verify behavior
5. **Update Documentation**: Modify README.md with usage examples
6. **Maintain Simplicity**: Avoid adding complexity or unnecessary dependencies

### Fixing Bugs

1. **Reproduce**: Understand the bug by testing with the exact scenario
2. **Minimal Fix**: Change only what's necessary to fix the issue
3. **Validate**: Test the fix doesn't break existing functionality
4. **Document**: Update README.md if behavior changes

### Code Style Changes

1. **Format First**: Run `cargo fmt` to auto-format code
2. **Lint**: Run `cargo clippy -- -D warnings` to catch issues
3. **Auto-fix**: Use `cargo clippy --fix` when possible
4. **Manual Review**: Check that auto-fixes are correct

### Updating Dependencies

⚠️ **Important**: Always check security before updating dependencies

```bash
# Check for security vulnerabilities (if cargo-audit is available)
cargo audit

# Update dependencies
cargo update

# Update specific dependency
cargo update -p indicatif

# Rebuild and test
cargo build --release
./target/release/rsleep 5
```

## Important Constraints and Guidelines

### DO

✅ **Follow Rust Best Practices**
- Use idiomatic Rust patterns
- Leverage the standard library when possible
- Write self-documenting code with meaningful names

✅ **Maintain Performance**
- Keep binary size small (release profile uses `opt-level = "s"`)
- Ensure fast startup times (this is a CLI tool)
- Profile uses LTO and strips debug symbols

✅ **Validate Input**
- Always validate user input before processing
- Provide clear, actionable error messages
- Exit with appropriate status codes (1 for errors)

✅ **Keep Dependencies Minimal**
- Current dependency: only `indicatif` v0.18.1
- Justify any new dependencies thoroughly
- Check for security vulnerabilities before adding

✅ **Update Documentation**
- Modify README.md when changing behavior or adding features
- Keep usage examples current and accurate
- Add inline comments only for complex logic

### DO NOT

❌ **Do Not Add Unnecessary Complexity**
- Keep the tool simple and focused
- Avoid feature creep
- Resist adding configuration files or extensive options

❌ **Do Not Break Existing Functionality**
- Maintain backward compatibility
- Test with various inputs before committing
- Respect the existing CLI interface

❌ **Do Not Add Tests Without Discussion**
- The project currently has no test infrastructure
- Adding tests would require justification and planning
- Manual testing is currently the standard

❌ **Do Not Modify Release Configuration**
- The `[profile.release]` section is optimized for size
- Changes could increase binary size significantly
- LTO, stripping, and opt-level are carefully tuned

❌ **Do Not Remove or Skip Linting**
- Always run `cargo clippy` before submitting changes
- Code must compile without warnings
- Follow `rustfmt` formatting standards

## Edge Cases and Special Scenarios

### Input Edge Cases to Consider

1. **Zero Duration**: `rsleep 0` - should complete immediately
2. **Very Small Duration**: `rsleep 0.1` - should handle sub-second correctly
3. **Large Duration**: `rsleep 1000` - should work but takes real time
4. **Decimal Precision**: `rsleep 2.5` - must support decimal seconds
5. **Invalid Input**: 
   - Negative numbers: `rsleep -5` → Error
   - Non-numeric: `rsleep abc` → Error
   - Missing argument: `rsleep` → Error
   - Multiple arguments: `rsleep 1 2` → Error

### Progress Bar Rendering Considerations

⚠️ **Be Cautious When Modifying:**
- Progress bar template string (line 31-32 in main.rs)
- Progress characters (`#>-`)
- Update intervals (currently 0.25s)
- Step calculation formula (`seconds * 4.0`)

Changes to these can affect:
- Visual appearance and smoothness
- Time accuracy
- Terminal compatibility

### Platform Considerations

- **Cross-platform**: Should work on Linux, macOS, and Windows
- **Terminal Support**: Relies on ANSI escape codes (handled by `indicatif`)
- **No Special Permissions**: Doesn't require root/admin access

## Security Considerations

🔒 **Security Best Practices:**

1. **Input Validation**: Always validate user input (already implemented)
2. **Dependency Security**: Check for vulnerabilities before updates
3. **No Unsafe Code**: Avoid `unsafe` blocks unless absolutely necessary
4. **Minimal Attack Surface**: Small binary with few dependencies reduces risk
5. **Error Handling**: Use proper error handling, avoid panics in production
6. **CodeQL Analysis**: Automated security scanning runs via GitHub Actions (codeql.yml)

## Troubleshooting Guide

### Build Failures

**Issue**: `cargo build` fails
```bash
# Solution 1: Update Rust toolchain
rustup update

# Solution 2: Clean and rebuild
cargo clean
cargo build
```

**Issue**: Dependency resolution errors
```bash
# Solution: Remove Cargo.lock and rebuild
rm Cargo.lock
cargo build
```

### Runtime Issues

**Issue**: Progress bar doesn't render correctly
- Check terminal supports ANSI escape codes
- Test in different terminal emulators
- Verify `indicatif` version compatibility

**Issue**: Timing is inaccurate
- Note: Small inaccuracies expected due to 0.25s intervals
- System load can affect sleep accuracy
- This is normal behavior for sleep-based timing

## Contributing as an Agent

### Pull Request Checklist

Before submitting changes:

- [ ] Code compiles without errors or warnings (`cargo build`)
- [ ] Code passes clippy lints (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Manual testing completed with various inputs
- [ ] Edge cases tested (0, decimals, errors)
- [ ] README.md updated if behavior changed
- [ ] Changes are minimal and focused
- [ ] No unnecessary files committed (check with `git status`)

### Commit Message Guidelines

Use clear, descriptive commit messages:

```
Good Examples:
- "Fix negative duration validation"
- "Update progress bar color scheme"
- "Add support for fractional seconds"

Poor Examples:
- "fix bug"
- "changes"
- "update"
```

### What Makes a Good Change

✨ **Good Changes:**
- Fix a specific bug with minimal code modification
- Add a small, focused feature that aligns with the tool's purpose
- Improve error messages for better user experience
- Update documentation for clarity
- Optimize performance without changing behavior

⚠️ **Changes to Avoid:**
- Large refactorings without clear benefit
- Adding complex configuration systems
- Introducing new dependencies without strong justification
- Breaking backward compatibility
- Removing or modifying working code unnecessarily

## Release Process

Releases are automated through GitHub Actions:

- **release.yml**: Builds and publishes binaries for multiple platforms
- **publish.yml**: Publishes the crate to crates.io
- **codeql.yml**: Runs CodeQL security analysis for Rust code

As an agent, you typically **DO NOT** need to manage releases. If a release is needed:

1. Version updates should be discussed with maintainers
2. Semantic versioning is used (vX.Y.Z)
3. CI handles the actual release process

## Getting Help

If you encounter ambiguity or need clarification:

1. **Comment in PR**: Describe the uncertainty clearly
2. **Propose Options**: Suggest multiple solutions if applicable
3. **Ask for Review**: Request human input for significant architectural changes
4. **Default to Simplicity**: When in doubt, choose the simpler approach
5. **Document Assumptions**: Explain your reasoning in commit messages

## Summary for Quick Reference

| Task | Command |
|------|---------|
| Build | `cargo build` |
| Release Build | `cargo build --release` |
| Run | `./target/debug/rsleep <seconds>` |
| Format | `cargo fmt` |
| Lint | `cargo clippy -- -D warnings` |
| Clean | `cargo clean` |
| Update Deps | `cargo update` |

**Key Files:**
- `src/main.rs` - All application logic
- `Cargo.toml` - Dependencies and metadata
- `README.md` - User documentation
- `.github/copilot-instructions.md` - Detailed dev guidelines

**Key Principles:**
1. Keep changes minimal and focused
2. Maintain simplicity and speed
3. Validate all user input
4. Test manually before committing
5. Update documentation when needed
6. Follow Rust best practices
7. Check security implications

---

**Last Updated**: 2025-11-05
**Maintained By**: Repository maintainers and contributors
