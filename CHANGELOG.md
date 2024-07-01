# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Security

- Dependencies: update fossa-contrib/fossa-action digest to 8429059(pr [#1006])

## [2.4.0] - 2024-07-01

### Changed

- ci-pr change entry and release building(pr [#990](https://github.com/jerus-org/hcaptcha-rs/pull/990))
- ci-adopt the standard commands and jobs from the toolkit(pr [#996])
- ci-split script into three and select continuation direct to success if bot(pr [#997])
- chore-disable automatic updates for 'jerus-org/circleci-toolkit' package(pr [#1000])
- chore(renovate.json)-change 'enable' key to 'enabled' and set its value to false(pr [#1001])
- chore(scorecards-analysis.yml)-update workflow name to 'Scorecard analysis workflow'(pr [#1002])
- refactor-change file name from CHANGES.md to CHANGELOG.md in pre-release-replacements(pr [#1004])

### Fixed

- modify version search pattern in src/lib.rs file(pr [#1005])

### Security

- Dependencies: update fossa-contrib/fossa-action digest to 524596f(pr [#989](https://github.com/jerus-org/hcaptcha-rs/pull/989))
- Dependencies: update actions/checkout digest to 692973e(pr [#988](https://github.com/jerus-org/hcaptcha-rs/pull/988))
- Dependencies: update rust crate itertools to 0.13.0(pr [#987](https://github.com/jerus-org/hcaptcha-rs/pull/987))
- Dependencies: update ossf/scorecard-action digest to 0a8153a(pr [#986](https://github.com/jerus-org/hcaptcha-rs/pull/986))
- Dependencies: update github/codeql-action digest to ce5603b(pr [#985](https://github.com/jerus-org/hcaptcha-rs/pull/985))
- Dependencies: update fossa-contrib/fossa-action digest to ca0599a(pr [#991](https://github.com/jerus-org/hcaptcha-rs/pull/991))
- Dependencies: update fossa-contrib/fossa-action digest to fd87c8e(pr [#994])
- Dependencies: update rust crate lambda_runtime to 0.12.0(pr [#993])
- Dependencies: update github/codeql-action digest to 9b7c22c(pr [#992])
- Dependencies: update ossf/scorecard-action digest to 09f6ba3(pr [#995])
- Dependencies: update fossa-contrib/fossa-action digest to a024aa3(pr [#999])
- Dependencies: update github/codeql-action digest to de94575(pr [#998])
- Dependencies: update github/codeql-action action to v3.25.11(pr [#1003])

## [2.3.1] - 2024-01-27

### Fixed

- FIX: Length for validation of v2 secret ([#842](https://github.com/jerusdp/hcaptcha-rs/issues/842))
- Update dependencies

## [2.3.0] - 2024-01-07

### Added

- Add support for validating new secret format in Extended validation (`ext`) feature

### Fixed

- Update dependencies

## [2.2.2] - 2023-04-09

### Fixed

- Update dependencies
- prepare for clippy::uninlined_format_args to be style lint (warn by default)
- adapt to breaking changes in syn 2.0

## [2.2.1] - 2023-01-25

### Added

- Add enterprise features to hcaptcha
- Integration testing with hcaptcha.com
- Additional test suites for feature scenarios

### Changed

- Point README badge to circle ci and update min version to 1.56
- Documentation in samples
- Update Minium Rust Version to 1.60
- Test suites for feature scenarios
- Test suite file
- Do not check response score reason

### Fixed

- Update dependencies
- Replace fakeit with mockd
- Update dependencies

## [2.2.0] - 2022-11-17

### Added

- Add enterprise features to hcaptcha
- Integration testing with hcaptcha.com
- Additional test suites for feature scenarios
- Features to choose reqwest backends for TLS (thanks [@Lunarequest])

### Changed

- Point README badge to circle ci and update min version to 1.60
- Documentation in samples
- Minimum rust version 1.60
- Test suites for feature scenarios

### Fixed

- Update dependencies
- Replace fakeit with mockd
- Fix directory name .circleci

## [2.1.1] - 2022-07-04

### Changed

- Update to edition 2021
- Update dependencies

## [2.0.1] - 2021-10-27

### Added

- trait_implementation example
- trait implementation
- derive macro

### Fixed

- Spelling errors

## [2.0.0] - 2021-07-09

- *Notes**
- Validation of builder inputs*

Validation of secret and response inputs makes hcaptcha::new(secret, response) fallible. The function returns a result to address any validation failure.

Basic validation for both inputs ensures that the value is not empty or composed of only whitespace.

Extended validation for the secret key requires it to conform to "0x" followed by a 40 character hexadecimal string. The extended validation is feature flagged and can be disabled. The flag is enabled by default. To disable load the library with default-features = false.

The input to .sitekey(sitekey) has been changed to validate that the string slice supplied is a valid UUID.

The input to the .remoteip(remoteip) has been changed to validate that the string slice supplier is a valid ipv4 or ipv6 address.
- Logging / Tracing*

The previous version provided logging behind a feature flag. The log crate has been removed and replaced with tracing. Tracing has been instrumented for all public functions. Tracing is enabled by selected the "trace" feature.

Tracing is enabled at the info logging level for public methods. Additional tracing instrumentation and messages are available at the Debug log level.

The trace crates log feature is enabled so that log records are
emitted if a tracing subscriber is not found.
### Changed

- Rename user_ip and site_key to conform to Hcaptcha API documentation (remoteip and sitekey)
- Restore lambda_runtime as crate has been updated
- Validate client response before submission to Hcaptcha API
- Validate secret before submission to Hcaptcha API
- Validate remoteip as a v4 or v6 IP address before submission to Hcaptcha API
- Validate sitekey as a UUID before submission to Hcaptcha API
- Constrain sitekey string to a Uuid
- Place methods to access Enterprise only response data behind "enterprise" feature flag
- Replace logging with tracing; remove logging feature flag
- Revise documentation and enhance examples
- Adopt Request/Response/Error and Trait structure
- Struct HcaptchaCaptcha for client response (response, sitekey and remoteip) and new_with(captcha) to construct request using the HcaptchaCaptcha struct.

## [1.0.1] - 2021-03-03

### Changed

- Replace lambda_runtime with lamedh_runtime to avoid security issue RUSTSEC-2021-0020 in hyper 0.12.36.

[@Lunarequest]: https://github.com/Lunarequest/Lunarequest
[#994]: https://github.com/jerus-org/hcaptcha-rs/pull/994
[#993]: https://github.com/jerus-org/hcaptcha-rs/pull/993
[#992]: https://github.com/jerus-org/hcaptcha-rs/pull/992
[#995]: https://github.com/jerus-org/hcaptcha-rs/pull/995
[#996]: https://github.com/jerus-org/hcaptcha-rs/pull/996
[#999]: https://github.com/jerus-org/hcaptcha-rs/pull/999
[#997]: https://github.com/jerus-org/hcaptcha-rs/pull/997
[#998]: https://github.com/jerus-org/hcaptcha-rs/pull/998
[#1000]: https://github.com/jerus-org/hcaptcha-rs/pull/1000
[#1001]: https://github.com/jerus-org/hcaptcha-rs/pull/1001
[#1002]: https://github.com/jerus-org/hcaptcha-rs/pull/1002
[#1004]: https://github.com/jerus-org/hcaptcha-rs/pull/1004
[#1005]: https://github.com/jerus-org/hcaptcha-rs/pull/1005
[#1003]: https://github.com/jerus-org/hcaptcha-rs/pull/1003
[#1006]: https://github.com/jerus-org/hcaptcha-rs/pull/1006
[Unreleased]: https://github.com/jerus-org/hcaptcha-rs/compare/2.4.0...HEAD
[2.4.0]: https://github.com/jerus-org/hcaptcha-rs/compare/2.3.1...2.4.0
[2.3.1]: https://github.com/jerus-org/hcaptcha-rs/compare/2.3.0...2.3.1
[2.3.0]: https://github.com/jerus-org/hcaptcha-rs/compare/2.2.2...2.3.0
[2.2.2]: https://github.com/jerus-org/hcaptcha-rs/compare/2.2.1...2.2.2
[2.2.1]: https://github.com/jerus-org/hcaptcha-rs/compare/2.2.0...2.2.1
[2.2.0]: https://github.com/jerus-org/hcaptcha-rs/compare/2.1.1...2.2.0
[2.1.1]: https://github.com/jerus-org/hcaptcha-rs/compare/2.0.1...2.1.1
[2.0.1]: https://github.com/jerus-org/hcaptcha-rs/compare/2.0.0...2.0.1
[2.0.0]: https://github.com/jerus-org/hcaptcha-rs/compare/1.0.1...2.0.0
[1.0.1]: https://github.com/jerus-org/hcaptcha-rs/releases/tag/1.0.1
