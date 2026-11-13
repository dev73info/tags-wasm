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
3. 当前分支落在 `develop` / `master` 上先停下：新建分支，不要在稳定线上直接改。
4. 基线落后 `origin/develop` 就先 `git switch develop && git pull --ff-only`。
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
- 门禁没跑绿前开 Draft；绿了、描述填全即可合并（日常 PR 不要求独立审阅，审阅在打 tag 前做）。
- `develop` / `master` 只能经 PR 合入：不直推、不 `--force`。合并看门禁，**审阅看发版**：
  日常 PR 门禁绿即可合；打 tag 前必须有一轮独立审阅并给出 `PASS`。
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

涉及多个子工程时，按根仓 `AGENTS.md` 第 7 节的跨仓规则手工按仓操作（`scripts/xrepo.sh` 已于 2026-10-22 删除）；本仓内仍按上述规则。

## 与根仓对齐（2026-11-13）

> 跨仓口径以根仓 `AGENTS.md`、`docs/开发流程/AI开发标准流程.md` 为准；本节是收尾时必须执行的部分。

1. **本地收尾**：改完、验完 → `bash ../scripts/finish.sh tags-wasm`（门禁 → 合入本地 `develop` → 删分支 → 打证据）。
   分支只在「正在改」时存在，**一个仓同时只能有 0 或 1 条分支**；本地合并后不 push。
2. **说「提」才推**：只有用户明确要 push / 开 PR，才从本地 `develop` 开同名分支推送；
   远程 PR 门禁绿即可自行 squash 合并（日常 PR 不需独立审阅），**不要停在等确认**。
3. **业务事实**：资金、合同、电签、渠道环境、合规判断先读 `../docs/业务/事实包.md` 对应节；
   没有依据就查 `../docs/规则/` 或问用户，**不得自行下「二清 / 违规」结论**。
4. **单文件 ≤300 行**：新增文件硬上限；存量只减不增（见 `.claude/rules/code-quality.md`）。
5. **完成定义**：分支归零 + 门禁绿 + 行为验证 + 证据 + 回写，五条全过才算完。
