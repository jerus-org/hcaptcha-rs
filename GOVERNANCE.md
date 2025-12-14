<!--
SPDX-FileCopyrightText: 2025 jerusdp

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Project Governance

This document describes how the hcaptcha-rs project is governed: who has which
responsibilities, how decisions are made, and how continuity is ensured.

## Roles

- Maintainers
  - Overall responsibility for the project’s direction, releases, and security
    response. Approve and merge pull requests.
  - Current maintainer(s):
    - @jerusdp (primary)
  - Goal (Silver level): add at least one additional maintainer with publish and
    CI permissions.
- Reviewers
  - Community members who review issues/PRs and propose changes. Reviewers may
    be nominated as maintainers based on sustained, high-quality contribution.

## Decision-making

- Day-to-day: lazy consensus on issues and PRs. If no maintainer objects within
  a reasonable period, changes may proceed after review.
- Disagreements: resolved in GitHub issues/PRs. Maintainers seek consensus; if
  needed, the primary maintainer is the tie-breaker.
- Security: follow SECURITY.md for coordinated disclosure; maintainers may make
  time-critical decisions privately to protect users.

## Releases

- Release cadence: scheduled monthly on the 28th (early morning) via CI scheduled workflow; maintainers may trigger an out-of-band release for urgent fixes.
- Ownership: Maintainers are responsible for `cargo release` and GitHub releases.
- Provenance: Releases are cut from a clean `main` after CI is green; the scheduled CI job runs the release workflow to publish artifacts.

## Access & continuity

- Repositories: Maintainers hold write/admin permissions to GitHub repo.
- crates.io: At least two maintainers (target) have publish rights to `hcaptcha`
  and related crates.
- CI/secrets: Maintainers manage CI settings and secrets (CircleCI contexts).
- 2FA: All maintainers and publishers must use multi-factor authentication (2FA) on GitHub, crates.io, and CI accounts. Preferred methods are FIDO2/WebAuthn security keys or passkeys; TOTP is acceptable; SMS is not permitted. Recovery codes must be stored offline. Automation should use GitHub Apps or minimally scoped, short‑lived tokens; bot accounts must also have 2FA.
- Bus factor: Document co-maintainers in this section; onboard at least one
  additional maintainer with the permissions above.

## Process requirements

- All changes land via pull request; never commit to `main`.
- Checks: CI must be green, `cargo fmt` and `cargo clippy` must pass.
- Code of Conduct applies to all project spaces.

## Changes to this document

Proposed in PRs and approved by maintainers using the process above.
