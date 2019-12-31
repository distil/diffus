#!/usr/bin/env bash
# ./publish.sh 0.13.37
set -o errexit -o nounset -o pipefail -o xtrace

VERSION="$1"

command -v jq > /dev/null

cargo_publish () {
    (
        cd $1

        cargo package
        echo "Files added:"
        cargo package --list

        read -r -p "Looks good to publish to crates.io? " response
        case "$response" in
            [yY][eE][sS]|[yY])
                cargo publish
                ;;
            *)
                echo "Aborted"
                exit 5
                ;;
        esac
    )
}

(
    cd "$( dirname "${BASH_SOURCE[0]}" )"

    git fetch
    test -z "$(git status --porcelain)" || (echo "Dirty repo"; exit 2)
    test -z "$(git diff origin/master)" || (echo "Not up to date with origin/master"; exit 3)

    ./test.sh

    git fetch --tags
    git tag -l | sed '/^'"${VERSION}"'$/{q2}' > /dev/null \
        || (echo "${VERSION} already exists"; exit 4)

    find . \
        -iname Cargo.toml \
        -not -path "./target/*" \
        -exec sed -i 's/^version = .*$/version = "'"${VERSION}"'"/g' '{}' \; \
        -exec sed -i 's/^\(diffus-derive = { version = "\)\([0-9]*\.[0-9]*\.[0-9]*\)\(".*\)$/\1'"${VERSION}"'\3/g' '{}' \; \
        -exec git add '{}' \;

    git diff origin/master

    read -r -p "Deploying ${VERSION}, are you sure? [y/N]? " response
    case "$response" in
        [yY][eE][sS]|[yY])
            git commit -m"Version ${VERSION}"
            git tag "${VERSION}"
            git push origin "${VERSION}"
            git push origin master
            cargo_publish diffus-derive
            until cargo_publish diffus; do
                printf '.'
                sleep 1
            done
            ;;
        *)
            git checkout .
            echo "Aborted"
            ;;
    esac
)
