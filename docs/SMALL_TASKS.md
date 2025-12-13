<!--
SPDX-FileCopyrightText: 2025 jerusdp

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Small Tasks (Newcomer Guide)

This page explains what qualifies as a small task and how contributors can get started.

What is a small task
- Well-scoped change with clear acceptance criteria
- No API or breaking changes
- Typically 1–2 hours for someone new to the project

Examples
- Improve or add a rustdoc example
- Convert ad-hoc assertions to helper functions in tests
- Add missing unit tests for edge cases already documented
- Fix typos or clarify error messages and docs
- CI/doc build fixes that do not change behavior

How to pick one up
1. Browse issues labeled `good first issue` and `help wanted`.
2. Comment "I’d like to work on this" and wait to be assigned.
3. Follow CONTRIBUTING.md for workflow, testing, and DCO sign-off.

For maintainers: triage rules
- Keep at least 3 open `good first issue` items when possible.
- Add a short “Hints” section and acceptance checklist to each small task.
- Prefer docs/tests/CI tidy-ups for newcomers; avoid cross-crate refactors.