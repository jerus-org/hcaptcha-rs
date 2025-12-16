<!--
SPDX-FileCopyrightText: 2022 jerusdp

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Contributing to hcaptcha-rs

Thank you for your interest in contributing to hcaptcha-rs! We welcome contributions from the community and appreciate your help in making this project better. Please also review the project [Governance](GOVERNANCE.md).

## Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to community[@]jerus.ie.

## Getting Started

### Prerequisites

- Rust 1.88 or higher
- Git
- Familiarity with Cargo and Rust development

### Setting Up Your Development Environment

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/hcaptcha-rs.git
   cd hcaptcha-rs
   ```
3. Add the upstream repository as a remote:
   ```bash
   git remote add upstream https://github.com/jerus-org/hcaptcha-rs.git
   ```
4. Build the project:
   ```bash
   cargo build
   ```
5. Run the tests to ensure everything is working:
   ```bash
   cargo test --all
   ```

### Newcomers: Small tasks

If you’re looking for something easy to start with, check the issue lists:
- good first issue: https://github.com/jerus-org/hcaptcha-rs/labels/good%20first%20issue
- help wanted: https://github.com/jerus-org/hcaptcha-rs/labels/help%20wanted

See docs/SMALL_TASKS.md for what qualifies and how to proceed. Maintainers aim to keep a handful of well-scoped, beginner-friendly tasks available.

## How to Contribute

### Reporting Bugs

- Use the [GitHub Issues](https://github.com/jerus-org/hcaptcha-rs/issues) page to report bugs
- Before creating a new issue, please search existing issues to avoid duplicates
- Provide a clear description of the issue, including:
  - Steps to reproduce
  - Expected behavior
  - Actual behavior
  - Your environment (OS, Rust version, etc.)

### Security Vulnerabilities

**Do not report security vulnerabilities through public GitHub issues.**

Please refer to our [Security Policy](SECURITY.md) for instructions on how to report security vulnerabilities responsibly.

### Suggesting Enhancements

- Use the [GitHub Issues](https://github.com/jerus-org/hcaptcha-rs/issues) page to suggest enhancements
- Clearly describe the enhancement and its potential benefits
- Provide examples of how the feature would be used

### Submitting Pull Requests

#### Developer Certificate of Origin (DCO)

We use the Developer Certificate of Origin to certify contributions. Please sign your commits using `-s`:

```bash
git commit -s -m "✨ feat: add X"
```

If you forget, you can amend the latest commit with:

```bash
git commit --amend -s --no-edit
```

You may also enable the GitHub DCO app on your fork to enforce sign-off.

1. **Create a new branch** from `main` for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```
   or
   ```bash
   git checkout -b fix/issue-description
   ```

2. **Make your changes** following the coding standards below

3. **Write tests** for your changes to ensure code coverage

4. **Run the test suite** to ensure all tests pass:
   ```bash
   cargo test --all
   ```

5. **Check for code quality issues**:
   ```bash
   cargo clippy --all-targets --all-features
   ```
   Fix any warnings or errors reported by Clippy.

6. **Format your code**:
   ```bash
   cargo fmt --all
   ```

7. **Commit your changes** with a descriptive commit message prefixed with an appropriate emoji:
   ```bash
   git commit -m "✨ Add new feature for X"
   ```
   
   Common emoji prefixes:
   - ✨ `:sparkles:` - New feature
   - 🐛 `:bug:` - Bug fix
   - 📚 `:books:` - Documentation
   - 🎨 `:art:` - Code style/formatting
   - ♻️ `:recycle:` - Refactoring
   - ✅ `:white_check_mark:` - Tests
   - ⬆️ `:arrow_up:` - Dependency updates
   - 🔒 `:lock:` - Security fixes

8. **Push your branch** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

9. **Create a Pull Request** on GitHub:
   - Navigate to the [hcaptcha-rs repository](https://github.com/jerus-org/hcaptcha-rs)
   - Click "New Pull Request"
   - Select your fork and branch
   - Provide a clear description of your changes
   - Reference any related issues

10. **Request review** from @jerusdp or other maintainers

### Pull Request Guidelines

- **Never commit directly to `main`** - all changes must go through a pull request
- Keep pull requests focused on a single feature or fix
- Write clear, descriptive commit messages
- Ensure all tests pass and code quality checks are clean
- Update documentation as needed
- Add yourself to the contributors list if it's your first contribution

### Pull Requests from Forks

If you're contributing from a forked repository, please note the following:

#### Automated Testing

When you submit a PR from your fork, CircleCI will automatically run a **fork-safe validation workflow** that includes:

- ✅ Code formatting checks (`cargo fmt`)
- ✅ Linting (`cargo clippy`)
- ✅ Build verification for all packages
- ✅ All test suites (unit, integration, doc, wasm)
- ✅ Dependency audits

These tests run with **no access to secrets**, ensuring your PR is tested safely.

#### Restricted Tests

Some checks require restricted access and will not run automatically on fork PRs:

- **SonarCloud analysis**: Code quality scans (requires maintainer privileges)
- **Code coverage uploads**: Coverage reports (requires tokens)
- **PRLOG updates**: See below

These will show as "Unauthorized" in CircleCI - this is **expected and normal**.

#### PRLOG (Changelog) Updates

**Important**: The PRLOG.md changelog is **not updated in your PR branch**. Instead:

1. Your PR goes through validation and review
2. After merge to main, an automated job updates PRLOG.md
3. The changelog entry appears on the main branch

This approach maintains security and consistency for all contributors.

#### Full Validation (When Needed)

For complex or high-risk changes, maintainers may create a branch in the main repository to run full validation including:

- SonarCloud security and quality analysis
- Code coverage with upload to Codecov
- All security scans

You'll be notified if this is needed for your PR.

#### Review Timeline

- **Automated tests**: 10-15 minutes
- **Maintainer review**: Typically 2-3 business days
- **Full validation** (if needed): Additional 10-15 minutes

For more details, see [docs/FORK_PR_REVIEW.md](docs/FORK_PR_REVIEW.md).

### Coding Standards

#### Language and style
- Language: Rust 2021, MSRV = 1.88 (see `Cargo.toml`).
- Style: rustfmt default style; code must be formatted (`cargo fmt --all --check`).
- Linting: clippy-clean (no warnings) across all targets/features (`cargo clippy --all-targets --all-features`).

#### Rust API & design conventions
- Follow the Rust API Guidelines (naming, error handling, docs, semver).
- Public items should have rustdoc with examples where sensible; examples compile as doctests in CI.

#### Error handling and panics
- Library code must not panic on valid inputs; avoid `unwrap`/`expect` outside tests/examples.
- Prefer `Result<_, Error>` with meaningful error variants.

#### Unsafe code policy
- Unsafe Rust is disallowed unless strictly necessary and justified with inline comments and tests.
- If required, limit scope to the smallest module and document safety invariants.

#### Dependencies and security
- Prefer widely-used, well-maintained crates; compatible licenses only (MIT/Apache-2.0).
- Keep dependencies updated via Renovate; handle issues per SECURITY.md.

#### Commit/PR requirements
- CI must be green: fmt, clippy, tests, and doctests (README/examples).
- DCO sign-off is required for every commit (`git commit -s`).

#### Build flags and environment

- The project uses Cargo. Standard environment variables such as `RUSTFLAGS`, `RUSTDOCFLAGS`, `CC`, and `CFLAGS` are honored by the Rust toolchain and build scripts.
- The build does not rely on recursive cross-directory builds or custom wrappers.
- CI does not strip debug information during builds; release artifacts preserve standard debug info unless users choose otherwise.
- For deterministic, reproducible builds, see [docs/REPRODUCIBLE_BUILDS.md](docs/REPRODUCIBLE_BUILDS.md) for the exact flags, environment variables, and container pinning used in CI.

### Testing

#### Test Policy

The project maintains a policy that as major new functionality is added to the software, tests of that functionality must be added to the automated test suite. This ensures that:
- New features are properly validated
- Regressions can be detected early
- Code quality and reliability are maintained over time

#### Testing Guidelines

- Write unit tests for all new functionality
- Write integration tests where appropriate
- Check for duplicated code in test functions and add helper functions when duplications are found
- Aim for high test coverage
- All tests must pass before submitting a pull request

The project includes multiple test suites in the workspace:
- `test-suite-default` - Default feature tests
- `test-suite-enterprise` - Enterprise feature tests
- `test-suite-native-only` - Native TLS only tests
- `test-suite-no-default` - Tests with no default features
- `test-suite-rustls-only` - RustLS only tests
- `test-suite-trace` - Tracing feature tests
- `test-suite-cli` - CLI tests
- `test-wasm` - WebAssembly tests

### Documentation

- Document all public APIs with rustdoc comments
- Include examples in documentation where helpful
- Update the README.md if your changes affect usage
- Keep PRLOG.md updated with notable changes

### Licensing

- All contributions must be licensed under MIT OR Apache-2.0
- Include SPDX license identifiers in new files:
  ```rust
  // SPDX-FileCopyrightText: 2022 jerusdp
  //
  // SPDX-License-Identifier: MIT OR Apache-2.0
  ```

## Project Structure

The project is organized as a Cargo workspace:

- `hcaptcha/` - Core library
- `hcaptcha-cli/` - Command-line interface
- `hcaptcha_derive/` - Derive macros
- `mock-verifier/` - Mock verification server for testing
- `test-suite-*/` - Various test suites
- `test-wasm/` - WebAssembly tests

## Development Workflow

1. Sync your fork with upstream regularly:
   ```bash
   git fetch upstream
   git checkout main
   git merge upstream/main
   ```

2. Keep your feature branch up to date:
   ```bash
   git checkout feature/your-feature-name
   git rebase main
   ```

3. Before submitting a PR, ensure:
   - [ ] All tests pass (`cargo test --all`)
   - [ ] Code is formatted (`cargo fmt --all`)
   - [ ] No clippy warnings (`cargo clippy --all-targets --all-features`)
   - [ ] Documentation is updated
   - [ ] Commit messages follow the emoji prefix convention

## Getting Help

- Check the [documentation](https://docs.rs/hcaptcha)
- Search [existing issues](https://github.com/jerus-org/hcaptcha-rs/issues)
- Ask questions in a new issue or discussion

## Recognition

Contributors are recognized in several ways:
- Listed in project documentation
- Mentioned in release notes
- GitHub contributor graph

## Support the Project

If you find this project useful, consider:
- ⭐ Starring the repository
- ☕ [Buying the maintainer a coffee](https://buymeacoffee.com/jerusdp)
- 💝 [Sponsoring on GitHub](https://github.com/sponsors/jerusdp)

Thank you for contributing to hcaptcha-rs!
