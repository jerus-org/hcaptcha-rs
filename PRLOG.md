# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- 📦 build(ci)-add circleci audit config(pr [#1441])

### Fixed

- deps: update rust crate lambda_runtime to v1(pr [#1442])
- deps: update actions/upload-artifact action to v5(pr [#1444])
- deps: update github/codeql-action action to v4(pr [#1443])
- deps: update actions/checkout action to v6(pr [#1445])
- deps: update rust crate uuid to 1.19.0(pr [#1446])

## [3.1.0] - 2025-11-28

### Fixed

- deps: update rust crate clap to 4.5.53(pr [#1431])
- deps: update rust crate mockd to 0.4.55(pr [#1432])
- deps: update rust crate quote to 1.0.42(pr [#1433])
- deps: update rust crate syn to 2.0.111(pr [#1434])
- deps: update rust crate trybuild to 1.0.114(pr [#1435])
- deps: update rust crate wasm-bindgen to 0.2.105(pr [#1436])
- deps: update rust crate wasm-bindgen-futures to 0.4.55(pr [#1437])
- deps: update rust crate wasm-bindgen-test to 0.3.55(pr [#1438])
- deps: update rust crate wiremock to 0.6.5(pr [#1439])
- deps: update serde packages(pr [#1440])

## [3.0.33] - 2025-10-28

### Fixed

- deps: update dependency toolkit to v2.13.5(pr [#1421])
- deps: update ossf/scorecard-action action to v2.4.3(pr [#1422])
- deps: update rust crate clap to 4.5.50(pr [#1423])
- deps: update rust crate mockd to 0.4.54(pr [#1424])
- deps: update rust crate proc-macro2 to 1.0.103(pr [#1425])
- deps: update rust crate quote to 1.0.41(pr [#1426])
- deps: update rust crate reqwest to 0.12.24(pr [#1427])
- deps: update rust crate syn to 2.0.108(pr [#1428])
- deps: update rust crate thiserror to 2.0.17(pr [#1429])
- deps: update rust crate trybuild to 1.0.112(pr [#1430])

## [3.0.32] - 2025-09-29

### Changed

- 🔧 chore(release)-add pre-release hook to release.toml(pr [#1415])
- 🔧 chore(release)-update pre-release replacements in release.toml(pr [#1416])
- 👷 ci(circleci)-update circleci toolkit version(pr [#1417])
- ♻️ refactor(release-hook)-remove redundant README build steps(pr [#1418])
- 🔧 chore(release-hook)-update release script for hcaptcha(pr [#1419])
- 👷 ci(circleci)-update circleci-toolkit orb version(pr [#1420])

## [3.0.31] - 2025-09-28

### Changed

- chore-rename CHANGELOG.md to PRLOG.md(pr [#1402])
- 🔧 chore(ci)-update CircleCI toolkit version(pr [#1413])
- 📦 build(hcaptcha)-add release hook script for README and changelog(pr [#1414])
- 🔧 chore(release)-add pre-release hook to release.toml(pr [#1415])

### Fixed

- deps: update rust crate chrono to 0.4.42(pr [#1403])
- deps: update rust crate clap to 4.5.48(pr [#1404])
- deps: update rust crate lambda_runtime to 0.14.4(pr [#1405])
- deps: update rust crate log to 0.4.28(pr [#1406])
- deps: update rust crate mockd to 0.4.53(pr [#1407])
- deps: update rust crate trybuild to 1.0.111(pr [#1408])
- deps: update rust crate url to 2.5.7(pr [#1409])
- deps: update rust crate uuid to 1.18.1(pr [#1410])
- deps: update rust crate wasm-bindgen to 0.2.103(pr [#1411])
- deps: update rust crate wasm-bindgen-futures to 0.4.53(pr [#1412])

### Security

- Dependencies: bump tracing-subscriber from 0.3.19 to 0.3.20 in the cargo group across 1 directory(pr [#1400])

## [3.0.30] - 2025-08-28

### Fixed

- deps: update github/codeql-action action to v3.29.11(pr [#1390])
- deps: update rust crate async-trait to 0.1.89(pr [#1391])
- deps: update rust crate clap to 4.5.45(pr [#1392])
- deps: update rust crate clap-verbosity-flag to 3.0.4(pr [#1393])
- deps: update rust crate mockd to 0.4.51(pr [#1394])
- deps: update rust crate proc-macro2 to 1.0.101(pr [#1395])
- deps: update rust crate reqwest to 0.12.23(pr [#1396])
- deps: update rust crate serde_json to 1.0.143(pr [#1397])
- deps: update rust crate syn to 2.0.106(pr [#1398])
- deps: update rust crate thiserror to 2.0.16(pr [#1399])

## [3.0.29] - 2025-07-28

### Added

- add client reuse functionality with `verify_request` method for efficient multiple requests (pr [#1373])

### Changed

- refactor-client methods to use shared internal `make_request` method (pr [#1373])
- 🔧 chore(vscode)-add SonarLint project configuration(pr [#1374])
- Use-new-formatting-syntax-for-variables(pr [#1375])
- Update-configuration-and-toolkit-version(pr [#1376])
- 👷 ci(config)-add committer-based pipeline selection(pr [#1377])
- 👷 ci(circleci)-add update_pcu parameter to config(pr [#1378])
- Simplify-renovate-configuration(pr [#1379])

### Fixed

- deps: update rust crate clap to 4.5.41(pr [#1381])
- deps: update github/codeql-action action to v3.29.4(pr [#1380])
- deps: update rust crate lambda_runtime to 0.14.3(pr [#1382])
- deps: update rust crate mockd to 0.4.50(pr [#1383])
- deps: update rust crate rand to 0.9.2(pr [#1384])
- deps: update rust crate reqwest to 0.12.22(pr [#1385])
- deps: update rust crate serde_json to 1.0.141(pr [#1386])
- deps: update rust crate trybuild to 1.0.106(pr [#1387])
- deps: update dependency toolkit to v2.12.1(pr [#1388])
- deps: update rust crate tokio to 1.46.1(pr [#1389])

## [3.0.28] - 2025-06-28

### Fixed

- deps: update rust crate color-eyre to 0.6.5(pr [#1366])
- deps: update ossf/scorecard-action action to v2.4.2(pr [#1364])
- deps: update rust crate clap to 4.5.40(pr [#1365])
- deps: update rust crate mockd to 0.4.49(pr [#1367])
- deps: update rust crate reqwest to 0.12.20(pr [#1368])
- deps: update rust crate syn to 2.0.104(pr [#1369])
- deps: update rust crate wiremock to 0.6.4(pr [#1370])
- deps: update github/codeql-action action to v3.29.0(pr [#1371])
- deps: update rust crate lambda_runtime to 0.14.2(pr [#1372])

## [3.0.27] - 2025-05-28

### Changed

- 🔧 chore(config)-update Renovate schedule(pr [#1354])

### Fixed

- deps: update github/codeql-action action to v3.28.18(pr [#1355])
- deps: update rust crate clap to 4.5.38(pr [#1356])
- deps: update rust crate clap-verbosity-flag to 3.0.3(pr [#1357])
- deps: update rust crate color-eyre to 0.6.4(pr [#1358])
- deps: update rust crate mockd to 0.4.48(pr [#1359])
- deps: update rust crate trybuild to 1.0.105(pr [#1360])
- deps: update rust crate url to 2.5.4(pr [#1361])
- deps: update rust crate tokio to 1.45.1(pr [#1362])
- deps: update rust crate uuid to 1.17.0(pr [#1363])

## [3.0.26] - 2025-05-03

### Changed

- 👷 ci(circleci)-remove unused configuration parameters(pr [#1353])

### Fixed

- deps: update rust crate chrono to 0.4.41(pr [#1350])
- deps: update rust crate syn to 2.0.101(pr [#1351])
- deps: update rust crate mockd to 0.4.47(pr [#1352])

## [3.0.25] - 2025-05-01

### Changed

- 👷 ci(circleci)-update GitHub release workflow(pr [#1349])

## [3.0.24] - 2025-05-01

### Changed

- 👷 ci(circleci)-update circleci toolkit orb version(pr [#1348])

## [3.0.23] - 2025-04-30

### Changed

- 👷 ci(circleci)-update toolkit orb version and comment out flags(pr [#1345])
- 🔧 chore(ci)-clean up CircleCI GitHub release config(pr [#1346])
- 👷 ci(circleci)-update circleci-toolkit orb version(pr [#1347])

## [3.0.22] - 2025-04-30

### Changed

- 👷 ci(circleci)-remove release filters from config(pr [#1344])

## [3.0.21] - 2025-04-26

### Changed

- 👷 ci(circleci)-remove filters from save_next_version job(pr [#1342])
- 👷 ci(circleci)-update github release workflow configuration(pr [#1343])

## [3.0.20] - 2025-04-25

### Changed

- 👷 ci(github)-add CircleCI config for GitHub release automation(pr [#1341])

## [3.0.19] - 2025-04-25

### Changed

- 👷 ci(circleci)-add release filters and update workflows(pr [#1340])

## [3.0.18] - 2025-04-25

### Fixed

- deps: update github/codeql-action action to v3.28.16(pr [#1337])
- deps: update rust crate clap to 4.5.37(pr [#1338])
- deps: update rust crate mockd to 0.4.46(pr [#1339])

## [3.0.17] - 2025-04-24

### Changed

- 👷 ci(circleci)-add condition for updating pcu(pr [#1335])
- 🔧 chore(dependencies)-update rust version in Cargo.toml(pr [#1336])

## [3.0.16] - 2025-04-23

### Changed

- 👷 ci(config)-simplify package configuration in CircleCI(pr [#1334])

## [3.0.15] - 2025-04-23

### Changed

- 👷 ci(circleci)-update pipeline configuration for release handling(pr [#1331])
- 👷 ci(circleci)-add pcu_verbosity parameter to config(pr [#1332])
- 📝 docs(changelog)-update changelog for version 3.0.14(pr [#1333])

### Fixed

- deps: update rust crate clap to 4.5.36(pr [#1326])
- deps: update rust crate proc-macro2 to 1.0.95(pr [#1327])
- deps: update rust crate rand to 0.9.1(pr [#1328])
- deps: update rust crate tokio to 1.44.2(pr [#1329])
- deps: update dependency toolkit to v2.8.1(pr [#1330])

## [3.0.14] - 2025-04-13

### Fixed

- deps: update dependency toolkit to v2.5.1(pr [#1314])
- deps: update actions/upload-artifact action to v4.6.2(pr [#1310])
- deps: update github/codeql-action action to v3.28.12(pr [#1311])
- deps: update rust crate async-trait to 0.1.88(pr [#1312])
- deps: update rust crate reqwest to 0.12.15(pr [#1313])
- deps: update rust crate mockd to 0.4.44(pr [#1315])
- deps: update github/codeql-action action to v3.28.13(pr [#1316])
- deps: update rust crate clap to 4.5.34(pr [#1317])
- deps: update rust crate log to 0.4.27(pr [#1318])
- deps: update rust crate mockd to 0.4.45(pr [#1319])
- deps: update rust crate clap to 4.5.35(pr [#1320])
- deps: update rust crate env_logger to 0.11.8(pr [#1321])
- deps: update rust crate tokio to v1.44.2 [security](pr [#1323])
- deps: update github/codeql-action action to v3.28.15(pr [#1324])

### Security

- Dependencies: bump openssl from 0.10.71 to 0.10.72 in the cargo group across 1 directory(pr [#1322])

## [3.0.13] - 2025-03-14

### Fixed

- deps: update github/codeql-action action to v3.28.11(pr [#1300])
- deps: update rust crate clap to 4.5.32(pr [#1301])
- deps: update rust crate env_logger to 0.11.7(pr [#1302])
- deps: update rust crate quote to 1.0.40(pr [#1303])
- deps: update rust crate reqwest to 0.12.14(pr [#1304])
- deps: update rust crate serde to 1.0.219(pr [#1305])
- deps: update rust crate syn to 2.0.100(pr [#1306])
- deps: update rust crate tokio to 1.44.1(pr [#1307])
- deps: update rust crate mockd to 0.4.43(pr [#1308])
- deps: update rust crate uuid to 1.16.0(pr [#1309])

## [3.0.12] - 2025-03-08

### Fixed

- deps: update rust crate async-trait to 0.1.87(pr [#1290])
- deps: update rust crate mockd to 0.4.42(pr [#1291])
- deps: update rust crate proc-macro2 to 1.0.94(pr [#1292])
- deps: update rust crate quote to 1.0.39(pr [#1293])
- deps: update rust crate serde_json to 1.0.140(pr [#1294])
- deps: update rust crate syn to 2.0.99(pr [#1295])
- deps: update rust crate thiserror to 2.0.12(pr [#1296])
- deps: update rust crate trybuild to 1.0.104(pr [#1297])
- deps: update rust crate wiremock to 0.6.3(pr [#1298])
- deps: update dependency toolkit to v2.1.0(pr [#1299])

## [3.0.11] - 2025-03-01

### Fixed

- deps: update actions/upload-artifact action to v4.6.1(pr [#1282])
- deps: update dependency toolkit to v2.0.13(pr [#1283])
- deps: update github/codeql-action action to v3.28.10(pr [#1284])
- deps: update ossf/scorecard-action action to v2.4.1(pr [#1285])
- deps: update rust crate chrono to 0.4.40(pr [#1286])
- deps: update rust crate clap to 4.5.31(pr [#1287])
- deps: update rust crate macrotest to 1.1.0(pr [#1288])
- deps: update rust crate uuid to 1.15.1(pr [#1289])

## [3.0.10] - 2025-02-22

### Fixed

- deps: update rust crate clap to 4.5.30(pr [#1277])
- deps: update serde packages(pr [#1278])
- deps: update rust crate log to 0.4.26(pr [#1279])
- deps: update rust crate mockd to 0.4.40(pr [#1280])
- deps: update rust crate uuid to 1.14.0(pr [#1281])

## [3.0.9] - 2025-02-15

### Fixed

- deps: update github/codeql-action action to v3.28.9(pr [#1274])
- deps: update rust crate clap to 4.5.29(pr [#1275])
- deps: update rust crate mockd to 0.4.39(pr [#1276])

## [3.0.8] - 2025-02-08

### Changed

- 👷 ci(circleci): add pcu verbosity to config(pr [#1267])

### Fixed

- deps: update rust crate async-trait to 0.1.86(pr [#1269])
- deps: update rust crate clap to 4.5.28(pr [#1270])
- deps: update rust crate uuid to 1.13.1(pr [#1273])
- deps: update rust crate mockd to 0.4.38(pr [#1271])
- deps: update rust crate syn to 2.0.98(pr [#1272])

### Security

- Dependencies: bump openssl from 0.10.68 to 0.10.70 in the cargo group across 1 directory(pr [#1268])

## [3.0.7] - 2025-02-01

### Changed

- Update SECURITY.md(pr [#1266])

### Fixed

- deps: update github/codeql-action action to v3.28.8(pr [#1262])
- deps: update rust crate serde_json to 1.0.138(pr [#1263])
- deps: update rust crate mockd to 0.4.37(pr [#1264])
- deps: update rust crate rand to 0.9.0(pr [#1265])

## [3.0.6] - 2025-01-25

### Fixed

- deps: update dependency toolkit to v2.0.4(pr [#1249])
- deps: update rust crate clap to 4.5.27(pr [#1250])
- deps: update rust crate trybuild to 1.0.103(pr [#1251])
- deps: update rust crate uuid to 1.12.1(pr [#1252])
- deps: update rust crate wasm-bindgen to 0.2.100(pr [#1253])
- deps: update rust crate wasm-bindgen-futures to 0.4.50(pr [#1254])
- deps: update rust crate wasm-bindgen-test to 0.3.50(pr [#1255])
- deps: update serde packages(pr [#1256])
- deps: update actions/upload-artifact action to v4.6.0(pr [#1257])
- deps: update github/codeql-action action to v3.28.4(pr [#1258])
- deps: update rust crate itertools to 0.14.0(pr [#1259])
- deps: update rust crate tokio to 1.43.0(pr [#1260])
- deps: update rust crate mockd to 0.4.36(pr [#1261])

## [3.0.5] - 2025-01-18

### Fixed

- deps: update rust crate log to 0.4.25(pr [#1245])
- deps: update rust crate mockd to 0.4.35(pr [#1246])
- deps: update rust crate proc-macro2 to 1.0.93(pr [#1247])
- deps: update rust crate thiserror to 2.0.11(pr [#1248])

## [3.0.4] - 2025-01-11

### Fixed

- deps: update rust crate async-trait to 0.1.85(pr [#1241])
- deps: update rust crate clap to 4.5.26(pr [#1242])
- deps: update rust crate syn to 2.0.96(pr [#1243])
- deps: update rust crate thiserror to 2.0.10(pr [#1244])

## [3.0.3] - 2025-01-04

### Changed

- chore(circleci)-update toolkit orb to version 2.0.0 and add new jobs to workflows(pr [#1240])

### Fixed

- deps: update rust crate reqwest to 0.12.12(pr [#1236])
- deps: update rust crate syn to 2.0.94(pr [#1237])
- deps: update rust crate thiserror to 2.0.9(pr [#1238])
- deps: update rust crate tokio to 1.42.0(pr [#1239])

## [3.0.2] - 2024-12-30

### Changed

- chore(circleci)-update toolkit orb version and add make_release job to workflows(pr [#1229])
- chore(circleci)-update parameter name from cargo_package to package in config file(pr [#1234])
- ci(circleci)-update job naming in release workflow config(pr [#1235])

### Fixed

- deps: update rust crate quote to 1.0.38(pr [#1231])
- deps: update rust crate env_logger to 0.11.6(pr [#1230])
- deps: update rust crate mockd to 0.4.33(pr [#1232])
- deps: update rust crate reqwest to 0.12.10(pr [#1233])

## [3.0.1] - 2024-12-21

### Changed

- chore(circleci)-simplify release workflow by removing redundant job configuration(pr [#1224])

### Fixed

- deps: update github/codeql-action action to v3.27.9(pr [#1221])
- deps: update rust crate chrono to 0.4.39(pr [#1222])
- deps: update rust crate mockd to 0.4.31(pr [#1223])
- deps: update rust crate clap-verbosity-flag to 3.0.2(pr [#1225])
- deps: update rust crate serde to 1.0.216(pr [#1226])
- deps: update rust crate mockd to 0.4.32(pr [#1227])
- deps: update rust crate thiserror to 2.0.8(pr [#1228])

## [3.0.0] - 2024-12-09

### Changed

- Prepare-for-v3.0.0(pr [#1219])

## [3.0.0] - 2024-12-09

### Changed

- chore-update CircleCI config and Cargo.lock dependencies(pr [#1214])
- ci(circleci)-update config to use matrix parameters for package selection(pr [#1215])
- ci(circleci)-update job names for release workflow(pr [#1216])
- ci(circleci)-separate release jobs for hcaptcha and hcaptcha_derive(pr [#1217])
- chore(ci)-consolidate release steps for hcaptcha packages(pr [#1218])

## [2.8.10] - 2024-12-07

### Fixed

- deps: update rust crate clap-verbosity-flag to 3.0.1(pr [#1205])
- deps: update tracing packages(pr [#1206])
- deps: update github/codeql-action action to v3.27.6(pr [#1208])
- deps: update rust crate clap to 4.5.23(pr [#1209])
- deps: update rust crate syn to 2.0.90(pr [#1210])
- deps: update rust crate thiserror to 2.0.4(pr [#1211])

## [2.8.10] - 2024-12-07

### Fixed

- deps: update rust crate clap-verbosity-flag to 3.0.1(pr [#1205])
- deps: update tracing packages(pr [#1206])
- deps: update github/codeql-action action to v3.27.6(pr [#1208])
- deps: update rust crate clap to 4.5.23(pr [#1209])
- deps: update rust crate syn to 2.0.90(pr [#1210])
- deps: update rust crate thiserror to 2.0.4(pr [#1211])

## [2.8.9] - 2024-11-23

### Added

- change default feature from nativetls-backend to rustls-backend(pr [#1203])

### Changed

- chore-update renovate configuration to include prConcurrentLimit setting(pr [#1189])
- chore-update renovate schedule to run before 11:00am on Friday(pr [#1190])
- refactor-rename HcaptchaError to Error across the codebase(pr [#1197])
- 1191 version 30(pr [#1198])
- refactor-rename and deprecate verify_client_response method in favour of verify(pr [#1200])
- Ver3/1199 update documentation(pr [#1201])
- test-additional tests to improve code coverage(pr [#1204])
- BREAKING: docs-update README with breaking changes for version 3.0.0(pr [#1207])
- Merge github.com:jerus-org/hcaptcha-rs into 1191-version-30(pr [#1213])

## [2.8.9] - 2024-11-23

### Fixed

- deps: update github/codeql-action action to v3.27.5(pr [#1185])
- deps: update rust crate clap-verbosity-flag to 2.2.3(pr [#1186])
- deps: update rust crate mockd to 0.4.29(pr [#1187])
- deps: update rust crate proc-macro2 to 1.0.92(pr [#1188])
- deps: update rust crate serde_json to 1.0.133(pr [#1192])
- deps: update rust crate syn to 2.0.89(pr [#1193])
- deps: update rust crate claims to 0.8.0(pr [#1194])
- deps: update rust crate clap-verbosity-flag to v3(pr [#1195])
- circleci: update test command in audit step(pr [#1212])

## [2.8.8] - 2024-11-16

### Fixed

- deps: update rust crate thiserror to 2.0.2(pr [#1179])
- deps: update rust crate thiserror to 2.0.3(pr [#1180])
- deps: update rust crate serde to 1.0.215(pr [#1181])
- deps: update github/codeql-action action to v3.27.2(pr [#1182])
- deps: update github/codeql-action action to v3.27.4(pr [#1183])
- deps: update rust crate clap to 4.5.21(pr [#1184])

## [2.8.7] - 2024-11-09

### Changed

- ci-add code coverage job to CircleCI config(pr [#1169])
- chore(circleci)-update toolkit orb to version 1.16.1(pr [#1171])
- chore(circleci)-remove custom code coverage job and use toolkit/code_coverage instead(pr [#1173])

### Fixed

- deps: update rust crate syn to 2.0.87(pr [#1165])
- deps: update rust crate thiserror to 1.0.67(pr [#1166])
- deps: update rust crate url to 2.5.3(pr [#1167])
- deps: update rust crate thiserror to 1.0.68(pr [#1168])
- deps: update rust crate thiserror to v2(pr [#1170])
- deps: update dependency toolkit to v1.17.0(pr [#1172])
- deps: update rust crate tokio to 1.41.1(pr [#1174])
- deps: update dependency toolkit to v1.18.0(pr [#1175])
- deps: update dependency toolkit to v1.19.0(pr [#1176])
- deps: update github/codeql-action action to v3.27.1(pr [#1177])
- deps: update rust crate thiserror to 2.0.1(pr [#1178])

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
- Constrain sitekey string to an Uuid
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
[#1168]: https://github.com/jerus-org/hcaptcha-rs/pull/1168
[#1169]: https://github.com/jerus-org/hcaptcha-rs/pull/1169
[#1170]: https://github.com/jerus-org/hcaptcha-rs/pull/1170
[#1171]: https://github.com/jerus-org/hcaptcha-rs/pull/1171
[#1172]: https://github.com/jerus-org/hcaptcha-rs/pull/1172
[#1173]: https://github.com/jerus-org/hcaptcha-rs/pull/1173
[#1174]: https://github.com/jerus-org/hcaptcha-rs/pull/1174
[#1175]: https://github.com/jerus-org/hcaptcha-rs/pull/1175
[#1176]: https://github.com/jerus-org/hcaptcha-rs/pull/1176
[#1177]: https://github.com/jerus-org/hcaptcha-rs/pull/1177
[#1178]: https://github.com/jerus-org/hcaptcha-rs/pull/1178
[#1179]: https://github.com/jerus-org/hcaptcha-rs/pull/1179
[#1180]: https://github.com/jerus-org/hcaptcha-rs/pull/1180
[#1181]: https://github.com/jerus-org/hcaptcha-rs/pull/1181
[#1182]: https://github.com/jerus-org/hcaptcha-rs/pull/1182
[#1183]: https://github.com/jerus-org/hcaptcha-rs/pull/1183
[#1184]: https://github.com/jerus-org/hcaptcha-rs/pull/1184
[#1185]: https://github.com/jerus-org/hcaptcha-rs/pull/1185
[#1186]: https://github.com/jerus-org/hcaptcha-rs/pull/1186
[#1187]: https://github.com/jerus-org/hcaptcha-rs/pull/1187
[#1188]: https://github.com/jerus-org/hcaptcha-rs/pull/1188
[#1189]: https://github.com/jerus-org/hcaptcha-rs/pull/1189
[#1190]: https://github.com/jerus-org/hcaptcha-rs/pull/1190
[#1192]: https://github.com/jerus-org/hcaptcha-rs/pull/1192
[#1193]: https://github.com/jerus-org/hcaptcha-rs/pull/1193
[#1194]: https://github.com/jerus-org/hcaptcha-rs/pull/1194
[#1195]: https://github.com/jerus-org/hcaptcha-rs/pull/1195
[#1197]: https://github.com/jerus-org/hcaptcha-rs/pull/1197
[#1198]: https://github.com/jerus-org/hcaptcha-rs/pull/1198
[#1200]: https://github.com/jerus-org/hcaptcha-rs/pull/1200
[#1201]: https://github.com/jerus-org/hcaptcha-rs/pull/1201
[#1203]: https://github.com/jerus-org/hcaptcha-rs/pull/1203
[#1204]: https://github.com/jerus-org/hcaptcha-rs/pull/1204
[#1207]: https://github.com/jerus-org/hcaptcha-rs/pull/1207
[#1212]: https://github.com/jerus-org/hcaptcha-rs/pull/1212
[#1205]: https://github.com/jerus-org/hcaptcha-rs/pull/1205
[#1206]: https://github.com/jerus-org/hcaptcha-rs/pull/1206
[#1208]: https://github.com/jerus-org/hcaptcha-rs/pull/1208
[#1209]: https://github.com/jerus-org/hcaptcha-rs/pull/1209
[#1210]: https://github.com/jerus-org/hcaptcha-rs/pull/1210
[#1211]: https://github.com/jerus-org/hcaptcha-rs/pull/1211
[#1213]: https://github.com/jerus-org/hcaptcha-rs/pull/1213
[#1214]: https://github.com/jerus-org/hcaptcha-rs/pull/1214
[#1215]: https://github.com/jerus-org/hcaptcha-rs/pull/1215
[#1216]: https://github.com/jerus-org/hcaptcha-rs/pull/1216
[#1217]: https://github.com/jerus-org/hcaptcha-rs/pull/1217
[#1218]: https://github.com/jerus-org/hcaptcha-rs/pull/1218
[#1219]: https://github.com/jerus-org/hcaptcha-rs/pull/1219
[#1221]: https://github.com/jerus-org/hcaptcha-rs/pull/1221
[#1222]: https://github.com/jerus-org/hcaptcha-rs/pull/1222
[#1223]: https://github.com/jerus-org/hcaptcha-rs/pull/1223
[#1224]: https://github.com/jerus-org/hcaptcha-rs/pull/1224
[#1225]: https://github.com/jerus-org/hcaptcha-rs/pull/1225
[#1226]: https://github.com/jerus-org/hcaptcha-rs/pull/1226
[#1227]: https://github.com/jerus-org/hcaptcha-rs/pull/1227
[#1228]: https://github.com/jerus-org/hcaptcha-rs/pull/1228
[#1229]: https://github.com/jerus-org/hcaptcha-rs/pull/1229
[#1231]: https://github.com/jerus-org/hcaptcha-rs/pull/1231
[#1230]: https://github.com/jerus-org/hcaptcha-rs/pull/1230
[#1232]: https://github.com/jerus-org/hcaptcha-rs/pull/1232
[#1233]: https://github.com/jerus-org/hcaptcha-rs/pull/1233
[#1234]: https://github.com/jerus-org/hcaptcha-rs/pull/1234
[#1235]: https://github.com/jerus-org/hcaptcha-rs/pull/1235
[#1236]: https://github.com/jerus-org/hcaptcha-rs/pull/1236
[#1237]: https://github.com/jerus-org/hcaptcha-rs/pull/1237
[#1238]: https://github.com/jerus-org/hcaptcha-rs/pull/1238
[#1239]: https://github.com/jerus-org/hcaptcha-rs/pull/1239
[#1240]: https://github.com/jerus-org/hcaptcha-rs/pull/1240
[#1241]: https://github.com/jerus-org/hcaptcha-rs/pull/1241
[#1242]: https://github.com/jerus-org/hcaptcha-rs/pull/1242
[#1243]: https://github.com/jerus-org/hcaptcha-rs/pull/1243
[#1244]: https://github.com/jerus-org/hcaptcha-rs/pull/1244
[#1245]: https://github.com/jerus-org/hcaptcha-rs/pull/1245
[#1246]: https://github.com/jerus-org/hcaptcha-rs/pull/1246
[#1247]: https://github.com/jerus-org/hcaptcha-rs/pull/1247
[#1248]: https://github.com/jerus-org/hcaptcha-rs/pull/1248
[#1249]: https://github.com/jerus-org/hcaptcha-rs/pull/1249
[#1250]: https://github.com/jerus-org/hcaptcha-rs/pull/1250
[#1251]: https://github.com/jerus-org/hcaptcha-rs/pull/1251
[#1252]: https://github.com/jerus-org/hcaptcha-rs/pull/1252
[#1253]: https://github.com/jerus-org/hcaptcha-rs/pull/1253
[#1254]: https://github.com/jerus-org/hcaptcha-rs/pull/1254
[#1255]: https://github.com/jerus-org/hcaptcha-rs/pull/1255
[#1256]: https://github.com/jerus-org/hcaptcha-rs/pull/1256
[#1257]: https://github.com/jerus-org/hcaptcha-rs/pull/1257
[#1258]: https://github.com/jerus-org/hcaptcha-rs/pull/1258
[#1259]: https://github.com/jerus-org/hcaptcha-rs/pull/1259
[#1260]: https://github.com/jerus-org/hcaptcha-rs/pull/1260
[#1261]: https://github.com/jerus-org/hcaptcha-rs/pull/1261
[#1262]: https://github.com/jerus-org/hcaptcha-rs/pull/1262
[#1263]: https://github.com/jerus-org/hcaptcha-rs/pull/1263
[#1264]: https://github.com/jerus-org/hcaptcha-rs/pull/1264
[#1265]: https://github.com/jerus-org/hcaptcha-rs/pull/1265
[#1266]: https://github.com/jerus-org/hcaptcha-rs/pull/1266
[#1267]: https://github.com/jerus-org/hcaptcha-rs/pull/1267
[#1268]: https://github.com/jerus-org/hcaptcha-rs/pull/1268
[#1269]: https://github.com/jerus-org/hcaptcha-rs/pull/1269
[#1270]: https://github.com/jerus-org/hcaptcha-rs/pull/1270
[#1273]: https://github.com/jerus-org/hcaptcha-rs/pull/1273
[#1271]: https://github.com/jerus-org/hcaptcha-rs/pull/1271
[#1272]: https://github.com/jerus-org/hcaptcha-rs/pull/1272
[#1274]: https://github.com/jerus-org/hcaptcha-rs/pull/1274
[#1275]: https://github.com/jerus-org/hcaptcha-rs/pull/1275
[#1276]: https://github.com/jerus-org/hcaptcha-rs/pull/1276
[#1277]: https://github.com/jerus-org/hcaptcha-rs/pull/1277
[#1278]: https://github.com/jerus-org/hcaptcha-rs/pull/1278
[#1279]: https://github.com/jerus-org/hcaptcha-rs/pull/1279
[#1280]: https://github.com/jerus-org/hcaptcha-rs/pull/1280
[#1281]: https://github.com/jerus-org/hcaptcha-rs/pull/1281
[#1282]: https://github.com/jerus-org/hcaptcha-rs/pull/1282
[#1283]: https://github.com/jerus-org/hcaptcha-rs/pull/1283
[#1284]: https://github.com/jerus-org/hcaptcha-rs/pull/1284
[#1285]: https://github.com/jerus-org/hcaptcha-rs/pull/1285
[#1286]: https://github.com/jerus-org/hcaptcha-rs/pull/1286
[#1287]: https://github.com/jerus-org/hcaptcha-rs/pull/1287
[#1288]: https://github.com/jerus-org/hcaptcha-rs/pull/1288
[#1289]: https://github.com/jerus-org/hcaptcha-rs/pull/1289
[#1290]: https://github.com/jerus-org/hcaptcha-rs/pull/1290
[#1291]: https://github.com/jerus-org/hcaptcha-rs/pull/1291
[#1292]: https://github.com/jerus-org/hcaptcha-rs/pull/1292
[#1293]: https://github.com/jerus-org/hcaptcha-rs/pull/1293
[#1294]: https://github.com/jerus-org/hcaptcha-rs/pull/1294
[#1295]: https://github.com/jerus-org/hcaptcha-rs/pull/1295
[#1296]: https://github.com/jerus-org/hcaptcha-rs/pull/1296
[#1297]: https://github.com/jerus-org/hcaptcha-rs/pull/1297
[#1298]: https://github.com/jerus-org/hcaptcha-rs/pull/1298
[#1299]: https://github.com/jerus-org/hcaptcha-rs/pull/1299
[#1300]: https://github.com/jerus-org/hcaptcha-rs/pull/1300
[#1301]: https://github.com/jerus-org/hcaptcha-rs/pull/1301
[#1302]: https://github.com/jerus-org/hcaptcha-rs/pull/1302
[#1303]: https://github.com/jerus-org/hcaptcha-rs/pull/1303
[#1304]: https://github.com/jerus-org/hcaptcha-rs/pull/1304
[#1305]: https://github.com/jerus-org/hcaptcha-rs/pull/1305
[#1306]: https://github.com/jerus-org/hcaptcha-rs/pull/1306
[#1307]: https://github.com/jerus-org/hcaptcha-rs/pull/1307
[#1308]: https://github.com/jerus-org/hcaptcha-rs/pull/1308
[#1309]: https://github.com/jerus-org/hcaptcha-rs/pull/1309
[#1314]: https://github.com/jerus-org/hcaptcha-rs/pull/1314
[#1310]: https://github.com/jerus-org/hcaptcha-rs/pull/1310
[#1311]: https://github.com/jerus-org/hcaptcha-rs/pull/1311
[#1312]: https://github.com/jerus-org/hcaptcha-rs/pull/1312
[#1313]: https://github.com/jerus-org/hcaptcha-rs/pull/1313
[#1315]: https://github.com/jerus-org/hcaptcha-rs/pull/1315
[#1316]: https://github.com/jerus-org/hcaptcha-rs/pull/1316
[#1317]: https://github.com/jerus-org/hcaptcha-rs/pull/1317
[#1318]: https://github.com/jerus-org/hcaptcha-rs/pull/1318
[#1319]: https://github.com/jerus-org/hcaptcha-rs/pull/1319
[#1320]: https://github.com/jerus-org/hcaptcha-rs/pull/1320
[#1321]: https://github.com/jerus-org/hcaptcha-rs/pull/1321
[#1322]: https://github.com/jerus-org/hcaptcha-rs/pull/1322
[#1323]: https://github.com/jerus-org/hcaptcha-rs/pull/1323
[#1324]: https://github.com/jerus-org/hcaptcha-rs/pull/1324
[#1326]: https://github.com/jerus-org/hcaptcha-rs/pull/1326
[#1327]: https://github.com/jerus-org/hcaptcha-rs/pull/1327
[#1328]: https://github.com/jerus-org/hcaptcha-rs/pull/1328
[#1329]: https://github.com/jerus-org/hcaptcha-rs/pull/1329
[#1330]: https://github.com/jerus-org/hcaptcha-rs/pull/1330
[#1331]: https://github.com/jerus-org/hcaptcha-rs/pull/1331
[#1332]: https://github.com/jerus-org/hcaptcha-rs/pull/1332
[#1333]: https://github.com/jerus-org/hcaptcha-rs/pull/1333
[#1334]: https://github.com/jerus-org/hcaptcha-rs/pull/1334
[#1335]: https://github.com/jerus-org/hcaptcha-rs/pull/1335
[#1336]: https://github.com/jerus-org/hcaptcha-rs/pull/1336
[#1337]: https://github.com/jerus-org/hcaptcha-rs/pull/1337
[#1338]: https://github.com/jerus-org/hcaptcha-rs/pull/1338
[#1339]: https://github.com/jerus-org/hcaptcha-rs/pull/1339
[#1340]: https://github.com/jerus-org/hcaptcha-rs/pull/1340
[#1341]: https://github.com/jerus-org/hcaptcha-rs/pull/1341
[#1342]: https://github.com/jerus-org/hcaptcha-rs/pull/1342
[#1344]: https://github.com/jerus-org/hcaptcha-rs/pull/1344
[#1345]: https://github.com/jerus-org/hcaptcha-rs/pull/1345
[#1346]: https://github.com/jerus-org/hcaptcha-rs/pull/1346
[#1347]: https://github.com/jerus-org/hcaptcha-rs/pull/1347
[#1348]: https://github.com/jerus-org/hcaptcha-rs/pull/1348
[#1349]: https://github.com/jerus-org/hcaptcha-rs/pull/1349
[#1353]: https://github.com/jerus-org/hcaptcha-rs/pull/1353
[#1350]: https://github.com/jerus-org/hcaptcha-rs/pull/1350
[#1351]: https://github.com/jerus-org/hcaptcha-rs/pull/1351
[#1352]: https://github.com/jerus-org/hcaptcha-rs/pull/1352
[#1354]: https://github.com/jerus-org/hcaptcha-rs/pull/1354
[#1355]: https://github.com/jerus-org/hcaptcha-rs/pull/1355
[#1356]: https://github.com/jerus-org/hcaptcha-rs/pull/1356
[#1357]: https://github.com/jerus-org/hcaptcha-rs/pull/1357
[#1358]: https://github.com/jerus-org/hcaptcha-rs/pull/1358
[#1359]: https://github.com/jerus-org/hcaptcha-rs/pull/1359
[#1360]: https://github.com/jerus-org/hcaptcha-rs/pull/1360
[#1361]: https://github.com/jerus-org/hcaptcha-rs/pull/1361
[#1362]: https://github.com/jerus-org/hcaptcha-rs/pull/1362
[#1363]: https://github.com/jerus-org/hcaptcha-rs/pull/1363
[#1366]: https://github.com/jerus-org/hcaptcha-rs/pull/1366
[#1364]: https://github.com/jerus-org/hcaptcha-rs/pull/1364
[#1365]: https://github.com/jerus-org/hcaptcha-rs/pull/1365
[#1367]: https://github.com/jerus-org/hcaptcha-rs/pull/1367
[#1368]: https://github.com/jerus-org/hcaptcha-rs/pull/1368
[#1369]: https://github.com/jerus-org/hcaptcha-rs/pull/1369
[#1370]: https://github.com/jerus-org/hcaptcha-rs/pull/1370
[#1371]: https://github.com/jerus-org/hcaptcha-rs/pull/1371
[#1372]: https://github.com/jerus-org/hcaptcha-rs/pull/1372
[#1373]: https://github.com/jerus-org/hcaptcha-rs/pull/1373
[#1374]: https://github.com/jerus-org/hcaptcha-rs/pull/1374
[#1375]: https://github.com/jerus-org/hcaptcha-rs/pull/1375
[#1376]: https://github.com/jerus-org/hcaptcha-rs/pull/1376
[#1377]: https://github.com/jerus-org/hcaptcha-rs/pull/1377
[#1378]: https://github.com/jerus-org/hcaptcha-rs/pull/1378
[#1379]: https://github.com/jerus-org/hcaptcha-rs/pull/1379
[#1381]: https://github.com/jerus-org/hcaptcha-rs/pull/1381
[#1380]: https://github.com/jerus-org/hcaptcha-rs/pull/1380
[#1382]: https://github.com/jerus-org/hcaptcha-rs/pull/1382
[#1383]: https://github.com/jerus-org/hcaptcha-rs/pull/1383
[#1384]: https://github.com/jerus-org/hcaptcha-rs/pull/1384
[#1385]: https://github.com/jerus-org/hcaptcha-rs/pull/1385
[#1386]: https://github.com/jerus-org/hcaptcha-rs/pull/1386
[#1387]: https://github.com/jerus-org/hcaptcha-rs/pull/1387
[#1388]: https://github.com/jerus-org/hcaptcha-rs/pull/1388
[#1389]: https://github.com/jerus-org/hcaptcha-rs/pull/1389
[#1390]: https://github.com/jerus-org/hcaptcha-rs/pull/1390
[#1391]: https://github.com/jerus-org/hcaptcha-rs/pull/1391
[#1392]: https://github.com/jerus-org/hcaptcha-rs/pull/1392
[#1393]: https://github.com/jerus-org/hcaptcha-rs/pull/1393
[#1394]: https://github.com/jerus-org/hcaptcha-rs/pull/1394
[#1395]: https://github.com/jerus-org/hcaptcha-rs/pull/1395
[#1396]: https://github.com/jerus-org/hcaptcha-rs/pull/1396
[#1397]: https://github.com/jerus-org/hcaptcha-rs/pull/1397
[#1398]: https://github.com/jerus-org/hcaptcha-rs/pull/1398
[#1399]: https://github.com/jerus-org/hcaptcha-rs/pull/1399
[#1400]: https://github.com/jerus-org/hcaptcha-rs/pull/1400
[#1402]: https://github.com/jerus-org/hcaptcha-rs/pull/1402
[#1403]: https://github.com/jerus-org/hcaptcha-rs/pull/1403
[#1404]: https://github.com/jerus-org/hcaptcha-rs/pull/1404
[#1405]: https://github.com/jerus-org/hcaptcha-rs/pull/1405
[#1406]: https://github.com/jerus-org/hcaptcha-rs/pull/1406
[#1407]: https://github.com/jerus-org/hcaptcha-rs/pull/1407
[#1408]: https://github.com/jerus-org/hcaptcha-rs/pull/1408
[#1409]: https://github.com/jerus-org/hcaptcha-rs/pull/1409
[#1410]: https://github.com/jerus-org/hcaptcha-rs/pull/1410
[#1411]: https://github.com/jerus-org/hcaptcha-rs/pull/1411
[#1412]: https://github.com/jerus-org/hcaptcha-rs/pull/1412
[#1413]: https://github.com/jerus-org/hcaptcha-rs/pull/1413
[#1414]: https://github.com/jerus-org/hcaptcha-rs/pull/1414
[#1415]: https://github.com/jerus-org/hcaptcha-rs/pull/1415
[#1416]: https://github.com/jerus-org/hcaptcha-rs/pull/1416
[#1417]: https://github.com/jerus-org/hcaptcha-rs/pull/1417
[#1418]: https://github.com/jerus-org/hcaptcha-rs/pull/1418
[#1419]: https://github.com/jerus-org/hcaptcha-rs/pull/1419
[#1420]: https://github.com/jerus-org/hcaptcha-rs/pull/1420
[#1421]: https://github.com/jerus-org/hcaptcha-rs/pull/1421
[#1422]: https://github.com/jerus-org/hcaptcha-rs/pull/1422
[#1423]: https://github.com/jerus-org/hcaptcha-rs/pull/1423
[#1424]: https://github.com/jerus-org/hcaptcha-rs/pull/1424
[#1425]: https://github.com/jerus-org/hcaptcha-rs/pull/1425
[#1426]: https://github.com/jerus-org/hcaptcha-rs/pull/1426
[#1427]: https://github.com/jerus-org/hcaptcha-rs/pull/1427
[#1428]: https://github.com/jerus-org/hcaptcha-rs/pull/1428
[#1429]: https://github.com/jerus-org/hcaptcha-rs/pull/1429
[#1430]: https://github.com/jerus-org/hcaptcha-rs/pull/1430
[#1431]: https://github.com/jerus-org/hcaptcha-rs/pull/1431
[#1432]: https://github.com/jerus-org/hcaptcha-rs/pull/1432
[#1433]: https://github.com/jerus-org/hcaptcha-rs/pull/1433
[#1434]: https://github.com/jerus-org/hcaptcha-rs/pull/1434
[#1435]: https://github.com/jerus-org/hcaptcha-rs/pull/1435
[#1436]: https://github.com/jerus-org/hcaptcha-rs/pull/1436
[#1437]: https://github.com/jerus-org/hcaptcha-rs/pull/1437
[#1438]: https://github.com/jerus-org/hcaptcha-rs/pull/1438
[#1439]: https://github.com/jerus-org/hcaptcha-rs/pull/1439
[#1440]: https://github.com/jerus-org/hcaptcha-rs/pull/1440
[#1441]: https://github.com/jerus-org/hcaptcha-rs/pull/1441
[#1442]: https://github.com/jerus-org/hcaptcha-rs/pull/1442
[#1444]: https://github.com/jerus-org/hcaptcha-rs/pull/1444
[#1443]: https://github.com/jerus-org/hcaptcha-rs/pull/1443
[#1445]: https://github.com/jerus-org/hcaptcha-rs/pull/1445
[#1446]: https://github.com/jerus-org/hcaptcha-rs/pull/1446
[Unreleased]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.1.0...HEAD
[3.1.0]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.33...v3.1.0
[3.0.33]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.32...v3.0.33
[3.0.32]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.31...v3.0.32
[3.0.31]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.30...v3.0.31
[3.0.30]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.29...v3.0.30
[3.0.29]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.28...v3.0.29
[3.0.28]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.27...v3.0.28
[3.0.27]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.26...v3.0.27
[3.0.26]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.25...v3.0.26
[3.0.25]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.24...v3.0.25
[3.0.24]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.23...v3.0.24
[3.0.23]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.22...v3.0.23
[3.0.22]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.21...v3.0.22
[3.0.21]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.20...v3.0.21
[3.0.20]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.19...v3.0.20
[3.0.19]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.18...v3.0.19
[3.0.18]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.17...v3.0.18
[3.0.17]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.16...v3.0.17
[3.0.16]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.15...v3.0.16
[3.0.15]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.14...v3.0.15
[3.0.14]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.13...v3.0.14
[3.0.13]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.12...v3.0.13
[3.0.12]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.11...v3.0.12
[3.0.11]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.10...v3.0.11
[3.0.10]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.9...v3.0.10
[3.0.9]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.8...v3.0.9
[3.0.8]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.7...v3.0.8
[3.0.7]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.6...v3.0.7
[3.0.6]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.5...v3.0.6
[3.0.5]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.4...v3.0.5
[3.0.4]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.3...v3.0.4
[3.0.3]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.2...v3.0.3
[3.0.2]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.1...v3.0.2
[3.0.1]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.0...v3.0.1
[3.0.0]: https://github.com/jerus-org/hcaptcha-rs/compare/v3.0.0...v3.0.0
[3.0.0]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.8.10...v3.0.0
[2.8.10]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.8.10...v2.8.10
[2.8.10]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.8.10...v2.8.10
[2.8.9]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.8.9...v2.8.9
[2.8.9]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.8.8...v2.8.9
[2.8.8]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.8.7...v2.8.8
[2.8.7]: https://github.com/jerus-org/hcaptcha-rs/compare/v2.8.6...v2.8.7
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
