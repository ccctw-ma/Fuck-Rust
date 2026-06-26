# ripgrep 源码 Rust 学习站 / Rust via ripgrep

English-only README: [README.en.md](./README.en.md)

一个完全用 Rust 实现的渐进式 Rust 学习网站。课程主线已经改为“通过阅读 ripgrep 源码学习 Rust”：每个知识模块都锚到 ripgrep 的真实源码片段，再对照 The Rust Programming Language 的规则做题。前端使用 Yew 编译到 WASM，学习路径、题目校验和进度统计放在纯 Rust `learning_core` crate 中，页面部署到 Cloudflare Pages。

A progressive Rust learning site implemented entirely in Rust. The curriculum now teaches Rust through ripgrep source code: every topic anchors to a real ripgrep snippet, then maps the relevant rule from The Rust Programming Language into exercises. The frontend is Yew compiled to WASM, while curriculum, checking, and progress logic live in the pure Rust `learning_core` crate. The site is deployed to Cloudflare Pages.

## 功能 / Features

- 按 ripgrep 源码组织 12 个学习章节：入口与退出码、模式分发、pattern 读取、进程资源、字节切片、writer 借用、解压 builder、Option/Result、配置集合、搜索 pipeline、globset 泛型与并行搜索。
- 300+ 道在线互动练习：单选、填空、代码输出判断、步骤排序，每个模块按基础、进阶、挑战递进，并尽量避免同一模块内重复题型。
- 即时反馈：提交后展示参考答案、解释和下一题入口。
- 源码阅读模块：每课展示 ripgrep 源码路径、行号、源码职责、Rust Book 对应规则、题目落点和源码链接。
- 本地进度：使用浏览器 `localStorage` 保存完成状态、答题统计、语言和主题偏好。
- UI：支持 light/dark 模式和中文/英文双语。
- CI/CD：GitHub Actions 执行格式、Clippy、测试、覆盖率门禁、Cloudflare Pages 部署和运行监听。

- Learning path follows 12 ripgrep source modules: entry and exit codes, mode dispatch, pattern loading, process resources, byte slices, writer borrowing, decompression builders, Option/Result boundaries, config collections, search pipelines, globset generics, and parallel search.
- 300+ interactive exercises: single choice, fill-in-the-blank, output prediction, and step ordering, arranged as basic, practice, and challenge questions for each module.
- Instant feedback with expected answers, explanations, and the next exercise.
- Source reading modules show the ripgrep file path, line range, source responsibility, mapped Rust Book rule, question focus, and source link.
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
