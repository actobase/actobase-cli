#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIST_DIR="${ROOT_DIR}/dist"
TARGET_DIR="${ROOT_DIR}/target/release"

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$ARCH" in
    x86_64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
esac

VERSION="$(grep '^version =' "${ROOT_DIR}/Cargo.toml" | head -n1 | sed -E 's/.*"([^"]+)".*/\1/')"
BUNDLE_NAME="actobase-${VERSION}-${OS}-${ARCH}"
BUNDLE_DIR="${DIST_DIR}/${BUNDLE_NAME}"
ASSET_NAME="${BUNDLE_NAME}.tar.gz"
SHA256_NAME="${ASSET_NAME}.sha256"
RELEASE_JSON_PATH="${DIST_DIR}/release.json"

mkdir -p "$DIST_DIR"
cargo build --manifest-path "${ROOT_DIR}/Cargo.toml" --release
rm -rf "$BUNDLE_DIR"
mkdir -p "$BUNDLE_DIR"
cp "${TARGET_DIR}/actobase" "${BUNDLE_DIR}/actobase"
cp "${ROOT_DIR}/README.md" "${BUNDLE_DIR}/README.md"

(
    cd "$DIST_DIR"
    tar -czf "${ASSET_NAME}" "${BUNDLE_NAME}"
    sha256sum "${ASSET_NAME}" > "${SHA256_NAME}"
)

SHA256_VALUE="$(awk '{print $1}' "${DIST_DIR}/${SHA256_NAME}")"

python3 - "$RELEASE_JSON_PATH" "$VERSION" "$OS" "$ARCH" "$ASSET_NAME" "$SHA256_NAME" "$SHA256_VALUE" "$BUNDLE_NAME" <<'PY'
import json
import sys
from datetime import datetime, timezone

release_path, version, os_name, arch, asset_name, sha_name, sha_value, bundle_dir = sys.argv[1:9]

payload = {
    "name": "actobase",
    "version": version,
    "platform": f"{os_name}-{arch}",
    "asset_name": asset_name,
    "asset_url": f"https://actobase.com/downloads/{asset_name}",
    "sha256_name": sha_name,
    "sha256_url": f"https://actobase.com/downloads/{sha_name}",
    "sha256": sha_value,
    "bundle_dir": bundle_dir,
    "generated_at_utc": datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
}

with open(release_path, "w", encoding="utf-8") as handle:
    json.dump(payload, handle, indent=2)
    handle.write("\n")
PY

echo "release bundle created:"
echo "${DIST_DIR}/${ASSET_NAME}"
echo "${DIST_DIR}/${SHA256_NAME}"
echo "${RELEASE_JSON_PATH}"
