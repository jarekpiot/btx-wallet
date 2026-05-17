#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ -x "${script_dir}/../bin/btx-qt" ]; then
  btx_qt="${script_dir}/../bin/btx-qt"
elif [ -x "${script_dir}/btx-qt" ]; then
  btx_qt="${script_dir}/btx-qt"
else
  printf 'error: btx-qt not found next to the release launcher\n' >&2
  exit 1
fi

exec "${btx_qt}" \
  -prune=4096 \
  -pruneduringinit=4096 \
  -retainshieldedcommitmentindex=1 \
  "$@"
