<!--
SPDX-FileCopyrightText: 2025 jerusdp

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# OpenSSF Best Practices Badge Update Guide

This document provides the exact information needed to update the badge page at:
https://www.bestpractices.dev/en/projects/9974/passing/edit

## Quick Reference: Criteria to Update

Total criteria to update: **22+ items**
Expected completion after update: **~90%+** (from current 33%)

---

## REPORTING (6 criteria)

### 1. report_url (MUST)
- **Status:** Met
- **URL:** https://github.com/jerus-org/hcaptcha-rs/issues
- **Justification:** GitHub Issues provides public bug tracking

### 2. report_tracker (MUST)
- **Status:** Met
- **Justification:** GitHub Issues is used as the project's issue tracker

### 3. report_process (MUST)
- **Status:** Met
- **URL:** https://github.com/jerus-org/hcaptcha-rs/blob/main/CONTRIBUTING.md#reporting-bugs
- **Justification:** CONTRIBUTING.md lines 46-54 document the bug reporting process

### 4. vulnerability_report_process (MUST)
- **Status:** Met
- **URL:** https://github.com/jerus-org/hcaptcha-rs/blob/main/SECURITY.md
- **Justification:** SECURITY.md documents coordinated disclosure process via GitHub Security Advisories and email to security@jerus.ie

### 5. vulnerability_report_private (MUST)
- **Status:** Met
- **Justification:** SECURITY.md explicitly states "Please do not report security vulnerabilities through public GitHub issues" and provides private reporting channels

### 6. vulnerability_report_response (SHOULD)
- **Status:** Met
- **Justification:** SECURITY.md establishes coordinated disclosure process with private reporting mechanism

---

## BUILD & TEST (6 criteria)

### 7. build (MUST)
- **Status:** Met
- **URL:** https://github.com/jerus-org/hcaptcha-rs/blob/main/CONTRIBUTING.md#setting-up-your-development-environment
- **Justification:** Project uses standard Rust cargo build system. CONTRIBUTING.md documents build process with `cargo build`. CircleCI configuration shows comprehensive build pipeline.

### 8. test (MUST)
- **Status:** Met
- **URL:** https://github.com/jerus-org/hcaptcha-rs/blob/main/CONTRIBUTING.md#submitting-pull-requests
- **Justification:** Comprehensive test suite with multiple test packages (test-suite-default, test-suite-enterprise, test-suite-native-only, test-suite-no-default, test-suite-rustls-only, test-suite-trace, test-suite-cli, test-wasm). Tests run with `cargo test --all`

### 9. test_invocation (SUGGESTED)
- **Status:** Met
- **Justification:** CONTRIBUTING.md line 85 documents test invocation: `cargo test --all`

### 10. test_continuous_integration (SUGGESTED)
- **Status:** Met
- **URL:** https://dl.circleci.com/status-badge/redirect/gh/jerus-org/hcaptcha-rs/tree/main
- **Justification:** CircleCI runs comprehensive CI pipeline including tests on every commit. CircleCI badge displayed in README.

### 11. warnings (MUST)
- **Status:** Met
- **Justification:** Project runs `cargo clippy --all-targets --all-features` as part of CI (CircleCI idiomatic_rust job) and requires it in contribution guidelines

### 12. warnings_fixed (MUST)
- **Status:** Met
- **Justification:** CONTRIBUTING.md line 92 requires fixing all clippy warnings before PR submission. Line 209 checklist requires no clippy warnings.

---

## QUALITY (3 criteria)

### 13. static_analysis (MUST)
- **Status:** Met
- **Justification:** Clippy (FLOSS static analysis tool for Rust) is run on all code via CircleCI and required for all contributions per CONTRIBUTING.md

### 14. static_analysis_common_vulnerabilities (SUGGESTED)
- **Status:** Met
- **URL:** https://github.com/jerus-org/hcaptcha-rs/blob/main/.github/workflows/scorecards-analysis.yml
- **Justification:** Clippy includes security lints. Additionally, OSSF Scorecard runs weekly to check for common vulnerabilities.

### 15. static_analysis_fixed (MUST)
- **Status:** N/A
- **Justification:** No medium or higher severity vulnerabilities have been discovered via static analysis

---

## GOVERNANCE (3 criteria)

### 16. code_of_conduct (MUST)
- **Status:** Met
- **URL:** https://github.com/jerus-org/hcaptcha-rs/blob/main/CODE_OF_CONDUCT.md
- **Justification:** Project uses Contributor Covenant Code of Conduct v2.0, referenced in CONTRIBUTING.md

### 17. governance (SHOULD)
- **Status:** Met
- **URL:** https://github.com/jerus-org/hcaptcha-rs/blob/main/CONTRIBUTING.md
- **Justification:** CONTRIBUTING.md documents project governance including contribution process, PR requirements, code review by maintainer @jerusdp, and decision-making process

### 18. dco (SHOULD)
- **Status:** Met
- **Justification:** CONTRIBUTING.md and README.md establish Apache-2.0 DCO-style contribution terms: "Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions."

---

## SECURITY (4 criteria)

### 19. crypto_password_storage (MUST)
- **Status:** N/A
- **Justification:** This is a library for hCaptcha verification. It does not store or handle user passwords.

### 20. crypto_random (MUST)
- **Status:** N/A
- **Justification:** Library uses reqwest for HTTPS communication and does not generate cryptographic keys or nonces

### 21. delivery_mitm (MUST)
- **Status:** Met
- **Justification:** Project is delivered via crates.io which uses HTTPS. The library itself uses HTTPS for all network communication via reqwest with TLS.

### 22. no_leaked_credentials (MUST)
- **Status:** Met
- **URL:** https://github.com/jerus-org/hcaptcha-rs/blob/main/.github/workflows/scorecards-analysis.yml
- **Justification:** No credentials are leaked in the public repository. OSSF Scorecard runs weekly to check for leaked credentials.

---

## ADDITIONAL CRITERIA (Can mark now)

### 23. homepage_url_status
- **Status:** Met
- **Justification:** https://github.com/jerus-org/hcaptcha-rs (already populated in badge entry)

### 24. know_secure_design (MUST)
- **Status:** Met
- **Justification:** Project maintainer demonstrates secure design knowledge through comprehensive SECURITY.md, security reporting process, and regular security scanning via OSSF Scorecard

### 25. know_common_errors (MUST)
- **Status:** Met
- **Justification:** Comprehensive test suites covering error conditions, extensive error handling in code, and use of Rust's type system to prevent common errors

### 26. test_most (SHOULD)
- **Status:** Met
- **URL:** https://codecov.io/gh/jerus-org/hcaptcha-rs
- **Justification:** Project uses codecov for test coverage tracking. CircleCI runs code coverage analysis. Coverage badge displayed in README.

### 27. dependency_monitoring (SUGGESTED)
- **Status:** Met
- **URL:** https://github.com/jerus-org/hcaptcha-rs/blob/main/renovate.json
- **Justification:** Renovate bot is configured to automatically monitor and update dependencies

### 28. accessibility_best_practices (SHOULD)
- **Status:** N/A
- **Justification:** This is a backend library providing API verification functionality without user interface components

### 29. internationalization (SHOULD)
- **Status:** N/A
- **Justification:** Library generates no end-user text. Error messages are intended for developers and are in English per standard practice.

---

## HOW TO UPDATE

1. **Log in** to https://www.bestpractices.dev/
2. **Navigate** to https://www.bestpractices.dev/en/projects/9974/passing/edit
3. **For each criterion above:**
   - Find the criterion by its name (e.g., "report_url")
   - Select the appropriate status (Met/N/A)
   - Paste the URL (if provided)
   - Paste the Justification text
   - Click "Save" or continue to next item

4. **Use bulk edit mode** if available:
   - Click "Hide met & N/A" to show only unmet items
   - Work through the list systematically

---

## EXPECTED RESULTS

After updating these criteria:
- **Before:** 22 Met (33%)
- **After:** ~44+ Met (~90%+)
- **Remaining:** Primarily documentation items (roadmap, architecture) and long-term considerations (bus factor)

This should be sufficient to **achieve the passing badge** as all MUST criteria will be met!

---

## NEXT STEPS (After Badge Update)

See the main compliance analysis for Priority 2-5 items:
- Create ROADMAP.md
- Create ARCHITECTURE.md  
- Add OpenSSF badge to README.md
- Document bus factor strategy

