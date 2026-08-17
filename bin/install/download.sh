#!/usr/bin/env sh

set -eu

SCRIPT="$(basename "$0")"
cd "$(dirname "$0")"

# Detect OS
case "$(uname -s)" in
  Linux*)
    OS="linux"
    ;;
  Darwin*)
    OS="macos"
    ;;
  MINGW*|MSYS*|CYGWIN*)
    OS="windows"
    ;;
  *)
    echo "Unsupported OS: $(uname -s)"
    exit 1
    ;;
esac

REPO="yantonov/dot"
# /releases/latest returns the latest published (non-draft, non-prerelease)
# release, unlike /tags which isn't documented as sorted by recency and
# would also happily point at a tag whose release is still a draft.
LATEST_TAG=$(
  curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
  | grep '"tag_name"' \
  | head -n 1 \
  | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/'
)

if [ -z "${LATEST_TAG}" ] || [ "${LATEST_TAG}" = "null" ]; then
  echo "Could not determine the latest published release for ${REPO}"
  exit 1
fi

APP_NAME="dot"
EXECUTABLE_FILENAME="${APP_NAME}"
ARCHIVE_NAME="${EXECUTABLE_FILENAME}-${OS}-${LATEST_TAG}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/${ARCHIVE_NAME}"

echo "Latest tag: ${LATEST_TAG}"
echo "Downloading: ${DOWNLOAD_URL}"

TMP_DIR="$(mktemp -d)"
ARCHIVE_PATH="${TMP_DIR}/${ARCHIVE_NAME}"
CHECKSUM_PATH="${ARCHIVE_PATH}.sha256"

echo $ARCHIVE_PATH

# Download archive
curl -fL "${DOWNLOAD_URL}" -o "${ARCHIVE_PATH}"

# Download and verify checksum
curl -fL "${DOWNLOAD_URL}.sha256" -o "${CHECKSUM_PATH}"

if command -v sha256sum >/dev/null 2>&1; then
  CHECKSUM_TOOL="sha256sum"
elif command -v shasum >/dev/null 2>&1; then
  CHECKSUM_TOOL="shasum -a 256"
else
  echo "Neither sha256sum nor shasum is available to verify the download"
  rm -rf "${TMP_DIR}"
  exit 1
fi

if ! (cd "${TMP_DIR}" && ${CHECKSUM_TOOL} -c "$(basename "${CHECKSUM_PATH}")"); then
  echo "Checksum verification failed for ${ARCHIVE_PATH}"
  rm -rf "${TMP_DIR}"
  exit 1
fi

# Extract archive
tar -xzf "${ARCHIVE_PATH}" -C "${TMP_DIR}"

# Find binary inside extracted files
BIN_PATH="$(find "${TMP_DIR}" -type f -exec sh -c 'test -x "$1"' _ {} \; -print | head -n 1)"

if [ -z "${BIN_PATH}" ]; then
  echo "Executable ${EXECUTABLE_FILENAME} is not found in the archive ${TMP_DIR}"
  rm -rf "${TMP_DIR}"
  exit 1
fi

TARGET_DIR="${HOME}/.local/bin"
mkdir -p "${TARGET_DIR}"

# Preserve the archived binary's own name (e.g. dot.exe on Windows) instead
# of assuming it always matches APP_NAME.
BINARY_NAME="$(basename "${BIN_PATH}")"

# Copy binary to the target directory
cp "${BIN_PATH}" "${TARGET_DIR}/${BINARY_NAME}"
chmod +x "${TARGET_DIR}/${BINARY_NAME}"

# Cleanup
rm -rf "${TMP_DIR}"

echo "Installed: ${TARGET_DIR}/${BINARY_NAME}"
