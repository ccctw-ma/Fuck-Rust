#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"
export PATH="$HOME/.cargo/bin:$PATH"

if [[ -f ".env" ]]; then
  set -a
  # shellcheck disable=SC1091
  source ".env"
  set +a
fi

: "${CLOUDFLARE_API_TOKEN:?CLOUDFLARE_API_TOKEN is required}"
: "${CLOUDFLARE_ACCOUNT_ID:?CLOUDFLARE_ACCOUNT_ID is required}"

PROJECT_NAME="${CLOUDFLARE_PAGES_PROJECT:-fuck-rust}"
export CLOUDFLARE_API_TOKEN
export CLOUDFLARE_ACCOUNT_ID
WRANGLER_HOME_DIR="$ROOT_DIR/.wrangler-home"

if ! command -v trunk >/dev/null 2>&1; then
  cargo install trunk --locked
fi

rustup target add wasm32-unknown-unknown

(cd apps/web && env -u NO_COLOR trunk build --release)

mkdir -p "$WRANGLER_HOME_DIR"

run_wrangler() {
  HOME="$WRANGLER_HOME_DIR" \
    XDG_CONFIG_HOME="$WRANGLER_HOME_DIR/.config" \
    npx --yes wrangler@3 "$@"
}

run_wrangler pages project create "$PROJECT_NAME" --production-branch main \
  || echo "Cloudflare Pages project may already exist; continuing to deploy."

run_wrangler pages deploy dist --project-name "$PROJECT_NAME" --commit-dirty=true
