#!/usr/bin/env bash
set -euo pipefail

BTX_CORE_REPO="${BTX_CORE_REPO:-https://github.com/btxchain/btx.git}"
BTX_CORE_REF="${BTX_CORE_REF:-v0.29.7}"
BTX_CORE_COMMIT="${BTX_CORE_COMMIT:-2d983afab1338762b43d2614cb1104ac8a1520ec}"
BTX_ARTIFACT_VERSION="${BTX_ARTIFACT_VERSION:-v0.1.0-qt}"
BTX_USE_DEPENDS="${BTX_USE_DEPENDS:-1}"
BTX_BUILD_DEPLOY="${BTX_BUILD_DEPLOY:-0}"
BTX_RUN_SECURITY_CHECKS="${BTX_RUN_SECURITY_CHECKS:-1}"
BTX_WITH_QRENCODE="${BTX_WITH_QRENCODE:-ON}"
BTX_WERROR="${BTX_WERROR:-OFF}"
BTX_EXTRA_CXX_FLAGS="${BTX_EXTRA_CXX_FLAGS:-}"
BTX_JOBS="${BTX_JOBS:-}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

abs_path() {
  case "$1" in
    /*) printf '%s\n' "$1" ;;
    [A-Za-z]:/*|[A-Za-z]:\\*) printf '%s\n' "$1" ;;
    *) printf '%s/%s\n' "${REPO_ROOT}" "$1" ;;
  esac
}

WORK_DIR="$(abs_path "${BTX_WORK_DIR:-.work}")"
CORE_DIR="$(abs_path "${BTX_CORE_DIR:-${WORK_DIR}/btx-core}")"
BUILD_ROOT="$(abs_path "${BTX_BUILD_ROOT:-${WORK_DIR}/build}")"
STAGE_ROOT="$(abs_path "${BTX_STAGE_ROOT:-${WORK_DIR}/stage}")"
ARTIFACT_DIR="$(abs_path "${BTX_ARTIFACT_DIR:-artifacts}")"

log() {
  printf '==> %s\n' "$*"
}

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "missing required command: $1"
}

find_python() {
  if [ -n "${PYTHON:-}" ] && "${PYTHON}" --version >/dev/null 2>&1; then
    printf '%s\n' "${PYTHON}"
    return 0
  fi
  if command -v python3 >/dev/null 2>&1 && python3 --version >/dev/null 2>&1; then
    printf '%s\n' "python3"
    return 0
  fi
  if command -v python >/dev/null 2>&1 && python --version >/dev/null 2>&1; then
    printf '%s\n' "python"
    return 0
  fi
  return 1
}

jobs() {
  if [ -n "${BTX_JOBS}" ]; then
    printf '%s\n' "${BTX_JOBS}"
  elif command -v nproc >/dev/null 2>&1; then
    nproc
  elif command -v sysctl >/dev/null 2>&1; then
    sysctl -n hw.ncpu
  else
    printf '2\n'
  fi
}

host_os() {
  case "$(uname -s)" in
    Linux*) printf 'linux' ;;
    Darwin*) printf 'macos' ;;
    MINGW*|MSYS*|CYGWIN*) printf 'windows' ;;
    *) uname -s | tr '[:upper:]' '[:lower:]' ;;
  esac
}

host_arch() {
  case "$(uname -m)" in
    x86_64|amd64) printf 'x86_64' ;;
    arm64|aarch64) printf 'arm64' ;;
    *) uname -m ;;
  esac
}

default_depends_host() {
  local os arch
  os="$(host_os)"
  arch="$(host_arch)"
  case "${os}:${arch}" in
    linux:x86_64) printf 'x86_64-pc-linux-gnu' ;;
    linux:arm64) printf 'aarch64-linux-gnu' ;;
    macos:x86_64) printf 'x86_64-apple-darwin' ;;
    macos:arm64) printf 'arm64-apple-darwin' ;;
    windows:x86_64) printf 'x86_64-w64-mingw32' ;;
    *) printf '%s\n' "" ;;
  esac
}

sha256_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1"
  else
    shasum -a 256 "$1"
  fi
}

deterministic_archive() {
  local archive="$1"
  local source_dir="$2"
  local format="$3"
  "${PYTHON_BIN}" - "${source_dir}" "${archive}" "${format}" "${SOURCE_DATE_EPOCH}" <<'PY'
import datetime
import gzip
import os
import stat
import sys
import tarfile
import zipfile

source_dir, archive, fmt, epoch_raw = sys.argv[1:]
epoch = int(epoch_raw)
zip_epoch = max(epoch, 315532800)
zip_dt = tuple(datetime.datetime.utcfromtimestamp(zip_epoch).timetuple()[:6])

def iter_files(root):
    entries = []
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames.sort()
        filenames.sort()
        for name in filenames:
            full = os.path.join(dirpath, name)
            rel = os.path.relpath(full, root).replace(os.sep, "/")
            entries.append((rel, full))
    return entries

if fmt == "zip":
    with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as zf:
        for rel, full in iter_files(source_dir):
            st = os.stat(full)
            info = zipfile.ZipInfo(rel, zip_dt)
            info.external_attr = (stat.S_IMODE(st.st_mode) & 0xFFFF) << 16
            with open(full, "rb") as fh:
                zf.writestr(info, fh.read(), compress_type=zipfile.ZIP_DEFLATED, compresslevel=9)
elif fmt == "tar.gz":
    tmp_tar = archive + ".tar"
    with tarfile.open(tmp_tar, "w", format=tarfile.PAX_FORMAT) as tf:
        for rel, full in iter_files(source_dir):
            st = os.stat(full)
            info = tf.gettarinfo(full, arcname=rel)
            info.uid = 0
            info.gid = 0
            info.uname = ""
            info.gname = ""
            info.mtime = epoch
            info.mode = stat.S_IMODE(st.st_mode)
            with open(full, "rb") as fh:
                tf.addfile(info, fh)
    with open(tmp_tar, "rb") as raw, open(archive, "wb") as out:
        with gzip.GzipFile(filename="", mode="wb", fileobj=out, mtime=epoch, compresslevel=9) as gz:
            while True:
                chunk = raw.read(1024 * 1024)
                if not chunk:
                    break
                gz.write(chunk)
    os.remove(tmp_tar)
else:
    raise SystemExit(f"unsupported archive format: {fmt}")
PY
}

require_cmd git
require_cmd cmake
require_cmd make
PYTHON_BIN="$(find_python)" || die "missing required command: python3 or python"

mkdir -p "${WORK_DIR}" "${BUILD_ROOT}" "${STAGE_ROOT}" "${ARTIFACT_DIR}"

if [ ! -d "${CORE_DIR}/.git" ]; then
  log "Cloning official BTX core ${BTX_CORE_REF}"
  git clone "${BTX_CORE_REPO}" "${CORE_DIR}"
fi

log "Checking out pinned BTX core"
git -C "${CORE_DIR}" fetch --tags --force "${BTX_CORE_REPO}" "${BTX_CORE_REF}"
git -C "${CORE_DIR}" checkout --detach "${BTX_CORE_REF}"

actual_commit="$(git -C "${CORE_DIR}" rev-parse HEAD)"
[ "${actual_commit}" = "${BTX_CORE_COMMIT}" ] || die "BTX core commit mismatch: expected ${BTX_CORE_COMMIT}, got ${actual_commit}"

source_date_epoch="$(git -C "${CORE_DIR}" show -s --format=%ct HEAD)"
export SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-${source_date_epoch}}"
export ZERO_AR_DATE=1
export TZ=UTC
export LC_ALL=C
export LANG=C

target_os="${BTX_TARGET_OS:-$(host_os)}"
target_arch="${BTX_TARGET_ARCH:-$(host_arch)}"
depends_host="${BTX_DEPENDS_HOST:-$(default_depends_host)}"
if [ "${target_os}" = "windows" ] && [ -z "${BTX_EXTRA_CXX_FLAGS}" ]; then
  # Compatibility shim for the pinned official BTX core tag on MinGW.
  BTX_EXTRA_CXX_FLAGS="-include cstdint"
fi
artifact_name="btx-wallet-${BTX_ARTIFACT_VERSION}-${target_os}-${target_arch}"
build_dir="${BUILD_ROOT}/${artifact_name}"
stage_dir="${STAGE_ROOT}/${artifact_name}"

rm -rf "${build_dir}" "${stage_dir}"
mkdir -p "${build_dir}" "${stage_dir}"

if [ "${BTX_USE_DEPENDS}" = "1" ] && [ -n "${depends_host}" ] && [ -d "${CORE_DIR}/depends" ]; then
  log "Building official BTX depends tree for ${depends_host}"
  make -C "${CORE_DIR}/depends" "HOST=${depends_host}" -j"$(jobs)"
  toolchain_file="${CORE_DIR}/depends/${depends_host}/toolchain.cmake"
else
  log "Skipping depends tree; using system toolchain"
  toolchain_file=""
fi

cmake_args=(
  -S "${CORE_DIR}"
  -B "${build_dir}"
  -DCMAKE_BUILD_TYPE=Release
  -DCMAKE_INSTALL_PREFIX="${stage_dir}"
  -DBUILD_GUI=ON
  -DBUILD_DAEMON=ON
  -DBUILD_CLI=ON
  -DBUILD_TX=OFF
  -DBUILD_UTIL=OFF
  -DBUILD_WALLET_TOOL=ON
  -DBUILD_TESTS=OFF
  -DBUILD_GUI_TESTS=OFF
  -DBUILD_BENCH=OFF
  -DBUILD_FUZZ_BINARY=OFF
  -DINSTALL_MAN=OFF
  -DENABLE_WALLET=ON
  -DENABLE_HARDENING=ON
  -DREDUCE_EXPORTS=ON
  -DWERROR="${BTX_WERROR}"
  -DWITH_QRENCODE="${BTX_WITH_QRENCODE}"
  -DWITH_ZMQ=OFF
  -DWITH_MINIUPNPC=OFF
  -DWITH_NATPMP=OFF
  -DCMAKE_FIND_PACKAGE_NO_PACKAGE_REGISTRY=ON
  -DCMAKE_FIND_PACKAGE_NO_SYSTEM_PACKAGE_REGISTRY=ON
)

if [ -n "${toolchain_file}" ] && [ -f "${toolchain_file}" ]; then
  cmake_args+=("-DCMAKE_TOOLCHAIN_FILE=${toolchain_file}")
fi

if [ -n "${BTX_EXTRA_CXX_FLAGS}" ]; then
  cmake_args+=("-DCMAKE_CXX_FLAGS=${BTX_EXTRA_CXX_FLAGS}")
fi

if [ "${target_os}" = "macos" ] && command -v xcrun >/dev/null 2>&1; then
  macos_sdk_path="$(xcrun --sdk macosx --show-sdk-path 2>/dev/null || true)"
  if [ -d "${macos_sdk_path}/System/Library/Frameworks/Accelerate.framework" ]; then
    cmake_args+=("-DBTX_ACCELERATE_FRAMEWORK=${macos_sdk_path}/System/Library/Frameworks/Accelerate.framework")
  fi
fi

log "Configuring hardened release build"
cmake "${cmake_args[@]}"

log "Building btx-qt and companion CLI tools"
cmake --build "${build_dir}" --config Release -j"$(jobs)"

if [ "${BTX_RUN_SECURITY_CHECKS}" = "1" ]; then
  if cmake --build "${build_dir}" --config Release --target help | grep -Eq '(^|[[:space:]])check-security($|[[:space:]:])'; then
    log "Running upstream binary security checks"
    cmake --build "${build_dir}" --config Release --target check-security
  else
    log "No upstream check-security target generated for this platform/configuration"
  fi
fi

if [ "${BTX_BUILD_DEPLOY}" = "1" ]; then
  if cmake --build "${build_dir}" --config Release --target help | grep -Eq '(^|[[:space:]])deploy($|[[:space:]:])'; then
    log "Building upstream deploy target"
    cmake --build "${build_dir}" --config Release --target deploy -j"$(jobs)"
  else
    log "No upstream deploy target generated for this platform/configuration; continuing with signed archive"
  fi
fi

log "Installing staged release tree"
cmake --install "${build_dir}" --config Release --prefix "${stage_dir}" --strip

mkdir -p "${stage_dir}/BTX-Wallet-Phase0"
cp "${REPO_ROOT}/docs/btx-pruned.conf" "${stage_dir}/BTX-Wallet-Phase0/btx-pruned.conf"
cp "${REPO_ROOT}/docs/FIRST-RUN.md" "${stage_dir}/BTX-Wallet-Phase0/FIRST-RUN.md"
cp "${REPO_ROOT}/RELEASE-CHECKLIST.md" "${stage_dir}/BTX-Wallet-Phase0/RELEASE-CHECKLIST.md"
cp "${REPO_ROOT}/scripts/launch-btx-qt-pruned.sh" "${stage_dir}/BTX-Wallet-Phase0/launch-btx-qt-pruned.sh"
cp "${REPO_ROOT}/scripts/launch-btx-qt-pruned.cmd" "${stage_dir}/BTX-Wallet-Phase0/launch-btx-qt-pruned.cmd"
chmod 0755 "${stage_dir}/BTX-Wallet-Phase0/launch-btx-qt-pruned.sh"

cat > "${stage_dir}/BTX-Wallet-Phase0/SOURCE-MANIFEST.txt" <<EOF
BTX Wallet artifact: ${artifact_name}
Wallet release version: ${BTX_ARTIFACT_VERSION}
BTX core repository: ${BTX_CORE_REPO}
BTX core ref: ${BTX_CORE_REF}
BTX core commit: ${actual_commit}
SOURCE_DATE_EPOCH: ${SOURCE_DATE_EPOCH}
Target OS: ${target_os}
Target arch: ${target_arch}
Depends host: ${depends_host:-system}
CMake: $(cmake --version | head -n 1)
Security checks requested: ${BTX_RUN_SECURITY_CHECKS}
Warnings as errors: ${BTX_WERROR}
Extra CXX flags: ${BTX_EXTRA_CXX_FLAGS:-none}
EOF

(
  cd "${stage_dir}"
  find . -type f -print | LC_ALL=C sort | while IFS= read -r file; do
    clean="${file#./}"
    if command -v sha256sum >/dev/null 2>&1; then
      sha256sum "${clean}"
    else
      shasum -a 256 "${clean}"
    fi
  done > BTX-Wallet-Phase0/PAYLOAD-SHA256SUMS.tmp
  mv BTX-Wallet-Phase0/PAYLOAD-SHA256SUMS.tmp BTX-Wallet-Phase0/PAYLOAD-SHA256SUMS
)

if [ "${target_os}" = "windows" ]; then
  archive="${ARTIFACT_DIR}/${artifact_name}.zip"
  deterministic_archive "${archive}" "${stage_dir}" "zip"
else
  archive="${ARTIFACT_DIR}/${artifact_name}.tar.gz"
  deterministic_archive "${archive}" "${stage_dir}" "tar.gz"
fi

sha256_file "${archive}" > "${archive}.sha256"

log "Build complete"
log "Artifact: ${archive}"
log "SHA256: $(cut -d ' ' -f 1 "${archive}.sha256")"
