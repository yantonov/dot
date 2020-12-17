#!/usr/bin/env bash
set -eu

cd "$(dirname "$0")/.."

TARGET="${HOME}/bin/dot"
if [ -L "${TARGET}" ]; then
    echo 'Remove old link'
    rm "${TARGET}"
fi

echo 'Create new link'
ln -s $(pwd)/target/release/dot ${TARGET}

echo 'Done'
