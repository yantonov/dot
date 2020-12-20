#!/bin/sh

cd "$(dirname "$0")"

cd ..

EXECUTABLE_NAME="$(basename $(pwd))"

TARGET="$(pwd)/target/debug/${EXECUTABLE_NAME}"

echo "binary file is here: ${TARGET}"
