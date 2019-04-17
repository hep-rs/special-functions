#!/bin/bash

# Echo all commands before executing them
set -o xtrace
# Forbid any unset variables
set -o nounset
# Exit on any error
set -o errexit

# Ensure there are no outstanding lints.
check_lints() {
    cargo clippy $FEATURES
}

# Ensure the code is correctly formatted.
check_format() {
    cargo fmt -- --check
}

# Run the test suite.
check_tests() {
    cargo test $FEATURES
}

main() {
    check_lints
    check_format
    check_tests
}

main
