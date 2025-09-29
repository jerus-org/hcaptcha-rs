#!/bin/sh

# Build Changelog
gen-changelog generate --display-summaries --name CHANGE.md --package hcaptcha_derive --next-version "$SEMVER"