#!/bin/sh

# Build an updated README
cat docs/readme/head.md > README.md
# shellcheck disable=SC2129
cat docs/readme/lib.md >> README.md
cat docs/main.md >> README.md
cat docs/readme/tail.md >> README.md   

# Build Changelog
gen-changelog generate --display-summaries --name CHANGE.md --package hcaptcha --next-version "$SEMVER"