#!/usr/bin/env bash

set -xeuo pipefail

export RUST_BACKTRACE=1
export CARGO_BUILD_JOBS=2

cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
cargo build --workspace --release
cargo test --workspace
cargo doc --workspace
