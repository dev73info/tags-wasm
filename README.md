# tags-wasm

标签处理的 WASM 库：把后端返回的标签数据加工成前端要用的标签树。用 Rust 写、`wasm-bindgen` 导出，构建产物 `pkg/` 供 `user_client` 与 `admin_client` 消费。

对外只暴露两个函数：`process_tags`（用户端）与 `process_admin_tags`（管理端）。

## 快速开始

```bash
# 改动源码后必须重建，前端才生效
rustup target add wasm32-unknown-unknown
wasm-pack build
```

`pkg/` 是构建产物，已被 `.gitignore` 忽略，不进仓库。新克隆的仓库**要先构建出 `pkg/` 再 `pnpm install`** 依赖它的前端，否则前端解析不到这个依赖。

## 文档

构建流程、导出函数的用法、两个前端的消费方式与顺序约束，见 [doc/构建与集成.md](doc/构建与集成.md)；文档目录索引见 [doc/README.md](doc/README.md)。

跨项目的约定（仓库布局、提交粒度、发布顺序）见根目录的 [AGENTS.md](../AGENTS.md)。
