#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"

toolchain="${MONTY_RUST_TOOLCHAIN:-stable}"
rustc_path="$(rustup which --toolchain "${toolchain}" rustc)"

export RUSTUP_TOOLCHAIN="${toolchain}"
export RUSTC="${rustc_path}"

echo "Using ${RUSTUP_TOOLCHAIN}: $("${RUSTC}" --version)"
echo "Running: cargo build $*"

cargo build "$@"
