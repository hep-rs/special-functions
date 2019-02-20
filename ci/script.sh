#!/usr/bin/bash

# Exit on any error
set -eux

# Run clippy and see if it has anything to say
clippy_lints() {
    cargo clippy $FEATURES
}

# Run rustfmt
check_format() {
    cargo fmt -- --check
}

# Run the standard build and test suite.
build_and_test() {
    cargo build $FEATURES
    cargo test $FEATURES
}

main() {
    clippy_lints
    check_format
    build_and_test
}

main
