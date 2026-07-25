#!/bin/bash

# SPDX-FileCopyrightText: 2022 jerusdp
#
# SPDX-License-Identifier: MIT OR Apache-2.0

set -exo pipefail

NAME="CHANGE.md"
PACKAGE=hcaptcha
REPO_DIR="../../"

# Build Changelog
gen-changelog generate \
    --display-summaries \
    --name ${NAME} \
    --package ${PACKAGE} \
    --repository-dir ${REPO_DIR} \
    --next-version "$SEMVER"

# Refresh the third-party license notices so every release ships current
# attribution — the same release-time assurance exercise as the changelog above.
# Runs from the crate directory, where about.toml / about.hbs live and where
# THIRD-PARTY-LICENSES.md is packaged (it is listed in the crate `include`).
# cargo-about is provided by the release container, alongside gen-changelog.
cargo about generate about.hbs --output-file THIRD-PARTY-LICENSES.md