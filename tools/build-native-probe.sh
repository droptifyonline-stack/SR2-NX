#!/usr/bin/env bash
set -euo pipefail
repo=$(cd "$(dirname "$0")/.." && pwd)
engine=${CONTAINER_ENGINE:-docker}
image=${SR2_LIBNX_IMAGE:-sr2-libnx-toolchain:local}
stage="$repo/target/native-switch"
output="$repo/target/switch-artifacts"
mkdir -p "$stage" "$output"
cp -a "$repo/native/sr2-probe/." "$stage/"
if ! "$engine" image inspect "$image" >/dev/null 2>&1; then
    "$engine" build -t "$image" -f "$repo/native/sr2-probe/Containerfile" "$repo/native/sr2-probe"
fi
"$engine" run --rm --log-driver=none -v "$stage:/workspace" -w /workspace "$image" make -j"${JOBS:-3}"
cp "$stage/sr2-native-probe.nro" "$output/sr2-native-probe.nro"
python3 "$repo/tools/verify-nro.py" "$output/sr2-native-probe.nro" > "$output/sr2-native-probe.nro.sha256"
cat "$output/sr2-native-probe.nro.sha256"
