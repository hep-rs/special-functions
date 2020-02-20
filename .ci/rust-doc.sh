#!/bin/bash

# Echo all commands before executing them
set -o xtrace
# Forbid any unset variables
set -o nounset
# Exit on any error
set -o errexit

# Preserve certain files from the gh-pages
for f in index.html; do
    cp "gh-pages/$f" ./target/doc/
done

# Remove everything from gh-pages and replace with the generated docs
rm -rf gh-pages/*
cp -R ./target/doc/* gh-pages/

cd gh-pages

# Prettify HTML code to allow editing
sudo apt install tidy
find . -type f -name "*.html" -print0 |
    xargs -0 -P$(nproc) tidy -m 2>/dev/null ||
    true

# Add MathJax support
find . -type f -name "*.html" -print0 |
    xargs -0 -P$(nproc) \
        sed -n -i \
            -e '/<\/head>/r ../.ci/mathjax.html' \
            -e '1x' \
            -e '2,${x;p}' \
            -e '${x;p}'

# Commit the changes
git config --global user.email "CI"
git config --global user.name "CI"
git add --all
git commit --message "CI: cargo doc ${GITHUB_SHA:0:7}"
