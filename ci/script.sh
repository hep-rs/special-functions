# Exit on any error
set -eux

# Run clippy and see if it has anything to say
clippy_lints() {
    if $CLIPPY ; then
        cargo clippy $FEATURES
    fi
}

# Run the standard build and test suite.
build_and_test() {
    cargo build
    cargo test $FEATURES
}

main() {
    build_and_test
    clippy_lints
}

main
