# Rust Ladder

A progressive Rust learning site implemented entirely in Rust. The frontend is built with Yew and compiled to WASM, while curriculum, answer checking, and progress logic live in the pure Rust `learning_core` crate. The site deploys to Cloudflare Pages.

## Features

- Learning path follows 12 chapters from The Rust Programming Language: variables, control flow, data types, ownership, slices, borrowing, structs, collections, error handling, iterators, generics, and concurrency.
- 30 interactive exercises: single choice, fill-in-the-blank, output prediction, and step ordering.
- Instant feedback with expected answers, explanations, and the next exercise.
- Mini demos linked to Rust Book chapters.
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
