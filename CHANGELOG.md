# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- deps: update rust crate syn to 2.0.87(pr [#1165])
- deps: update rust crate thiserror to 1.0.67(pr [#1166])
- deps: update rust crate url to 2.5.3(pr [#1167])

## [2.8.6] - 2024-11-02

### Fixed

- deps: update rust crate mockd to 0.4.27(pr [#1164])

## [2.8.5] - 2024-11-02

### Fixed

- deps: update rust crate reqwest to 0.12.9(pr [#1159])
- deps: update rust crate serde to 1.0.214(pr [#1160])
- deps: update dependency toolkit to v1.15.0(pr [#1161])
- deps: update rust crate thiserror to 1.0.66(pr [#1163])
- deps: update rust crate syn to 2.0.86(pr [#1162])

## [2.8.4] - 2024-10-26

### Fixed

- deps: update rust crate mockd to 0.4.26(pr [#1158])

## [2.8.3] - 2024-10-26

### Fixed

- deps: update rust crate syn to 2.0.85(pr [#1149])

## [2.8.2] - 2024-10-25

### Changed

- ci(circleci)-update make_release job to support package matrix(pr [#1156])
- ci-update CircleCI config to separate hcaptcha release job(pr [#1157])

## [2.8.1] - 2024-10-25

### Changed

- ci(circleci)-update config to handle package prefix and verbosity options(pr [#1155])

## [2.8.0] - 2024-10-25

### Added

- add support for specifying package in release and publish steps(pr [#1153])

### Fixed

- circleci: remove unnecessary quotes in cargo release command(pr [#1154])

## [2.7.9] - 2024-10-24

### Changed

- chore(ci)-update default verbosity level in CircleCI config(pr [#1152])

## [2.7.8] - 2024-10-24

### Changed

- ci(circleci)-increase default verbosity level for pcu command(pr [#1151])

## [2.7.7] - 2024-10-24

### Changed

- ci-update CircleCI config to include pcu_verbosity and publish parameters(pr [#1150])

## [2.7.6] - 2024-10-24

### Fixed

- deps: update rust crate syn to 2.0.83(pr [#1148])

## [2.7.5] - 2024-10-23

### Fixed

- deps: update actions/checkout action to v4.2.2(pr [#1145])

## [2.7.4] - 2024-10-23

### Changed

- ci(circleci)-add update_pcu parameter to pipeline configuration(pr [#1147])

## [2.7.3] - 2024-10-23

### Fixed

- circleci: correct command order for GitHub release creation(pr [#1146])

## [2.7.2] - 2024-10-23

### Fixed

- circleci: correct conditional statements in config file(pr [#1144])

## [2.7.1] - 2024-10-23

### Changed

- ci-update CircleCI config to include pcu_workspace parameter and remove pcu_prefix matrix(pr [#1143])

## [2.7.0] - 2024-10-23

### Added

- add pcu_workspace parameter to config for workspace flag support(pr [#1142])

### Fixed

- deps: update rust crate proc-macro2 to 1.0.89(pr [#1138])
- deps: update rust crate serde to 1.0.213(pr [#1139])
- deps: update rust crate thiserror to 1.0.65(pr [#1140])
- deps: update github/codeql-action action to v3.27.0(pr [#1141])

## [2.6.4] - 2024-10-22

### Changed

- ci(circleci)-add commands for GitHub and Cargo release processes(pr [#1137])

### Fixed

- deps: update rust crate serde to 1.0.211(pr [#1135])
- deps: update rust crate tokio to 1.41.0(pr [#1136])

## [2.6.3] - 2024-10-21

### Changed

- ci(circleci)-update config to include wasm_test and adjust cargo release requirements(pr [#1133])

### Fixed

- circleci: update workflow dependencies for release jobs(pr [#1134])

## [2.6.2] - 2024-10-21

### Changed

- ci(circleci)-add job names and dependencies for release workflow(pr [#1132])

## [2.6.1] - 2024-10-21

### Changed

- ci(circleci)-add make_release job with matrix parameters to config(pr [#1131])

## [2.6.0] - 2024-10-21

### Added

- add new options for token, key, secret, and ip in CLI(pr [#1060])
- add async hcaptcha verification with color-eyre and tokio(pr [#1061])
- add snapbox dependency and initial command tests(pr [#1063])
- add trace feature to all test suite Cargo.toml files(pr [#1068])
- update renovate.json to enable circleci-toolkit and add sourceUrl(pr [#1079])
- add simple_captcha test case to CLI test suite(pr [#1084])
- add mock-verifier to build matrix(pr [#1105])
- add hcaptcha integration and async test support(pr [#1109])
- add new wasm example with hcaptcha integration(pr [#1117])

### Changed

- ci-update CircleCI config to adjust build matrix and comment out wasi builds(pr [#1067])
- chore-add CODEOWNERS file to define code ownership(pr [#1069])
- ci-add SonarCloud integration and security audit command(pr [#1070])
- ci-add security job to CircleCI workflow(pr [#1071])
- ci(circleci)-remove wasi-env executor and update jobs to use rust-env(pr [#1072])
- chore(circleci)-update toolkit orb and comment out sonarcloud and various cargo commands(pr [#1074])
- chore(ci)-remove commented-out job configurations from CircleCI config(pr [#1075])
- chore-update dependencies to use workspace versions in Cargo.toml files(pr [#1088])
- Create SECURITY.md(pr [#1093])
- refactor-rename test_suite_cli directory to test-suite-cli(pr [#1094])
- Hyphen-test-package-names(pr [#1095])
- Create CODE_OF_CONDUCT.md(pr [#1097])
- refactor(test-wasm)-remove console_error_panic_hook and unused utilities(pr [#1116])
- docs-update README to add Web Assembly section(pr [#1120])
- chore-update Cargo.toml to use workspace settings for edition, rust-version, and publish(pr [#1130])

### Fixed

- deps: update github/codeql-action action to v3.26.8(pr [#1064])
- deps: update rust crate clap to 4.5.18(pr [#1065])
- deps: update rust crate mockd to 0.4.20(pr [#1066])
- deps: update rust crate thiserror to 1.0.64(pr [#1073])
- deps: update github/codeql-action action to v3.26.9(pr [#1076])
- deps: update rust crate async-trait to 0.1.83(pr [#1077])
- deps: update actions/checkout action to v4.2.0(pr [#1078])
- deps: update rust crate clap-verbosity-flag to 2.2.2(pr [#1080])
- deps: update dependency toolkit to v1.11.0(pr [#1081])
- deps: update rust crate syn to 2.0.79(pr [#1082])
- deps: update rust crate mockd to 0.4.21(pr [#1083])
- deps: update github/codeql-action action to v3.26.10(pr [#1085])
- deps: update rust crate reqwest to 0.12.8(pr [#1087])
- deps: update rust crate clap to 4.5.19(pr [#1089])
- deps: update github/codeql-action action to v3.26.11(pr [#1090])
- deps: update rust crate mockd to 0.4.22(pr [#1091])
- deps: update rust crate proc-macro2 to 1.0.87(pr [#1096])
- deps: update actions/upload-artifact action to v4.4.1(pr [#1098])
- deps: update actions/checkout action to v4.2.1(pr [#1100])
- deps: update github/codeql-action action to v3.26.12(pr [#1101])
- deps: update rust crate clap to 4.5.20(pr [#1102])
- deps: update actions/upload-artifact action to v4.4.3(pr [#1103])
- deps: update rust crate mockd to 0.4.23(pr [#1104])
- deps: update rust crate mockd to 0.4.24(pr [#1106])
- deps: update github/codeql-action action to v3.26.13(pr [#1107])
- deps: update rust crate trybuild to 1.0.100(pr [#1108])
- deps: update rust crate wasm-bindgen to 0.2.95(pr [#1110])
- deps: update rust crate wasm-bindgen-futures to 0.4.45(pr [#1111])
- deps: update rust crate wasm-bindgen-test to 0.3.45(pr [#1112])
- deps: update rust crate uuid to 1.11.0(pr [#1113])
- deps: update dependency node to v6.3.0(pr [#1114])
- deps: update rust crate proc-macro2 to 1.0.88(pr [#1115])
- deps: update rust crate wasm-bindgen-test to 0.3.45(pr [#1119])
- deps: update rust crate wasm-bindgen-futures to 0.4.45(pr [#1118])
- deps: update rust crate serde_json to 1.0.129(pr [#1121])
- deps: update rust crate serde_json to 1.0.130(pr [#1122])
- deps: update rust crate trybuild to 1.0.101(pr [#1123])
- deps: update rust crate mockd to 0.4.25(pr [#1124])
- deps: update rust crate serde_json to 1.0.131(pr [#1125])
- deps: update rust crate serde_json to 1.0.132(pr [#1126])
- deps: update rust crate syn to 2.0.81(pr [#1127])
- deps: update rust crate syn to 2.0.82(pr [#1128])
- release: correct tag message and name placeholders in release config(pr [#1129])

### Security

- Dependencies: update multiple crate dependencies in Cargo.lock(pr [#1092])

## [2.5.0] - 2024-09-14

### Added

- add hcaptcha-cli package to workspace(pr [#1054])
- add clap and verbosity flag for command-line parsing(pr [#1057])

### Changed

- chore-update CircleCI config and renovate settings(pr [#1035])
- Add .circleci/config.yml(pr [#1041])
- ci-add cargo_args parameter to required_builds in CircleCI config(pr [#1055])
- chore-update Cargo.toml files to edition 2021 and set rust-version to 1.75(pr [#1059])

### Fixed

- hcaptcha_derive: update dependencies and correct spacing in Cargo.toml(pr [#1036])
- deps: update rust crate async-trait to 0.1.82(pr [#1037])
- deps: update rust crate env_logger to 0.11.5(pr [#1038])
- deps: update rust crate log to 0.4.22(pr [#1039])
- deps: update rust crate mockd to 0.4.18(pr [#1040])
- deps: update rust crate proc-macro2 to 1.0.86(pr [#1042])
- deps: update rust crate quote to 1.0.37(pr [#1043])
- deps: update rust crate reqwest to 0.12.7(pr [#1044])
- deps: update rust crate syn to 2.0.77(pr [#1045])
- deps: update rust crate wiremock to 0.6.2(pr [#1046])
- deps: update serde packages(pr [#1047])
- deps: update rust crate tokio to 1.40.0(pr [#1048])
- deps: update rust crate uuid to 1.10.0(pr [#1049])
- deps: update rust crate thiserror to 1.0.63(pr [#1050])
- deps: update rust crate tracing-test to 0.2.5(pr [#1051])
- deps: update rust crate trybuild to 1.0.99(pr [#1052])
- deps: update rust crate url to 2.5.2(pr [#1053])
- deps: update github/codeql-action action to v3.26.7(pr [#1056])
- deps: update rust crate mockd to 0.4.19(pr [#1058])

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
[#1036]: https://github.com/jerus-org/hcaptcha-rs/pull/1036
[#1037]: https://github.com/jerus-org/hcaptcha-rs/pull/1037
[#1041]: https://github.com/jerus-org/hcaptcha-rs/pull/1041
[#1038]: https://github.com/jerus-org/hcaptcha-rs/pull/1038
[#1039]: https://github.com/jerus-org/hcaptcha-rs/pull/1039
[#1040]: https://github.com/jerus-org/hcaptcha-rs/pull/1040
[#1042]: https://github.com/jerus-org/hcaptcha-rs/pull/1042
[#1043]: https://github.com/jerus-org/hcaptcha-rs/pull/1043
[#1044]: https://github.com/jerus-org/hcaptcha-rs/pull/1044
[#1045]: https://github.com/jerus-org/hcaptcha-rs/pull/1045
[#1046]: https://github.com/jerus-org/hcaptcha-rs/pull/1046
[#1047]: https://github.com/jerus-org/hcaptcha-rs/pull/1047
[#1048]: https://github.com/jerus-org/hcaptcha-rs/pull/1048
[#1049]: https://github.com/jerus-org/hcaptcha-rs/pull/1049
[#1050]: https://github.com/jerus-org/hcaptcha-rs/pull/1050
[#1051]: https://github.com/jerus-org/hcaptcha-rs/pull/1051
[#1052]: https://github.com/jerus-org/hcaptcha-rs/pull/1052
[#1053]: https://github.com/jerus-org/hcaptcha-rs/pull/1053
[#1054]: https://github.com/jerus-org/hcaptcha-rs/pull/1054
[#1055]: https://github.com/jerus-org/hcaptcha-rs/pull/1055
[#1056]: https://github.com/jerus-org/hcaptcha-rs/pull/1056
[#1057]: https://github.com/jerus-org/hcaptcha-rs/pull/1057
[#1058]: https://github.com/jerus-org/hcaptcha-rs/pull/1058
[#1059]: https://github.com/jerus-org/hcaptcha-rs/pull/1059
[#1060]: https://github.com/jerus-org/hcaptcha-rs/pull/1060
[#1061]: https://github.com/jerus-org/hcaptcha-rs/pull/1061
[#1063]: https://github.com/jerus-org/hcaptcha-rs/pull/1063
[#1064]: https://github.com/jerus-org/hcaptcha-rs/pull/1064
[#1065]: https://github.com/jerus-org/hcaptcha-rs/pull/1065
[#1066]: https://github.com/jerus-org/hcaptcha-rs/pull/1066
[#1067]: https://github.com/jerus-org/hcaptcha-rs/pull/1067
[#1068]: https://github.com/jerus-org/hcaptcha-rs/pull/1068
[#1069]: https://github.com/jerus-org/hcaptcha-rs/pull/1069
[#1070]: https://github.com/jerus-org/hcaptcha-rs/pull/1070
[#1071]: https://github.com/jerus-org/hcaptcha-rs/pull/1071
[#1072]: https://github.com/jerus-org/hcaptcha-rs/pull/1072
[#1073]: https://github.com/jerus-org/hcaptcha-rs/pull/1073
[#1074]: https://github.com/jerus-org/hcaptcha-rs/pull/1074
[#1075]: https://github.com/jerus-org/hcaptcha-rs/pull/1075
[#1076]: https://github.com/jerus-org/hcaptcha-rs/pull/1076
[#1077]: https://github.com/jerus-org/hcaptcha-rs/pull/1077
[#1078]: https://github.com/jerus-org/hcaptcha-rs/pull/1078
[#1080]: https://github.com/jerus-org/hcaptcha-rs/pull/1080
[#1079]: https://github.com/jerus-org/hcaptcha-rs/pull/1079
[#1081]: https://github.com/jerus-org/hcaptcha-rs/pull/1081
[#1082]: https://github.com/jerus-org/hcaptcha-rs/pull/1082
[#1083]: https://github.com/jerus-org/hcaptcha-rs/pull/1083
[#1084]: https://github.com/jerus-org/hcaptcha-rs/pull/1084
[#1085]: https://github.com/jerus-org/hcaptcha-rs/pull/1085
[#1088]: https://github.com/jerus-org/hcaptcha-rs/pull/1088
[#1087]: https://github.com/jerus-org/hcaptcha-rs/pull/1087
[#1089]: https://github.com/jerus-org/hcaptcha-rs/pull/1089
[#1090]: https://github.com/jerus-org/hcaptcha-rs/pull/1090
[#1091]: https://github.com/jerus-org/hcaptcha-rs/pull/1091
[#1092]: https://github.com/jerus-org/hcaptcha-rs/pull/1092
[#1093]: https://github.com/jerus-org/hcaptcha-rs/pull/1093
[#1094]: https://github.com/jerus-org/hcaptcha-rs/pull/1094
[#1095]: https://github.com/jerus-org/hcaptcha-rs/pull/1095
[#1096]: https://github.com/jerus-org/hcaptcha-rs/pull/1096
[#1097]: https://github.com/jerus-org/hcaptcha-rs/pull/1097
[#1098]: https://github.com/jerus-org/hcaptcha-rs/pull/1098
[#1100]: https://github.com/jerus-org/hcaptcha-rs/pull/1100
[#1101]: https://github.com/jerus-org/hcaptcha-rs/pull/1101
[#1102]: https://github.com/jerus-org/hcaptcha-rs/pull/1102
[#1103]: https://github.com/jerus-org/hcaptcha-rs/pull/1103
[#1104]: https://github.com/jerus-org/hcaptcha-rs/pull/1104
[#1106]: https://github.com/jerus-org/hcaptcha-rs/pull/1106
[#1109]: https://github.com/jerus-org/hcaptcha-rs/pull/1109
[#1107]: https://github.com/jerus-org/hcaptcha-rs/pull/1107
[#1108]: https://github.com/jerus-org/hcaptcha-rs/pull/1108
[#1110]: https://github.com/jerus-org/hcaptcha-rs/pull/1110
[#1111]: https://github.com/jerus-org/hcaptcha-rs/pull/1111
[#1112]: https://github.com/jerus-org/hcaptcha-rs/pull/1112
[#1113]: https://github.com/jerus-org/hcaptcha-rs/pull/1113
[#1114]: https://github.com/jerus-org/hcaptcha-rs/pull/1114
[#1116]: https://github.com/jerus-org/hcaptcha-rs/pull/1116
[#1115]: https://github.com/jerus-org/hcaptcha-rs/pull/1115
[#1117]: https://github.com/jerus-org/hcaptcha-rs/pull/1117
[#1119]: https://github.com/jerus-org/hcaptcha-rs/pull/1119
[#1118]: https://github.com/jerus-org/hcaptcha-rs/pull/1118
[#1120]: https://github.com/jerus-org/hcaptcha-rs/pull/1120
[#1121]: https://github.com/jerus-org/hcaptcha-rs/pull/1121
[#1122]: https://github.com/jerus-org/hcaptcha-rs/pull/1122
[#1123]: https://github.com/jerus-org/hcaptcha-rs/pull/1123
[#1124]: https://github.com/jerus-org/hcaptcha-rs/pull/1124
[#1125]: https://github.com/jerus-org/hcaptcha-rs/pull/1125
[#1126]: https://github.com/jerus-org/hcaptcha-rs/pull/1126
[#1127]: https://github.com/jerus-org/hcaptcha-rs/pull/1127
[#1128]: https://github.com/jerus-org/hcaptcha-rs/pull/1128
[#1129]: https://github.com/jerus-org/hcaptcha-rs/pull/1129
[#1130]: https://github.com/jerus-org/hcaptcha-rs/pull/1130
[#1131]: https://github.com/jerus-org/hcaptcha-rs/pull/1131
[#1132]: https://github.com/jerus-org/hcaptcha-rs/pull/1132
[#1133]: https://github.com/jerus-org/hcaptcha-rs/pull/1133
[#1134]: https://github.com/jerus-org/hcaptcha-rs/pull/1134
[#1135]: https://github.com/jerus-org/hcaptcha-rs/pull/1135
[#1136]: https://github.com/jerus-org/hcaptcha-rs/pull/1136
[#1137]: https://github.com/jerus-org/hcaptcha-rs/pull/1137
[#1138]: https://github.com/jerus-org/hcaptcha-rs/pull/1138
[#1139]: https://github.com/jerus-org/hcaptcha-rs/pull/1139
[#1140]: https://github.com/jerus-org/hcaptcha-rs/pull/1140
[#1141]: https://github.com/jerus-org/hcaptcha-rs/pull/1141
[#1142]: https://github.com/jerus-org/hcaptcha-rs/pull/1142
[#1143]: https://github.com/jerus-org/hcaptcha-rs/pull/1143
[#1144]: https://github.com/jerus-org/hcaptcha-rs/pull/1144
[#1146]: https://github.com/jerus-org/hcaptcha-rs/pull/1146
[#1147]: https://github.com/jerus-org/hcaptcha-rs/pull/1147
[#1145]: https://github.com/jerus-org/hcaptcha-rs/pull/1145
[#1148]: https://github.com/jerus-org/hcaptcha-rs/pull/1148
[#1150]: https://github.com/jerus-org/hcaptcha-rs/pull/1150
[#1151]: https://github.com/jerus-org/hcaptcha-rs/pull/1151
[#1152]: https://github.com/jerus-org/hcaptcha-rs/pull/1152
[#1153]: https://github.com/jerus-org/hcaptcha-rs/pull/1153
[#1154]: https://github.com/jerus-org/hcaptcha-rs/pull/1154
[#1155]: https://github.com/jerus-org/hcaptcha-rs/pull/1155
[#1156]: https://github.com/jerus-org/hcaptcha-rs/pull/1156
[#1157]: https://github.com/jerus-org/hcaptcha-rs/pull/1157
[#1149]: https://github.com/jerus-org/hcaptcha-rs/pull/1149
[#1158]: https://github.com/jerus-org/hcaptcha-rs/pull/1158
[#1159]: https://github.com/jerus-org/hcaptcha-rs/pull/1159
[#1160]: https://github.com/jerus-org/hcaptcha-rs/pull/1160
[#1161]: https://github.com/jerus-org/hcaptcha-rs/pull/1161
[#1163]: https://github.com/jerus-org/hcaptcha-rs/pull/1163
[#1162]: https://github.com/jerus-org/hcaptcha-rs/pull/1162
[#1164]: https://github.com/jerus-org/hcaptcha-rs/pull/1164
[#1165]: https://github.com/jerus-org/hcaptcha-rs/pull/1165
[#1166]: https://github.com/jerus-org/hcaptcha-rs/pull/1166
[#1167]: https://github.com/jerus-org/hcaptcha-rs/pull/1167
[Unreleased]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.8.6...HEAD
[2.8.6]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.8.5...v2.8.6
[2.8.5]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.8.4...v2.8.5
[2.8.4]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.8.3...v2.8.4
[2.8.3]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.8.2...v2.8.3
[2.8.2]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.8.1...v2.8.2
[2.8.1]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.8.0...v2.8.1
[2.8.0]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.7.9...v2.8.0
[2.7.9]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.7.8...v2.7.9
[2.7.8]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.7.7...v2.7.8
[2.7.7]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.7.6...v2.7.7
[2.7.6]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.7.5...v2.7.6
[2.7.5]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.7.4...v2.7.5
[2.7.4]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.7.3...v2.7.4
[2.7.3]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.7.2...v2.7.3
[2.7.2]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.7.1...v2.7.2
[2.7.1]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.7.0...v2.7.1
[2.7.0]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.6.4...v2.7.0
[2.6.4]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.6.3...v2.6.4
[2.6.3]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.6.2...v2.6.3
[2.6.2]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.6.1...v2.6.2
[2.6.1]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.6.0...v2.6.1
[2.6.0]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.5.0...v2.6.0
[2.5.0]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.4.9...v2.5.0
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
