#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"
export PATH="$HOME/.cargo/bin:$PATH"

MESSAGE="${1:-chore: iterate rust learning site}"
BRANCH="$(git rev-parse --abbrev-ref HEAD)"

if [[ -z "$BRANCH" ]]; then
  echo "Cannot ship from a detached HEAD."
  exit 1
fi

if [[ "${SYNC_GITHUB_SECRETS:-1}" == "1" ]]; then
  ./scripts/sync-github-secrets.sh
fi

rustup target add wasm32-unknown-unknown

cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo llvm-cov -p learning_core --fail-under-lines 90 --summary-only
(cd apps/web && trunk build --release)

if [[ -f ".env" ]]; then
  set -a
  # shellcheck disable=SC1091
  source ".env"
  set +a
fi

if [[ -n "${CLOUDFLARE_API_TOKEN:-}" ]]; then
  if rg -nF "$CLOUDFLARE_API_TOKEN" \
    -g '!target' -g '!dist' -g '!.wrangler-home' -g '!.env' .; then
    echo "Cloudflare API token was found in project files. Refusing to commit."
    exit 1
  fi
fi

git add -A

if git diff --cached --quiet; then
  echo "No staged changes to commit."
else
  git commit -m "$MESSAGE"
fi

git push origin "$BRANCH"

if command -v gh >/dev/null 2>&1 && gh run --help >/dev/null 2>&1; then
  echo "Waiting for the newest GitHub Actions run on $BRANCH..."
  sleep 8
  RUN_ID="$(gh run list --branch "$BRANCH" --limit 1 --json databaseId --jq '.[0].databaseId')"
  if [[ -n "$RUN_ID" && "$RUN_ID" != "null" ]]; then
    gh run watch "$RUN_ID" --exit-status
    gh run view "$RUN_ID" --json conclusion,displayTitle,event,status,url --jq .
  else
    echo "No GitHub Actions run found for branch $BRANCH."
  fi
else
  echo "GitHub CLI is unavailable; push completed, but workflow monitoring was skipped."
fi
