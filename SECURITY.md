<!--
SPDX-FileCopyrightText: 2022 jerusdp

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 3.x.x   | :white_check_mark: |
| < 3.0   | :x:                |

## Reporting a Vulnerability

If you believe you have found a security vulnerability in hcaptcha, please report it to us through coordinated disclosure.

**Please do not report security vulnerabilities through public GitHub issues, discussions, or pull requests.**

Report the Advisory through [GitHub's Security Advisory](https://github.com/jerus-org/hcaptcha-rs/security/advisories) service or send an email to security[@]jerus.ie.

## Security expectations and scope

- Scope: This library verifies hCaptcha tokens server-side by calling the hCaptcha API. It does not provide bot detection, WAF capabilities, rate limiting, DoS protection, or user management.
- Trust boundaries: Inputs come from untrusted clients; network calls target hCaptcha servers over HTTPS via reqwest.
- Configuration guidance:
  - Prefer the `rustls-backend` (default) unless your environment requires native TLS.
  - Set reasonable HTTP timeouts and follow error handling best practices.
  - Do not log secrets or raw tokens; tracing does not include secret values.
- Dynamic analysis: Weekly CI runs include Miri (UB/memory safety) and libFuzzer (parser fuzzing).

## Cryptography note

This project does not implement cryptographic primitives or protocols. TLS and certificate validation are delegated to well-vetted dependencies (reqwest/rustls or native-tls). As such, cryptography-specific criteria on bestpractices.dev are Not Applicable for this project.

## Vulnerability response process

- Acknowledgement: We acknowledge valid reports within 3 business days via GitHub Security Advisories or email.
- Triage: We privately assess severity and impact; if appropriate, we open a private GitHub Security Advisory and request a CVE ID via GitHub’s CVE services.
- Remediation: Fixes are developed and reviewed in a private security branch with tests and minimal diff.
- Target timelines: For High/Critical issues, we target a patch release within 30 days (best effort); for other severities within 90 days, unless coordination or complexity requires more time.
- Disclosure: After releasing fixes, we publish the advisory (and CVE if assigned), list affected versions, and provide mitigations/workarounds if applicable.
- Credit: We credit reporters who request attribution.
- Contact: See “Reporting a Vulnerability” above for contact methods and advisory submission.

## Multi-factor authentication (2FA) policy

To reduce account-takeover risk, the project requires multi-factor authentication for all privileged access:

- Scope: Maintainers, release publishers, and anyone with write/admin access to repositories or CI must use 2FA for all relevant services (GitHub organization, crates.io, CI providers).
- Methods: Prefer phishing-resistant authenticators (FIDO2/WebAuthn hardware security keys or passkeys). TOTP (authenticator app) is acceptable. SMS-based 2FA must not be used.
- Recovery: Store recovery codes offline in a secure location. Rotate immediately if exposed or after device loss.
- Automation: Prefer GitHub Apps for automation. If using PATs or tokens, scope to the minimum permissions and shortest lifetime possible. Bot accounts must also have 2FA enabled.
- Enforcement: The GitHub organization is expected to enable “Require two-factor authentication for everyone in your organization.” Members who do not enable 2FA will automatically lose access.
