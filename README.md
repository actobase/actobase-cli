# Actobase CLI

Thin Rust client for the live Actobase request and account surfaces.

Install surface:
- `https://actobase.com/install/`
- `https://actobase.com/install/release.json`

Current public commands:
- request a watch
- request QA
- check account status
- import or clear an existing API key

CLI commands:
- `auth import-key`
- `auth show`
- `auth clear`
- `account status`
- `watch request`
- `qa request`

Release helper:
- `./build_release_bundle.sh`

Smoke helper:
- `./run_cli_smoke.sh`

Default base URL:
- `https://actobase.com`

Default config path:
- `~/.config/actobase/client.json`

Quick start from source:

```bash
cargo run -- \
  --config /tmp/actobase-dev.json \
  watch request \
  --url https://example.com \
  --mode http \
  --every 60
```

Then inspect the stored account:

```bash
cargo run -- \
  --config /tmp/actobase-dev.json \
  account status
```

Once a key is stored, later `watch request` and `qa request` calls reuse the same Actobase account instead of minting a fresh one.

Equivalent curl-first docs:
- `https://actobase.com/llms.txt`
- `https://actobase.com/offer.json`
- `https://actobase.com/watch/offer.json`
- `https://actobase.com/qa/offer.json`
