#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_PATH="$(mktemp)"
WATCH_JSON="$(mktemp)"
QA_JSON="$(mktemp)"
STATUS_JSON="$(mktemp)"
BASE_URL="${ACTOBASE_BASE_URL:-https://actobase.com}"

rm -f "$CONFIG_PATH"

cleanup() {
    rm -f "$CONFIG_PATH" "$WATCH_JSON" "$QA_JSON" "$STATUS_JSON"
}
trap cleanup EXIT

cargo run --manifest-path "${ROOT_DIR}/Cargo.toml" -- --config "$CONFIG_PATH" --base-url "$BASE_URL" \
    watch request --url https://example.com --mode http --every 60 > "$WATCH_JSON"

cargo run --manifest-path "${ROOT_DIR}/Cargo.toml" -- --config "$CONFIG_PATH" --base-url "$BASE_URL" \
    qa request --site https://example.com --focus copy --cadence one-time > "$QA_JSON"

cargo run --manifest-path "${ROOT_DIR}/Cargo.toml" -- --config "$CONFIG_PATH" --base-url "$BASE_URL" \
    account status > "$STATUS_JSON"

python3 - <<'PY' "$WATCH_JSON" "$QA_JSON" "$STATUS_JSON"
import json
import sys

watch_path, qa_path, status_path = sys.argv[1:4]
with open(watch_path, "r", encoding="utf-8") as handle:
    watch = json.load(handle)
with open(qa_path, "r", encoding="utf-8") as handle:
    qa = json.load(handle)
with open(status_path, "r", encoding="utf-8") as handle:
    status = json.load(handle)

watch_account = watch["account"]["account_id"]
qa_account = qa["account"]["account_id"]
status_account = status["account_id"]

if watch_account != qa_account or qa_account != status_account:
    raise SystemExit("cli smoke failed: account reuse mismatch")

if qa["estimate"]["pricing_model"] != "deterministic_steps_plus_optional_investigation":
    raise SystemExit("cli smoke failed: unexpected qa pricing model")

print("cli smoke passed")
PY
