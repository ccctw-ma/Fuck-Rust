#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"
export PATH="$HOME/.cargo/bin:$PATH"

MESSAGE="${1:-chore: iterate rust learning site}"
BRANCH="$(git rev-parse --abbrev-ref HEAD)"

is_github_cli() {
  command -v gh >/dev/null 2>&1 && gh version 2>/dev/null | grep -Eq '^gh version [0-9]+'
}

github_repo_slug() {
  local remote_url
  remote_url="$(git config --get remote.origin.url)"

  case "$remote_url" in
    git@github.com:*.git)
      remote_url="${remote_url#git@github.com:}"
      echo "${remote_url%.git}"
      ;;
    git@github.com:*)
      echo "${remote_url#git@github.com:}"
      ;;
    https://github.com/*.git)
      remote_url="${remote_url#https://github.com/}"
      echo "${remote_url%.git}"
      ;;
    https://github.com/*)
      echo "${remote_url#https://github.com/}"
      ;;
    *)
      return 1
      ;;
  esac
}

watch_github_actions_via_api() {
  local repo_slug="$1"
  local branch="$2"

  python3 - "$repo_slug" "$branch" <<'PY'
import json
import os
import sys
import time
import urllib.error
import urllib.request

repo_slug = sys.argv[1]
branch = sys.argv[2]
token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")


def github_get(url: str) -> dict:
    headers = {
        "Accept": "application/vnd.github+json",
        "User-Agent": "fuck-rust-ship-script",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    if token:
        headers["Authorization"] = f"Bearer {token}"

    request = urllib.request.Request(url, headers=headers)
    with urllib.request.urlopen(request, timeout=20) as response:
        return json.loads(response.read().decode("utf-8"))


def latest_run() -> dict | None:
    url = f"https://api.github.com/repos/{repo_slug}/actions/runs?branch={branch}&per_page=1"
    data = github_get(url)
    runs = data.get("workflow_runs", [])
    return runs[0] if runs else None


run = None
for _ in range(12):
    run = latest_run()
    if run:
        break
    time.sleep(5)

if not run:
    print(f"No GitHub Actions run found for branch {branch} via GitHub REST API.")
    sys.exit(1)

run_id = run["id"]
run_url = run["html_url"]
print(f"Watching GitHub Actions run {run_id}: {run_url}")

for _ in range(90):
    data = github_get(f"https://api.github.com/repos/{repo_slug}/actions/runs/{run_id}")
    status = data.get("status")
    conclusion = data.get("conclusion")
    title = data.get("display_title") or data.get("name") or "GitHub Actions run"
    print(f"{title}: status={status}, conclusion={conclusion}")

    if status == "completed":
        print(json.dumps({
            "conclusion": conclusion,
            "displayTitle": title,
            "event": data.get("event"),
            "status": status,
            "url": run_url,
        }, ensure_ascii=False, indent=2))
        sys.exit(0 if conclusion == "success" else 1)

    time.sleep(10)

print(f"Timed out waiting for GitHub Actions run {run_id}: {run_url}")
sys.exit(1)
PY
}

if [[ -z "$BRANCH" ]]; then
  echo "Cannot ship from a detached HEAD."
  exit 1
fi

if [[ "${SYNC_GITHUB_SECRETS:-1}" == "1" ]]; then
  if is_github_cli; then
    ./scripts/sync-github-secrets.sh
  else
    echo "GitHub CLI is unavailable or 'gh' is not GitHub CLI; skipping GitHub secrets sync."
  fi
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

if is_github_cli; then
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
  if REPO_SLUG="$(github_repo_slug)"; then
    echo "GitHub CLI is unavailable or 'gh' is not GitHub CLI; falling back to GitHub REST API workflow monitoring."
    watch_github_actions_via_api "$REPO_SLUG" "$BRANCH"
  else
    echo "GitHub CLI is unavailable or 'gh' is not GitHub CLI, and remote.origin.url is not a GitHub repository; workflow monitoring cannot continue."
    exit 1
  fi
fi
