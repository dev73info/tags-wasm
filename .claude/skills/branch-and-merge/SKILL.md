---
name: branch-and-merge
description: tags-wasm 从 dev 开分支、提交、跑 wasm 门禁、开 PR（base=dev）、请求独立审阅、线性合并的一条龙。开始任何改动前、或不确定能否推稳定线 / 自己合并时使用。
---

# 开分支 → 门禁 → PR → 独立审阅 → 合并（tags-wasm）

集成分支 `dev`，稳定线 `master`。改动一律走分支 + PR。

## 一、开工前 30 秒

```bash
pwd
git fetch origin
git status --porcelain && git branch --show-current
```

- 工作区有别人的未提交改动：先停下，别动它。
- 当前在 `dev` / `master`：先切分支。

## 二、切分支

```bash
git switch dev && git pull --ff-only
git switch -c feat/<需求号>-<简称>
```

命名 `feat/<需求号>-<简称>` / `fix/…` / `chore/…` / `docs/…`；`hotfix/` 从 `master` 切。

## 三、提交

- 只 `git add <具体文件>`，不要 `git add -A` / `git add .`。
- Conventional Commits 前缀 + 中文简述，一次提交只做一件事。
- 提交信息里不写任何工具或 agent 署名。

## 四、门禁

用 `gate` 技能：`cargo check --target wasm32-unknown-unknown` + `wasm-pack build`，
改了公开行为还要在 `../user_client` / `../admin_client` 做宿主联动验证。

## 五、PR

- base 写 `dev`；`hotfix/` 写 `master`。
- 标题 `[需求号] 类型(范围): 中文简述`；正文按 `../docs/开发流程/PR模板.md` 五段填，
  写到仓库外的临时文件再 `--body-file`。
- 门禁没绿前开 Draft；绿了、描述填全再请求审阅。

## 六、审阅与合并

- `dev` / `master` 只能经 PR 合入：不直推、不 `--force`。
- 作者不自审自合，无审阅结论不合并；**AI 不给自己写 PASS**。
- `master` 有保护：至少 1 个批准、线性历史、禁强推、禁删分支。
- 推稳定线、改别人的分支、改 PR 的目标分支之前**先问用户**。
