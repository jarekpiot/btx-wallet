#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_DIR="${1:-${BTX_ARTIFACT_DIR:-artifacts}}"

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

command -v gpg >/dev/null 2>&1 || die "missing required command: gpg"
[ -d "${ARTIFACT_DIR}" ] || die "artifact directory not found: ${ARTIFACT_DIR}"
[ -f "${ARTIFACT_DIR}/SHA256SUMS" ] || die "missing ${ARTIFACT_DIR}/SHA256SUMS"
[ -f "${ARTIFACT_DIR}/SHA256SUMS.asc" ] || die "missing ${ARTIFACT_DIR}/SHA256SUMS.asc"

(
  cd "${ARTIFACT_DIR}"
  gpg --batch --verify SHA256SUMS.asc SHA256SUMS
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum -c SHA256SUMS
  else
    shasum -a 256 -c SHA256SUMS
  fi

  while IFS= read -r line; do
    artifact="${line#*  }"
    [ -f "${artifact}.asc" ] || die "missing detached signature for ${artifact}"
    gpg --batch --verify "${artifact}.asc" "${artifact}"
  done < SHA256SUMS
)

printf 'Release artifact verification passed for %s\n' "${ARTIFACT_DIR}"
