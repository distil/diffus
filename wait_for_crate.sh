#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

CRATE="$1"
VERSION="$2"

until curl --silent --fail "https://crates.io/api/v1/crates/${CRATE}" | jq ".versions[].num" | grep '^"'${VERSION}'"$' > /dev/null; do
    printf '.'
    sleep 1
done
