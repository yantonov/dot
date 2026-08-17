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

# install.sh passes the release it resolved, so that the script and the binary
# both come from one and the same release. Run on its own, this script falls
# back to the latest published one.
LATEST_TAG="${1:-}"

if [ -z "${LATEST_TAG}" ]; then
  # The version comes from the latest published release rather than from the
  # tag list. A tag exists the moment it is pushed, while the release built
  # from it stays a draft until someone publishes it, so the newest tag can
  # easily point at assets that cannot be downloaded yet. Following the
  # redirect of the 'latest release' page also keeps this free of a json
  # parser and of the unauthenticated api rate limit.
  LATEST_TAG="$(
    curl -fsSLo /dev/null -w '%{url_effective}' "https://github.com/${REPO}/releases/latest" \
    | sed 's#.*/tag/##'
  )"
fi

case "${LATEST_TAG}" in
  ''|*/*)
    echo "Cannot detect the latest published release of ${REPO}"
    exit 1
    ;;
esac

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

# Verify before unpacking, not after: linux and git bash carry sha256sum,
# macos carries shasum. Only the hash is compared, so the file name inside the
# checksum file does not have to match the temporary one.
if command -v sha256sum >/dev/null 2>&1; then
  ACTUAL_CHECKSUM="$(sha256sum "${ARCHIVE_PATH}" | awk '{print $1}')"
elif command -v shasum >/dev/null 2>&1; then
  ACTUAL_CHECKSUM="$(shasum -a 256 "${ARCHIVE_PATH}" | awk '{print $1}')"
else
  echo "Neither sha256sum nor shasum is available to verify the download"
  rm -rf "${TMP_DIR}"
  exit 1
fi

EXPECTED_CHECKSUM="$(awk '{print $1}' "${CHECKSUM_PATH}")"

if [ "${ACTUAL_CHECKSUM}" != "${EXPECTED_CHECKSUM}" ]; then
  echo "Checksum mismatch for ${ARCHIVE_NAME}"
  echo "  expected ${EXPECTED_CHECKSUM}"
  echo "  actual   ${ACTUAL_CHECKSUM}"
  rm -rf "${TMP_DIR}"
  exit 1
fi

echo "Checksum ok: ${ACTUAL_CHECKSUM}"

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

# Colon-wrapped comparison so the check matches a whole PATH entry rather
# than a substring of a longer, unrelated one.
case ":${PATH}:" in
  *":${TARGET_DIR}:"*)
    ;;
  *)
    echo "Note: ${TARGET_DIR} is not on your PATH yet. Add it, e.g.:"
    echo "  export PATH=\"${TARGET_DIR}:\$PATH\""
    ;;
esac
