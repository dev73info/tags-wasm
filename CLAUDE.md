# CLAUDE.md（子工程执行版）——tags-wasm

本仓是 73Info 工作区（Windows `E:\dev73info` / WSL `/home/banch/dev73info`）11 个子工程仓库之一，
根目录本身是只放文档与配置的第 12 个仓库。跨工程约定以根仓 `../CLAUDE.md`、`../AGENTS.md`
与 `../docs/开发流程/AI开发标准流程.md` 为准，**冲突以标准文档为准**；本文件只写本仓的差异项。

## 一、开工（在本仓动手之前，包括「只是看看 / 改个文案」）

> **CodeGraph 硬规则**：每次修改代码前先用 codegraph 定位符号与影响面
> （`codegraph_context` / `codegraph_search` → `codegraph_callers` / `codegraph_impact`），
> 详见 `.claude/rules/codegraph.md`。不要跳过索引直接 grep。

1. `pwd` 确认在本仓目录；一次只处理一个仓库，不跨仓批量提交。
2. `git fetch origin` → `git status --porcelain` → `git branch --show-current`。
3. 当前分支落在 `dev` / `master` 上先停下：新建分支，不要在稳定线上直接改。
4. 基线落后 `origin/dev` 就先 `git switch dev && git pull --ff-only`。
   **不要从过期的 `dev` 开工**：已 squash 合入的改动会以另一个 SHA 再带一遍，PR 会平白多出别人的文件。
5. 分支从最新 `dev` 切（`hotfix/` 从 `master` 切），命名 `feat/<需求号>-<简称>` / `fix/…` /
   `chore/…` / `docs/…`；跨仓库需求在各仓用**同名**分支，一条分支只做一件事。

## 二、提交

- `git add <具体文件>`，禁止 `git add -A` / `git add .`；提交前 `git diff --cached --check`。
- Conventional Commits 前缀 + 中文简述，一个提交一件事。
- 提交信息里**不写任何工具或 agent 署名**（无 `Co-Authored-By` / `Signed-off-by` / `Generated with`）。
- 绝不提交：`pkg/`（wasm-pack 产物）、`target/`。`Cargo.lock` 入库，别删。

## 三、PR

- base 写 `dev`；`hotfix/` 写 `master`。
- 标题 `[需求号] 类型(范围): 中文简述`；正文按 `../docs/开发流程/PR模板.md` 五段填，
  写到仓库外的临时文件再 `--body-file` 传。
- 门禁没跑绿前开 Draft；绿了、描述填全再请求审阅。
- `dev` / `master` 只能经 PR 合入：不直推、不 `--force`，作者不自审自合，无审阅结论不合并。
- 推稳定线、改别人的分支、改 PR 的目标分支之前**先问用户**。

## 四、本仓门禁

| 场景 | 命令 | 说明 |
| --- | --- | --- |
| 前置 | `rustup target add wasm32-unknown-unknown` | 新机器第一次构建前装 target（见 `doc/构建与集成.md`） |
| 改到 `src/**` | `cargo check --target wasm32-unknown-unknown` | 先编译过再打包 |
| 出产物 | `wasm-pack build` | **必跑**：消费方吃的是 `pkg/`，只改源码不 build 等于没改 |
| 联动验证 | 在 `../user_client` / `../admin_client` 跑各自校验 | 产物被两个宿主消费，改了要连宿主验一遍 |

## 五、本仓事实与红线

- 定位：Rust → WASM 的标签树处理库。消费方是 `../user_client` 与 `../admin_client`，
  它们读的是 **`pkg/` 里的产物**，所以「改源码 + 提交」不够，必须 `wasm-pack build` 后让宿主重新验证。
- `pkg/` 不入库（`.gitignore` 已排除）：宿主侧缺失时由本仓重新 build 生成，不要去别的机器上拷。
- **公开仓，`master` 有服务端保护**：要求 PR 且至少 1 个批准、`enforce_admins`、线性历史（squash / rebase）、
  禁强推、禁删除；`dev` 只禁强推与删除。你不能批准自己的 PR。
- 目前只有 `src/lib.rs`，没有集成测试；改动要有宿主侧走查证据，不要只写「本地编译通过」。
- 标签树的语义与数据结构改动属于**契约变更**：先在任务文件里冻结接口，再让两个宿主跟进。

## 跨仓需求

涉及多个子工程时，用根仓 `scripts/xrepo.sh` 编排（清单在根仓 `docs/任务/<需求号>-*/xrepo/`，用法见根仓 `skills/xrepo/SKILL.md`）；本仓内仍按上述规则。
