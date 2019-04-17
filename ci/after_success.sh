#!/bin/bash

# Echo all commands before executing them
set -o xtrace
# Forbid any unset variables
set -o nounset
# Exit on any error
set -o errexit

# We only need to run the coverage suite once
COVERAGE_RUN=false

run_kcov() {
    # Run kcov on all the test suites
    if [[ $COVERAGE_RUN != "true" ]]; then
        cargo coveralls
        COVERAGE_RUN=true
    fi
}

coverage_codecov() {
    if [[ "$TRAVIS_RUST_VERSION" != "stable" ]]; then
        return
    fi

    run_kcov

    bash <(curl -s https://codecov.io/bash) -s target/kcov
    echo "Uploaded code coverage to codecov.io"
}

coverage_coveralls() {
    if [[ "$TRAVIS_RUST_VERSION" != "stable" ]]; then
        return
    fi

    run_kcov

    # Data is automatically uploaded by kcov
}

git_setup() {
    git config --global user.email "travis@travis-ci.org"
    git config --global user.name "Travis CI"
}

git_commit_doc() {
    git clone --depth=1 --branch=gh-pages https://${GH_TOKEN}@github.com/$TRAVIS_REPO_SLUG.git ./target/doc-git

    cd ./target/doc-git
    for f in index.html; do
        cp $f ../doc/
    done
    rm -rf *
    cp -R ../doc/* .
    git add -A .
    git commit --message "Travis build: $TRAVIS_BUILD_NUMBER"
    git push
}

make_doc() {
    # Only want to update the docs when pushing to master.  Ultimately, this
    # should be changed to using a tag once things have stabilized.
    if [[ "$TRAVIS_RUST_VERSION" == "stable"
       && "$TRAVIS_EVENT_TYPE" == "push"
       && "$TRAVIS_BRANCH" == "master" ]]; then

        cargo doc $FEATURES

        read -r GTM_HEADER <<EOF
s#<head>#<head><!-- Google Tag Manager --><script>(function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],j=d.createElement(s),dl=l!='dataLayer'?'&l='+l:'';j.async=true;j.src='https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);})(window,document,'script','dataLayer','GTM-N9HX7G4');</script><!-- End Google Tag Manager -->#
EOF
        read -r GTM_BODY <<EOF
s#<body class="rustdoc mod">#<body class="rustdoc mod"><!-- Google Tag Manager (noscript) --><noscript><iframe src="https://www.googletagmanager.com/ns.html?id=GTM-N9HX7G4" height="0" width="0" style="display:none;visibility:hidden"></iframe></noscript><!-- End Google Tag Manager (noscript) -->#
EOF
        read -r MATHJAX <<EOF
s|</body>|<script src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.2/MathJax.js?config=TeX-MML-AM_CHTML"></script><script>MathJax.Hub.Config({TeX: {Macros: {dd: "{\\mathop{}\\!\\mathrm{d}}", textsc: ["\\mathrm{\\scriptsize #1}", 1], vt: ["\\boldsymbol{#1}", 1], pfrac: ["\\frac{\partial #1}{\partial #2}", 2], ddfrac: ["\\frac{\dd #1}{\dd #2}", 2], defeq: "\\mathrel{\\vcenter:}=", abs: ["\\lvert #1 \rvert", 1], angles: ["\\langle #1 \rangle", 1]}}});</script></body>|
EOF
        MATHJAX="${MATHJAX//\\/\\\\\\\\}"

        find ./target/doc -type f -name "*.html" -print0 |
            xargs -0 sed -i "$GTM_HEADER"
        find ./target/doc -type f -name "*.html" -print0 |
            xargs -0 sed -i "$GTM_BODY"
        find ./target/doc -type f -name "*.html" -print0 |
            xargs -0 sed -i "$MATHJAX"

        git_setup
        git_commit_doc
    fi
}

main() {
    coverage_coveralls
    coverage_codecov
    make_doc
}

main
