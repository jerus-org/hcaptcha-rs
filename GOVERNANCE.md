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

### Pull Request Review Process

#### Internal PRs (Organization Members)

- Full CircleCI validation runs automatically with all contexts
- Includes: fmt, clippy, tests, SonarCloud, code coverage
- PRLOG.md updated automatically after merge to main
- Requires maintainer approval before merge

#### Fork PRs (External Contributors)

**Automated Testing**:
- Fork-safe CircleCI validation runs automatically
- Includes: fmt, clippy, build, all test suites, doc tests
- Excludes: SonarCloud, code coverage uploads (no context access)
- PRLOG.md updated automatically after merge to main

**Maintainer Review**:
1. Verify automated checks passed
2. Perform code review (security, quality, design)
3. Assess risk level:
   - **Low risk** (docs, simple fixes, tests < 50 lines): Merge directly
   - **Medium/High risk** (features, refactoring, new deps): Create maintainer branch for full validation

**Full Validation (Medium/High Risk)**:
- Maintainer creates branch in main repo from fork PR
- Full CircleCI validation with all contexts (SonarCloud, coverage)
- Review results before merge

**Post-Merge**:
- PRLOG.md automatically updated on main branch
- One commit added to main with changelog entry

See [docs/FORK_PR_REVIEW.md](docs/FORK_PR_REVIEW.md) for detailed review guidelines.

## Changes to this document

Proposed in PRs and approved by maintainers using the process above.
