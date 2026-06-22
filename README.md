# Rust 阶梯学习站 / Rust Ladder

English-only README: [README.en.md](./README.en.md)

一个完全用 Rust 实现的渐进式 Rust 学习网站。前端使用 Yew 编译到 WASM，学习路径、题目校验和进度统计放在纯 Rust `learning_core` crate 中，页面部署到 Cloudflare Pages。

A progressive Rust learning site implemented entirely in Rust. The frontend is Yew compiled to WASM, while curriculum, checking, and progress logic live in the pure Rust `learning_core` crate. The site is deployed to Cloudflare Pages.

## 功能 / Features

- 按 The Rust Programming Language 组织 12 个学习章节：变量、控制流、数据类型、所有权、切片、借用、结构体、集合、错误处理、迭代器、泛型和并发。
- 30 道在线互动练习：单选、填空、代码输出判断、步骤排序。
- 即时反馈：提交后展示参考答案、解释和下一题入口。
- 小 demo：每课提供 Rust Book 对应章节、代码片段、输出和关键理解点。
- 本地进度：使用浏览器 `localStorage` 保存完成状态、答题统计、语言和主题偏好。
- UI：支持 light/dark 模式和中文/英文双语。
- CI/CD：GitHub Actions 执行格式、Clippy、测试、覆盖率门禁、Cloudflare Pages 部署和运行监听。

- Learning path follows 12 chapters from The Rust Programming Language: variables, control flow, data types, ownership, slices, borrowing, structs, collections, error handling, iterators, generics, and concurrency.
- 30 interactive exercises: single choice, fill-in-the-blank, output prediction, and step ordering.
- Instant feedback with expected answers, explanations, and the next exercise.
- Mini demos linked to Rust Book chapters.
- Local progress stored in browser `localStorage`, including language and theme preferences.
- Light/dark mode and Chinese/English UI.
- CI/CD with format, Clippy, tests, coverage gate, Cloudflare Pages deployment, and workflow monitoring.

## 本地开发 / Local Development

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
cargo test --workspace
cd apps/web
trunk serve
```

## 质量门禁 / Quality Gates

业务逻辑覆盖率门禁落在 `learning_core`，目标大于 90%。

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

## Cloudflare Pages 部署 / Deployment

每次完成迭代后推荐使用：

After each iteration, prefer:

```bash
./scripts/ship.sh "chore: describe this iteration"
```

脚本会同步 `.env` 到 GitHub Actions secrets、运行质量门禁、提交、推送，并监听最新 GitHub Actions run。

The script syncs `.env` into GitHub Actions secrets, runs quality gates, commits, pushes, and watches the newest GitHub Actions run.

本地部署会从 `.env` 读取 Cloudflare 配置；`.env` 已被 `.gitignore` 排除，不应提交。

Local deployment reads Cloudflare configuration from `.env`; `.env` is ignored and must not be committed.

```bash
./scripts/deploy-cloudflare.sh
```

GitHub Actions 需要配置以下 secrets：

GitHub Actions requires these secrets:

- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ACCOUNT_ID`
- `CLOUDFLARE_PAGES_PROJECT`

推送到 `main` 后，CI 会构建 `dist/` 并部署到 Cloudflare Pages。

Pushing to `main` runs CI, builds `dist/`, and deploys to Cloudflare Pages.
