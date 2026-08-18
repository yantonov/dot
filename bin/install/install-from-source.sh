#!/bin/sh
set -eu

cd "$(dirname "$0")/../.."

cargo build --release

EXECUTABLE_NAME="$(basename "$(pwd)")"

# cargo appends .exe to the binary name on Windows
if [ -f "target/release/${EXECUTABLE_NAME}.exe" ]; then
    BINARY_NAME="${EXECUTABLE_NAME}.exe"
else
    BINARY_NAME="${EXECUTABLE_NAME}"
fi

TARGET_DIR="${HOME}/.local/bin"
mkdir -p "${TARGET_DIR}"

TARGET="${TARGET_DIR}/${BINARY_NAME}"

cp "$(pwd)/target/release/${BINARY_NAME}" "${TARGET}"

echo "Installed ${BINARY_NAME} to ${TARGET}"
