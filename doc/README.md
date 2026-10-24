# tags-wasm 文档

标签处理的 WASM 库（Rust → wasm-bindgen），产出 `pkg/` 供前端消费。

## 本目录覆盖什么

- 该 crate 对外暴露的函数与调用方式
- 构建流程：改动 Rust 源码后怎么重建 `pkg/`
- 两个前端各自的依赖方式（`link:` 与 `file:`）

## 本目录不覆盖什么

- 标签业务的数据库结构与权限 → [backend_73info/doc/数据库Schema.md](../../backend_73info/doc/数据库Schema.md)
- 前端怎么渲染标签树 → [user_client/doc/架构.md](../../user_client/doc/架构.md)、[admin_client/doc/架构.md](../../admin_client/doc/架构.md)
- 跨项目约定 → [AGENTS.md](../../AGENTS.md)

## 文档清单

| 文件 | 状态 | 覆盖 |
| --- | --- | --- |
| [构建与集成.md](构建与集成.md) | 已写 | crate 结构、导出函数、`wasm-pack` 构建、前端消费方式 |

## 为什么不单列 `架构.md`

这个 crate 只有一个源文件 `src/lib.rs`，分层、模块清单、数据流都没有可拆的内容。它的全部设计都写进了 [构建与集成.md](构建与集成.md)，再拆一份 `架构.md` 只会造成两处重复。
