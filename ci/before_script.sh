#!/usr/bin/bash

# Exit on any error
set -eux

# Install clippy and rustfmt
rustup_tools() {
    rustup component add clippy rustfmt
}

# Remove old builds from cache
clean() {
    find target -type f -name "special-functions-*" -exec rm '{}' +
}


main() {
    rustup_tools
    clean
}

main
