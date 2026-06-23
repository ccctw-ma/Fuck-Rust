#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

is_github_cli() {
  command -v gh >/dev/null 2>&1 && gh version 2>/dev/null | grep -Eq '^gh version [0-9]+'
}

if ! is_github_cli; then
  echo "GitHub CLI is unavailable or 'gh' is not GitHub CLI; skipping GitHub secrets sync."
  exit 0
fi

if [[ -f ".env" ]]; then
  set -a
  # shellcheck disable=SC1091
  source ".env"
  set +a
fi

: "${CLOUDFLARE_API_TOKEN:?CLOUDFLARE_API_TOKEN is required}"
: "${CLOUDFLARE_ACCOUNT_ID:?CLOUDFLARE_ACCOUNT_ID is required}"

PROJECT_NAME="${CLOUDFLARE_PAGES_PROJECT:-fuck-rust}"

gh secret set CLOUDFLARE_API_TOKEN --body "$CLOUDFLARE_API_TOKEN"
gh secret set CLOUDFLARE_ACCOUNT_ID --body "$CLOUDFLARE_ACCOUNT_ID"
gh secret set CLOUDFLARE_PAGES_PROJECT --body "$PROJECT_NAME"

echo "GitHub Actions secrets are synced for Cloudflare Pages project: $PROJECT_NAME"
