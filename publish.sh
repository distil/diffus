#!/usr/bin/env bash
# ./publish.sh 0.13.37
set -o errexit -o nounset -o pipefail -o xtrace

VERSION="$1"
[ -z "${VERSION}" ] && echo "Need to provide a version" && exit 1

cargo_publish () {
    ( cd $1
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
                exit 3
                ;;
        esac

    )

}

( cd "$( dirname "${BASH_SOURCE[0]}" )"
    git fetch
    # FIXME test -z "$(git status --porcelain)" || (echo "dirty repo"; exit 5)
    # FIXME test -z "$(git diff origin/master)" || (echo "not up to date with origin/master"; exit 6)
    ./test.sh

    cargo fmt -- --check

    git fetch --tags
    git tag -l | sed '/^'"${VERSION}"'$/{q2}' > /dev/null \
        || (echo "${VERSION} already exists"; exit 2)

    find . -iname Cargo.toml \
        -not -path "./target/*" \
        -exec sed -i 's/^version = .*$/version = "'"${VERSION}"'"/g' '{}' \; \
        -exec git add '{}' \;

    git diff

    read -r -p "Deploying ${VERSION}, are you sure? [y/N]? " response
    case "$response" in
        [yY][eE][sS]|[yY])
            git commit -m"Version ${VERSION}"
            git tag "${VERSION}"
            git push origin "${VERSION}"
            cargo_publish diffus
            cargo_publish diffus-derive
            ;;
        *)
            git checkout .
            echo "Aborted"
            ;;
    esac
)
