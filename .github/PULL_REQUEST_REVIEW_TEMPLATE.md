<!--
SPDX-FileCopyrightText: 2025 jerusdp

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Pull Request Review Checklist

Use this checklist when reviewing pull requests, especially those from forked repositories.

## Automated Checks

- [ ] CircleCI fork-safe validation passed
  - [ ] `toolkit/idiomatic_rust` (fmt, clippy)
  - [ ] `toolkit/required_builds` (all packages)
  - [ ] `toolkit/common_tests`
  - [ ] `test-suite` (all variants)
  - [ ] `doc_tests`
  - [ ] `toolkit/test_wasm`

**Note**: Jobs requiring contexts (SonarCloud, coverage) will show "Unauthorized" for fork PRs - this is expected.

## Code Review

### Security

- [ ] No hardcoded secrets, API keys, passwords, or credentials
- [ ] Input validation appropriate for untrusted data
- [ ] No use of `unsafe` without justification and safety documentation
- [ ] Error handling doesn't expose sensitive information or internal details
- [ ] Constant-time comparisons used for sensitive data (tokens, keys, hashes)
- [ ] No logging of sensitive information or PII
- [ ] Cryptography uses well-vetted libraries (no custom crypto)

### Dependencies

- [ ] New dependencies are from trusted sources (crates.io, official repos)
- [ ] Licenses are compatible (MIT OR Apache-2.0 only)
- [ ] No suspicious, unmaintained, or abandoned packages
- [ ] Version constraints are appropriate (avoid wildcards)
- [ ] Dependency changes reviewed for security advisories
- [ ] No hallucinated or malicious dependencies

### Code Quality

- [ ] Follows Rust API Guidelines (naming, error handling, semver)
- [ ] Code is idiomatic Rust (uses standard patterns)
- [ ] No clippy warnings introduced
- [ ] Tests cover new functionality (unit + integration where appropriate)
- [ ] Documentation updated (rustdoc, README, guides)
- [ ] Public items have rustdoc comments with examples
- [ ] Code is readable and well-structured

### Testing

- [ ] Tests added for new functionality
- [ ] Tests cover edge cases and error conditions
- [ ] No duplicated test code (helpers extracted if needed)
- [ ] Tests are clear and maintainable
- [ ] Mocking is appropriate (e.g., network calls)

### DCO and Commits

- [ ] All commits have `Signed-off-by:` line (DCO)
- [ ] Commit messages follow emoji prefix convention
- [ ] Commits are atomic and logically organized

## Risk Assessment

Determine the risk level of this change:

| Change Type | Risk Level | Action |
|-------------|-----------|--------|
| Documentation only | Low | → Merge directly |
| Simple bug fix (< 50 lines) | Low | → Merge directly |
| Test additions only | Low | → Merge directly |
| Dependency patch updates | Low-Medium | → Check advisories, merge directly |
| New feature (< 200 lines) | Medium | → Consider maintainer branch |
| Dependency minor/major updates | Medium | → Create maintainer branch |
| Large refactoring | High | → Create maintainer branch |
| Security-sensitive code | High | → Maintainer branch + extra review |
| New dependencies | High | → Maintainer branch + supply chain review |

**Risk level for this PR**: [Low / Medium / High]

## Decision

### Option 1: Merge Directly (Low Risk)

- [ ] All automated checks passed
- [ ] Code review complete and approved
- [ ] No security concerns
- [ ] Risk assessment: Low
- [ ] Ready to merge directly

**Action**: Approve and merge to main. PRLOG will be updated automatically.

### Option 2: Full Validation Required (Medium/High Risk)

- [ ] Change is complex or high-risk
- [ ] Needs SonarCloud analysis
- [ ] Needs full security validation
- [ ] Create maintainer branch for full validation

**Action**: Create branch `maintainer/<branch-name>` in main repo:

```bash
# Fetch fork PR
gh pr checkout <PR_NUMBER>

# Or manually
git fetch origin pull/<PR_NUMBER>/head:pr-<PR_NUMBER>
git checkout pr-<PR_NUMBER>

# Push to maintainer branch
git push origin pr-<PR_NUMBER>:maintainer/<branch-name>
```

- [ ] Maintainer branch created
- [ ] Full CircleCI validation triggered
- [ ] SonarCloud analysis reviewed
- [ ] Code coverage reviewed
- [ ] All checks passed
- [ ] Ready to merge maintainer branch

## Additional Notes

### Security Concerns

[Document any security concerns or considerations]

### Breaking Changes

- [ ] No breaking changes
- [ ] Breaking changes documented and justified
- [ ] Breaking changes follow semver

### Follow-up Actions

[Any follow-up tasks or issues to create]

## Final Approval

- [ ] All checklist items complete
- [ ] PR approved
- [ ] Ready to merge
- [ ] Post-merge PRLOG update will occur automatically

---

**Reviewed by**: @[username]  
**Date**: [YYYY-MM-DD]

See [docs/FORK_PR_REVIEW.md](../docs/FORK_PR_REVIEW.md) for detailed review guidelines.
