#!/usr/bin/env bash
set -eu

cd "$(dirname "$0")/.."

cargo build --release

EXECUTABLE_NAME="$(basename $(pwd))"

TARGET="${HOME}/bin/${EXECUTABLE_NAME}"
if [ -L "${TARGET}" ]; then
    echo "Remove old link ${TARGET}"
    rm "${TARGET}"
fi

echo "Create new link ${TARGET}"
ln -s "$(pwd)/target/release/${EXECUTABLE_NAME}" "${TARGET}"

echo 'Done'
