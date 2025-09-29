#!/bin/bash
set -exo pipefail

NAME="CHANGE.md"
PACKAGE=hcaptcha_derive
REPO_DIR="../."

# Build Changelog
gen-changelog generate \
    --display-summaries \
    --name ${NAME} \
    --package ${PACKAGE} \
    --repository-dir ${REPO_DIR} \
    --next-version "$SEMVER"