<!--
SPDX-FileCopyrightText: 2025 jerusdp

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# hcaptcha-rs Roadmap

This document outlines the planned development direction for the hcaptcha-rs project over the next 12 months and beyond.

## Project Vision

hcaptcha-rs aims to provide a robust, secure, and easy-to-use Rust library for hCaptcha verification in backend services, with support for multiple platforms including native, WebAssembly, and serverless environments.

## Version Support Policy

The project follows semantic versioning (SemVer) and maintains the following support policy:

- **Current Major Version (3.x.x)**: Fully supported with bug fixes, security updates, and new features
- **Previous Major Versions (< 3.0)**: No longer supported
- **Minimum Rust Version**: 1.88+ (updated as needed to leverage new language features)

For security vulnerabilities, see [SECURITY.md](SECURITY.md) for our response and disclosure policy.

## 2025 Roadmap

### Q1 2025 (January - March) ✅ Completed

- ✅ Maintain stable 3.x releases with dependency updates
- ✅ Add comprehensive CONTRIBUTING.md documentation
- ✅ Add SPDX license headers across the codebase
- ✅ Improve code quality with clippy and formatting standards
- ✅ Add explicit test policy documentation

### Q2 2025 (April - June)

#### Maintenance & Quality
- Continue regular dependency updates via Renovate
- Maintain high test coverage (>80%)
- Continue OpenSSF Scorecard compliance
- Achieve OpenSSF Best Practices passing badge

#### Documentation
- Enhance API documentation with more examples
- Improve error handling documentation
- Add architecture documentation
- Create migration guides for common use cases

### Q3 2025 (July - September)

#### Features
- Explore additional hCaptcha API features as they become available
- Investigate performance optimizations for high-throughput scenarios
- Consider adding structured logging support improvements
- Evaluate request timeout and retry configuration enhancements

#### Platform Support
- Continue WebAssembly support and testing
- Maintain compatibility with serverless platforms (AWS Lambda, etc.)
- Test with latest Rust stable releases

### Q4 2025 (October - December)

#### Community & Ecosystem
- Gather community feedback on API ergonomics
- Evaluate feature requests from the community
- Consider framework integration examples (Actix, Axum, Rocket, etc.)
- Improve beginner-friendly documentation

#### Security
- Regular security audits and dependency reviews
- Continue automated security scanning via OSSF Scorecard
- Maintain vulnerability disclosure process

## Beyond 2025

### Potential Future Features (Not Committed)

These are areas of interest that may be explored based on community feedback and maintenance capacity:

- **Enhanced Enterprise Support**: Additional enterprise-specific features as the hCaptcha Enterprise API evolves
- **Async Runtime Flexibility**: Potential support for alternative async runtimes
- **Metrics & Observability**: Built-in metrics collection for monitoring verification patterns
- **Rate Limiting**: Optional client-side rate limiting utilities
- **Caching**: Optional response caching strategies for improved performance
- **Testing Utilities**: Enhanced mock server capabilities for integration testing

### Platform & Ecosystem Evolution

- Monitor and adapt to changes in the hCaptcha API
- Stay current with Rust ecosystem best practices
- Evaluate opportunities for collaboration with related projects
- Consider expanding platform support based on user demand

## Maintenance Commitments

### What We Will Continue To Do

1. **Security**: Promptly address security vulnerabilities in current major version
2. **Dependencies**: Keep dependencies up-to-date via automated tooling (Renovate)
3. **Quality**: Maintain high code quality standards with clippy and rustfmt
4. **Testing**: Maintain comprehensive test coverage across all features
5. **Documentation**: Keep documentation current and accurate
6. **Community**: Respond to issues and pull requests in a timely manner

### What We Will Not Do

1. **Breaking Changes**: No breaking changes within the 3.x.x version series
2. **Old Version Support**: No backports to versions prior to 3.x
3. **Feature Bloat**: Avoid adding features that significantly increase complexity without clear value
4. **Non-Rust Implementations**: This project focuses solely on Rust

## Contributing to the Roadmap

Community input is valuable! If you have suggestions for the roadmap:

1. **For Bug Reports**: Open an issue on [GitHub Issues](https://github.com/jerus-org/hcaptcha-rs/issues)
2. **For Feature Requests**: Open an issue with the `enhancement` label
3. **For Discussions**: Use GitHub Discussions for broader topics
4. **For Security**: Follow the process in [SECURITY.md](SECURITY.md)

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for detailed contribution guidelines.

## Roadmap Changes

This roadmap is a living document and may be updated based on:

- Community feedback and feature requests
- Changes in the hCaptcha API
- Rust ecosystem evolution
- Maintainer capacity and priorities
- Security requirements

Major changes to the roadmap will be announced via GitHub releases and discussions.

## Contact & Support

- **Maintainer**: Jeremiah Russell (@jerusdp)
- **Repository**: https://github.com/jerus-org/hcaptcha-rs
- **Documentation**: https://docs.rs/hcaptcha
- **Discussions**: https://github.com/jerus-org/hcaptcha-rs/discussions

---

*Last Updated: December 2025*
