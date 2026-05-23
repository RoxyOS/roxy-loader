#!/usr/bin/env bash

set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "usage: $0 <new-version>" >&2
    exit 2
fi

new_version="$1"
export NEW_VERSION="$new_version"

if [[ ! "$new_version" =~ ^[0-9]+\.[0-9]+\.[0-9]+([-.+][0-9A-Za-z.-]+)?$ ]]; then
    echo "invalid version: $new_version" >&2
    exit 2
fi

perl -0pi -e 's/(\[package\]\nname = "roxy-loader"\nversion = ")[^"]+(")/$1$ENV{NEW_VERSION}$2/' Cargo.toml
perl -0pi -e 's/(roxy-loader-utils = \{ version = ")[^"]+(")/$1$ENV{NEW_VERSION}$2/' Cargo.toml
perl -0pi -e 's/(roxy-loader-api = \{ version = ")[^"]+(")/$1$ENV{NEW_VERSION}$2/' Cargo.toml

perl -0pi -e 's/(\[package\]\nname = "roxy-loader-api"\nversion = ")[^"]+(")/$1$ENV{NEW_VERSION}$2/' api/Cargo.toml
perl -0pi -e 's/(\[package\]\nname = "roxy-loader-utils"\nversion = ")[^"]+(")/$1$ENV{NEW_VERSION}$2/' utils/Cargo.toml
perl -0pi -e 's/(const ROXY_LOADER_ARTIFACT_VERSION: &str = ")[^"]+(";)/$1$ENV{NEW_VERSION}$2/' utils/src/build_image.rs

cargo metadata --format-version=1 >/dev/null
cargo test -p xtask
cargo test -p roxy-loader-utils
