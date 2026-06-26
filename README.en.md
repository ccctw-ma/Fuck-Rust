# Rust via ripgrep

A progressive Rust learning site implemented entirely in Rust. The curriculum now teaches Rust through ripgrep source code: every topic anchors to a real ripgrep snippet, then maps the relevant rule from The Rust Programming Language into exercises. The frontend is built with Yew and compiled to WASM, while curriculum, answer checking, and progress logic live in the pure Rust `learning_core` crate. The site deploys to Cloudflare Pages.

## Features

- Learning path follows 12 ripgrep source modules: entry and exit codes, mode dispatch, pattern loading, process resources, byte slices, writer borrowing, decompression builders, Option/Result boundaries, config collections, search pipelines, globset generics, and parallel search.
- 300+ interactive exercises: single choice, fill-in-the-blank, output prediction, and step ordering, arranged as basic, practice, and challenge questions for each module.
- Instant feedback with expected answers, explanations, and the next exercise.
- Source reading modules show the ripgrep file path, line range, source responsibility, mapped Rust Book rule, question focus, and source link.
- Local progress stored in browser `localStorage`, including language and theme preferences.
- Light/dark mode and Chinese/English UI.
- CI/CD with format checks, Clippy, tests, coverage gate, GitHub Actions deployment, and workflow monitoring.

## Local Development

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
cargo test --workspace
cd apps/web
trunk serve
```

## Quality Gates

The coverage gate targets `learning_core`, with line coverage required to stay above 90%.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo install cargo-llvm-cov --locked
cargo llvm-cov -p learning_core --fail-under-lines 90 --summary-only
cd apps/web
trunk build --release
```

## Ship Flow

After each iteration, run:

```bash
./scripts/ship.sh "chore: describe this iteration"
```

The script syncs Cloudflare secrets from `.env` to GitHub Actions, runs all quality gates, commits changes, pushes to `origin/main`, and watches the newest GitHub Actions run until it finishes.

## Cloudflare Pages Deployment

Local direct deployment reads Cloudflare configuration from `.env`; `.env` is ignored and must not be committed.

```bash
./scripts/deploy-cloudflare.sh
```

GitHub Actions requires these secrets:

- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ACCOUNT_ID`
- `CLOUDFLARE_PAGES_PROJECT`

Pushing to `main` runs CI, builds `dist/`, and deploys to Cloudflare Pages.
