#!/usr/bin/bash

# Exit on any error
set -eux

install_kcov() {
    set -e
    # Download and install kcov
    wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz -O - | tar -xz
    cd kcov-master
    mkdir build
    cd build
    cmake ..
    make -j$(nproc)
    make install DESTDIR=../../kcov-build
    cd ../..
    rm -rf kcov-master
    set +e
}

run_kcov() {
    # Run kcov on all the test suites
    for file in target/debug/special_functions-*[^\.d]; do
        mkdir -p "target/cov/$(basename $file)";
        echo "Testing $(basename $file)"
        ./kcov-build/usr/local/bin/kcov \
            --exclude-pattern=/.cargo,/usr/lib\
            --verify "target/cov/$(basename $file)" \
            "$file";
    done

    bash <(curl -s https://codecov.io/bash)
    echo "Uploaded code coverage"
}

kcov_suite() {
    if [[ "$TRAVIS_RUST_VERSION" == "stable" ]]; then
        install_kcov
        run_kcov
    fi
}

setup_git() {
    git config --global user.email "travis@travis-ci.org"
    git config --global user.name "Travis CI"
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
s#</body>#<script src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.2/MathJax.js?config=TeX-MML-AM_CHTML"></script><script>MathJax.Hub.Config({TeX: {Macros: {dd: "{\\mathop{}\\!\\mathrm{d}}"}}});</script></body>#
EOF
        MATHJAX="${MATHJAX//\\/\\\\\\\\}"

        find ./target/doc -type f -name "*.html" -print0 |
            xargs -0 sed -i "$GTM_HEADER"
        find ./target/doc -type f -name "*.html" -print0 |
            xargs -0 sed -i "$GTM_BODY"
        find ./target/doc -type f -name "*.html" -print0 |
            xargs -0 sed -i "$MATHJAX"

        setup_git
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

    fi
}

main() {
    kcov_suite
    make_doc
}

main
