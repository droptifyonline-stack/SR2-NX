#!/usr/bin/env bash
set -euo pipefail
# Run only after building bevy-horizon NVK; use a disposable source staging directory
# because upstream rewrites Cargo.lock and requires its container nightly.
repo=$(cd "$(dirname "$0")/.." && pwd)
backend=${BEVY_HORIZON_DIR:?Set BEVY_HORIZON_DIR to the pinned bevy-horizon checkout}
stage=${SR2_SWITCH_STAGE:?Set SR2_SWITCH_STAGE to a new empty directory outside the repository}
if [[ -e "$stage" ]]; then echo "Staging directory must not exist" >&2; exit 1; fi
mkdir -p "$stage"
tar -C "$repo" --exclude=.git --exclude=target --exclude=rust-toolchain.toml --exclude=local-assets --exclude=private -cf - . | tar -C "$stage" -xf -
cd "$backend"
BEVY=0.18 JOBS=${JOBS:-4} scripts/build_game.sh "$stage" sr2-switch sr2-switch "SR2-NX Prototype"
