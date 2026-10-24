---
name: gate
description: 在 tags-wasm 跑 WASM 质量门禁（wasm32 编译 + wasm-pack 产物 + 宿主联动）。改到 src/** 之后、提交或开 PR 之前使用；不确定产物是否要重新构建时也用它。
---

# 跑门禁（tags-wasm）

## 一、前置

```bash
rustup target add wasm32-unknown-unknown   # 新机器第一次构建前
```

## 二、两步必跑

| 顺序 | 命令 | 说明 |
| --- | --- | --- |
| 1 | `cargo check --target wasm32-unknown-unknown` | 先保证能编译 |
| 2 | `wasm-pack build` | **必跑**：消费方吃的是 `pkg/`，只改源码不 build 等于没改 |

## 三、宿主联动（改了公开 API / 数据结构 / 行为时必须）

- 在 `../user_client` 跑 `pnpm type-check`，必要时 `pnpm gate:quick`
- 在 `../admin_client` 跑各自校验
- 两个宿主都消费 `pkg/`，要给出实际验证结论，不要只写「本地编译通过」

## 四、失败处置

1. **wasm32 编译失败**：先确认没有用到平台相关 API，再看依赖是否需要 `wasm-bindgen`。
2. **wasm-pack 失败**：看 `pkg/` 是否生成；不要去别的机器拷产物。
3. **宿主报错**：说明产物与宿主契约不一致，回本仓修，不要改宿主绕开。

## 五、红线

- `pkg/` 不入库、不跨机器拷贝，缺了就在本仓重新 build。
- 标签树的语义 / 数据结构改动属于**契约变更**：先在任务文件里冻结接口，再让两个宿主跟进。
- 公开仓：`master` 有服务端保护（要求 PR + 至少 1 个批准、线性历史、禁强推）。
