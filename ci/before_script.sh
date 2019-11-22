#!/bin/bash

# Echo all commands before executing them
set -o xtrace
# Forbid any unset variables
set -o nounset
# Exit on any error
set -o errexit

# Install clippy and rustfmt
rustup_tools() {
    rustup component add clippy rustfmt
}

# Install cargo tools
cargo_tools() {
    if [[ "$TRAVIS_RUST_VERSION" == "stable" ]]; then
        cargo install cargo-update || echo "cargo-update already installed"
        cargo install cargo-tarpaulin || echo "cargo-travis already installed"
        # Update cached binaries
        cargo install-update -a
    fi
}

main() {
    rustup_tools
    cargo_tools
}

main
