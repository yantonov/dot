#!/usr/bin/env sh

set -eu

REPO="yantonov/dot"

# One release provides both the installer script and the binary. Taken from
# master instead, download.sh would be whatever landed there a minute ago,
# and could be paired with a binary from a release that has never seen it.
# Set DOT_VERSION to install a specific release.
VERSION="${DOT_VERSION:-$(
  curl -fsSLo /dev/null -w '%{url_effective}' "https://github.com/${REPO}/releases/latest" \
  | sed 's#.*/tag/##'
)}"

case "${VERSION}" in
  ''|*/*)
    echo "Cannot detect the latest published release of ${REPO}"
    exit 1
    ;;
esac

SCRIPT_URL="https://raw.githubusercontent.com/${REPO}/${VERSION}/bin/install/download.sh"

echo "Installing dot from release ${VERSION}"

TMP_SCRIPT="$(mktemp)"
trap 'rm -f "${TMP_SCRIPT}"' EXIT

# Fetched to a file rather than piped into a shell: in 'curl | sh' the exit
# code belongs to the shell, and an empty input is a script that succeeds, so
# a missing script would pass unnoticed and the failure would surface later
# as something unrelated.
if ! curl -fsSL "${SCRIPT_URL}" -o "${TMP_SCRIPT}"; then
  echo "Cannot fetch ${SCRIPT_URL}"
  echo "Release ${VERSION} may not carry the installer script yet; set DOT_VERSION to a release that does"
  exit 1
fi

sh "${TMP_SCRIPT}" "${VERSION}"
