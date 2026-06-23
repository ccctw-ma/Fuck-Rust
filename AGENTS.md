# 项目约束

## 范围
- 本项目是 Rust 学习网站，核心目标是渐进式帮助用户学习 Rust。
- 学习内容、示例和章节顺序优先参考 The Rust Programming Language：`https://doc.rust-lang.org/book/`。
- 首版不执行用户输入的任意 Rust 代码，在线练习采用预置题目、规则校验和解释反馈。

## 技术栈
- 前端必须使用 Rust 实现，当前选择 Yew + WASM + Trunk。
- 学习路径、判题、进度统计等业务逻辑必须放在纯 Rust `learning_core` crate，便于单元测试。
- 不引入 React/Vue/Next.js 或其他 JS 前端框架。
- 部署目标是 Cloudflare Pages，构建产物为静态 `dist/`。

## UI 与产品
- 必须支持 light/dark 模式。
- 必须支持中文和英文双语。
- README 面向用户，也必须保持中英双语。
- UI 保持沉浸、简洁、少边框、暗色玻璃感为默认方向；亮色模式需要完整可读。

## 测试与质量
- 保持 `learning_core` 行覆盖率大于 90%。
- 硬约束：每次代码或产品迭代完成后，必须自动完成质量门禁、提交、推送和部署触发，不要停留在“本地已改完”等待用户再次确认。
- 默认同步方式：优先运行 `./scripts/ship.sh "chore: describe this iteration"`，完成质量门禁、提交、推送、CI/CD 和 workflow 监听；提交信息必须准确描述本次迭代。
- 如果 `ship.sh` 中的非核心辅助步骤因本地环境问题失败（例如当前环境的 `gh` 不是 GitHub CLI，导致 workflow 监听失败），但质量门禁、提交和推送已成功，必须向用户明确说明已完成同步以及跳过/失败的辅助步骤。
- 常规验证命令：
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo llvm-cov -p learning_core --fail-under-lines 90 --summary-only`
  - `cd apps/web && trunk build --release`

## 配置与密钥
- `.env` 只用于本地部署操作，必须保持在 `.gitignore` 中。
- 不要把 Cloudflare token、account id 或其他密钥写入 README、代码、CI 文件或日志。
- GitHub Actions 部署依赖仓库 secrets：`CLOUDFLARE_API_TOKEN`、`CLOUDFLARE_ACCOUNT_ID`、`CLOUDFLARE_PAGES_PROJECT`。
