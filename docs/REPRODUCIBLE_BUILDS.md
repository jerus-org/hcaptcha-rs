<!--
SPDX-FileCopyrightText: 2022 jerusdp

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Reproducible Builds

This document describes how to rebuild hcaptcha-rs in a way that produces bit-for-bit identical artifacts given the same inputs.

Scope:
- Library crates: `hcaptcha`, `hcaptcha_derive`
- CLI crate: `hcaptcha-cli`
- We do not distribute prebuilt binaries today; verification focuses on crate packages (`*.crate` tarballs) and any tagged release assets.

## Requirements

- Rust (MSRV) 1.88 or higher
- Cargo matching rustc above
- A pinned container image (preferred), or a Linux environment with standard toolchain
- Git checkout at the exact release tag (e.g., `v3.1.0`)

For highest assurance, build inside the pinned CI container image referenced by the CircleCI config. The config supports an overridable image parameter and can be set to an immutable `@sha256:` digest. See `/.circleci/config.yml` parameters `rust_image` and `base_image`.

## Determinism controls

The build uses the following settings:
- SOURCE_DATE_EPOCH: fixed to the timestamp of the last commit at the checked-out revision
- RUSTFLAGS: path remapping to eliminate host-specific absolute paths
- CFLAGS: path remapping for any C/C++ dependencies
- Debug info is preserved (`CARGO_PROFILE_RELEASE_DEBUG=true`), and is path-remapped

These are injected automatically in CI via a reusable `set_repro_env` command. Locally, set them before building.

## Rebuild instructions (containerized)

1. Ensure you are on the release tag:

```bash
git fetch --tags
TAG=$(git describe --tags --abbrev=0)
git checkout "$TAG"
```

2. Run a build in the CI container (replace the image with an immutable `@sha256:` digest when published):

```bash
# Example tag-based image; replace with @sha256:<digest> for strict pinning
IMAGE="jerusdp/ci-rust:1.88-wasi"

podman run --rm -it -v "$PWD":"/src":Z -w /src "$IMAGE" bash -lc '
  set -euxo pipefail
  export SOURCE_DATE_EPOCH=$(git log -1 --pretty=%ct)
  export CARGO_HOME=${CARGO_HOME:-/root/.cargo}
  export RUSTFLAGS="--remap-path-prefix=$PWD=/src --remap-path-prefix=$CARGO_HOME=/cargo ${RUSTFLAGS:-}"
  export CFLAGS="-g -O2 -fdebug-prefix-map=$PWD=/src -fmacro-prefix-map=$PWD=/src ${CFLAGS:-}"
  export CARGO_PROFILE_RELEASE_DEBUG=true
  cargo +stable package -p hcaptcha -Z unstable-options --allow-dirty --no-verify --output dist/hcaptcha.crate
  if [ -f hcaptcha-cli/Cargo.toml ]; then
    cargo +stable package -p hcaptcha-cli -Z unstable-options --allow-dirty --no-verify --output dist/hcaptcha-cli.crate
  fi
  (cd dist && sha256sum *.crate > checksums-sha256.txt)
'
```

## Verification

- Compare the checksums you computed with the checksums attached to the corresponding GitHub Release. If they match, your artifacts are bit-identical to the release artifacts for that tag.
- Alternatively, compare with the checksums shown by `crates.io` index for the published crate versions.

## Notes and caveats

- Using a different compiler backend, libc, or OS image may change output bits. Use the pinned container with an immutable digest for best results.
- Some third-party crates compile C code (e.g., TLS stacks). The provided `CFLAGS` path remapping mitigates variability from absolute paths in debug info.
- If you enable CPU-specific optimizations (e.g., `RUSTFLAGS=-C target-cpu=native`), outputs will not be reproducible across hosts.

## Maintenance

- When rotating the CI container, update the image reference in `/.circleci/config.yml` (prefer `image@sha256:<digest>`) and re-run a reproducible release to refresh published checksums.
