#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_DIR="${1:-${BTX_ARTIFACT_DIR:-artifacts}}"
GPG_KEY_ID="${BTX_GPG_KEY_ID:-}"
GPG_PASSPHRASE_FILE="${BTX_GPG_PASSPHRASE_FILE:-}"

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

command -v gpg >/dev/null 2>&1 || die "missing required command: gpg"
[ -d "${ARTIFACT_DIR}" ] || die "artifact directory not found: ${ARTIFACT_DIR}"

gpg_sign() {
  local target="$1"
  local args=(--batch --yes --armor --detach-sign)
  if [ -n "${GPG_KEY_ID}" ]; then
    args=(--local-user "${GPG_KEY_ID}" "${args[@]}")
  fi
  if [ -n "${GPG_PASSPHRASE_FILE}" ]; then
    args=(--pinentry-mode loopback --passphrase-file "${GPG_PASSPHRASE_FILE}" "${args[@]}")
  fi
  gpg "${args[@]}" "${target}"
}

find "${ARTIFACT_DIR}" -maxdepth 1 -type f \( -name '*.tar.gz' -o -name '*.zip' -o -name '*.dmg' -o -name '*.exe' \) -print | sort > "${ARTIFACT_DIR}/.artifact-list"
[ -s "${ARTIFACT_DIR}/.artifact-list" ] || die "no release artifacts found in ${ARTIFACT_DIR}"

(
  cd "${ARTIFACT_DIR}"
  : > SHA256SUMS
  while IFS= read -r artifact; do
    base="$(basename "${artifact}")"
    if command -v sha256sum >/dev/null 2>&1; then
      sha256sum "${base}" >> SHA256SUMS
    else
      shasum -a 256 "${base}" >> SHA256SUMS
    fi

    gpg_sign "${base}"
  done < .artifact-list

  gpg_sign SHA256SUMS
  rm -f .artifact-list
)

printf 'Signed artifacts and SHA256SUMS in %s\n' "${ARTIFACT_DIR}"
