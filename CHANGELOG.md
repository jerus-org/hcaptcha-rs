# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- chore-update CircleCI config and renovate settings(pr [#1035])

## [2.4.9] - 2024-09-07

### Fixed

- deps: update actions/upload-artifact action to v4.4.0(pr [#1034])

## [2.4.8] - 2024-08-31

### Fixed

- deps: update github/codeql-action action to v3.26.5(pr [#1032])
- deps: update github/codeql-action action to v3.26.6(pr [#1033])

## [2.4.7] - 2024-08-17

### Security

- Dependencies: update github/codeql-action action to v3.26.1(pr [#1030])
- Dependencies: update github/codeql-action action to v3.26.2(pr [#1031])

## [2.4.6] - 2024-08-10

### Security

- Dependencies: update github/codeql-action action to v3.26.0(pr [#1029])
- Dependencies: update actions/upload-artifact action to v4.3.6(pr [#1028])

## [2.4.5] - 2024-08-03

### Changed

- chore-update tag-message and tag-name format(pr [#1025])
- ci-add bot-check context to toolkit/make_release workflow(pr [#1026])

### Security

- Dependencies: update github/codeql-action action to v3.25.15(pr [#1023])
- Dependencies: update ossf/scorecard-action action to v2.4.0(pr [#1024])
- Dependencies: update actions/upload-artifact action to v4.3.5(pr [#1027])

## [2.4.4] - 2024-07-27

### Changed

- ci-standardise and adopt toolkit v0.24.0(pr [#1022])

### Security

- Dependencies: update github/codeql-action action to v3.25.13(pr [#1017])
- Dependencies: update fossa-contrib/fossa-action digest to baed402(pr [#1018])
- Dependencies: update fossa-contrib/fossa-action digest to 3627ae2(pr [#1019])
- Dependencies: update rust crate lambda_runtime to 0.13.0(pr [#1020])
- Dependencies: update github/codeql-action action to v3.25.14(pr [#1021])

## [2.4.3] - 2024-07-20

### Fixed

- replace hardcoded FOSSA API key with GitHub secret reference(pr [#1016])

### Security

- Dependencies: update fossa-contrib/fossa-action digest to 442f249(pr [#1014])
- Dependencies: update fossa-contrib/fossa-action digest to e323a00(pr [#1015])

## [2.4.2] - 2024-07-13

### Security

- Dependencies: update fossa-contrib/fossa-action digest to 80596a6(pr [#1009])
- Dependencies: update fossa-contrib/fossa-action digest to 0dd2a5e(pr [#1010])
- Dependencies: update fossa-contrib/fossa-action digest to a79a984(pr [#1011])
- Dependencies: update fossa-contrib/fossa-action digest to 0931c29(pr [#1012])
- Dependencies: update github/codeql-action action to v3.25.12(pr [#1013])

## [2.4.1] - 2024-07-06

### Security

- Dependencies: update fossa-contrib/fossa-action digest to 8429059(pr [#1006])
- Dependencies: update fossa-contrib/fossa-action digest to 43e532b(pr [#1007])
- Dependencies: update actions/upload-artifact action to v4.3.4(pr [#1008])

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
[#1007]: https://github.com/jerus-org/hcaptcha-rs/pull/1007
[#1008]: https://github.com/jerus-org/hcaptcha-rs/pull/1008
[#1009]: https://github.com/jerus-org/hcaptcha-rs/pull/1009
[#1010]: https://github.com/jerus-org/hcaptcha-rs/pull/1010
[#1011]: https://github.com/jerus-org/hcaptcha-rs/pull/1011
[#1012]: https://github.com/jerus-org/hcaptcha-rs/pull/1012
[#1013]: https://github.com/jerus-org/hcaptcha-rs/pull/1013
[#1014]: https://github.com/jerus-org/hcaptcha-rs/pull/1014
[#1015]: https://github.com/jerus-org/hcaptcha-rs/pull/1015
[#1016]: https://github.com/jerus-org/hcaptcha-rs/pull/1016
[#1017]: https://github.com/jerus-org/hcaptcha-rs/pull/1017
[#1018]: https://github.com/jerus-org/hcaptcha-rs/pull/1018
[#1019]: https://github.com/jerus-org/hcaptcha-rs/pull/1019
[#1020]: https://github.com/jerus-org/hcaptcha-rs/pull/1020
[#1021]: https://github.com/jerus-org/hcaptcha-rs/pull/1021
[#1022]: https://github.com/jerus-org/hcaptcha-rs/pull/1022
[#1023]: https://github.com/jerus-org/hcaptcha-rs/pull/1023
[#1024]: https://github.com/jerus-org/hcaptcha-rs/pull/1024
[#1025]: https://github.com/jerus-org/hcaptcha-rs/pull/1025
[#1026]: https://github.com/jerus-org/hcaptcha-rs/pull/1026
[#1027]: https://github.com/jerus-org/hcaptcha-rs/pull/1027
[#1029]: https://github.com/jerus-org/hcaptcha-rs/pull/1029
[#1028]: https://github.com/jerus-org/hcaptcha-rs/pull/1028
[#1030]: https://github.com/jerus-org/hcaptcha-rs/pull/1030
[#1031]: https://github.com/jerus-org/hcaptcha-rs/pull/1031
[#1032]: https://github.com/jerus-org/hcaptcha-rs/pull/1032
[#1033]: https://github.com/jerus-org/hcaptcha-rs/pull/1033
[#1034]: https://github.com/jerus-org/hcaptcha-rs/pull/1034
[#1035]: https://github.com/jerus-org/hcaptcha-rs/pull/1035
[Unreleased]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.4.9...HEAD
[2.4.9]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.4.8...v2.4.9
[2.4.8]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.4.7...v2.4.8
[2.4.7]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.4.6...v2.4.7
[2.4.6]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.4.5...v2.4.6
[2.4.5]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.4.4...v2.4.5
[2.4.4]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.4.3...v2.4.4
[2.4.3]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.4.2...v2.4.3
[2.4.2]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.4.1...v2.4.2
[2.4.1]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.4.0...v2.4.1
[2.4.0]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.3.1...v2.4.0
[2.3.1]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.3.0...v2.3.1
[2.3.0]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.2.2...v2.3.0
[2.2.2]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.2.1...v2.2.2
[2.2.1]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.2.0...v2.2.1
[2.2.0]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.1.1...v2.2.0
[2.1.1]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.0.1...v2.1.1
[2.0.1]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/jerus-org/hcaptcha-rs/compare/v1.0.1...v2.0.0
[1.0.1]: https://github.com/jerus-org/hcaptcha-rs/releases/tag/v1.0.1
