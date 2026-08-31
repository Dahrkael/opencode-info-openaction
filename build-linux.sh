#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

TARGET_DIR=".build-target"
OUT="opencode-info.sdPlugin"
ASSETS="assets"
TARGET="x86_64-unknown-linux-gnu"

echo "==> Building (release)..."
CARGO_TARGET_DIR="$TARGET_DIR" cargo build --release

echo "==> Assembling plugin directory from assets/..."
rm -rf "$OUT"
cp -r "$ASSETS" "$OUT"

echo "==> Copying binaries..."
BIN="$TARGET_DIR/release/opencode-info"
cp "$BIN" "$OUT/opencode-info-$TARGET"
cp "$BIN" "$OUT/opencode-info"

echo "==> Packaging streamDeckPlugin..."
rm -f opencode-info.streamDeckPlugin
# .streamDeckPlugin is a zip whose root contains the <name>.sdPlugin folder
if command -v zip >/dev/null 2>&1; then
    cd "$(dirname "$OUT")"
    zip -rq "$(basename opencode-info.streamDeckPlugin)" "$(basename "$OUT")"
    cd -
else
    python3 - "$OUT" opencode-info.streamDeckPlugin <<'PY'
import sys, zipfile, os
name, out = sys.argv[1], sys.argv[2]
with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
    for root, _dirs, files in os.walk(name):
        for f in files:
            p = os.path.join(root, f)
            z.write(p, os.path.join(name, os.path.relpath(p, name)))
PY
fi
echo "==> Done: $(pwd)/opencode-info.streamDeckPlugin"
echo "    Binary: $OUT/opencode-info-$TARGET ($(du -h "$OUT/opencode-info-$TARGET" | cut -f1))"
