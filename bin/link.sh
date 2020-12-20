#!/usr/bin/env bash
set -eu

cd "$(dirname "$0")/.."

EXECUTABLE_NAME="$(basename $(pwd))"

TARGET="${HOME}/bin/${EXECUTABLE_NAME}"
if [ -L "${TARGET}" ]; then
    echo 'Remove old link'
    rm "${TARGET}"
fi

echo 'Create new link'
ln -s $(pwd)/target/release/dot ${TARGET}

echo 'Done'
