#!/usr/bin/env bash
set -eu

cd "$(dirname "$0")/.."

cargo build --release

EXECUTABLE_NAME="$(basename $(pwd))"

TARGET="${HOME}/bin/${EXECUTABLE_NAME}"
if [ -f "${TARGET}" ] || [ -L "${TARGET}" ]; then
    echo "Remove old file ${TARGET}"
    rm "${TARGET}"
fi

echo "Create new link ${TARGET}"
cp "$(pwd)/target/release/${EXECUTABLE_NAME}" "${TARGET}"

echo 'Done'
