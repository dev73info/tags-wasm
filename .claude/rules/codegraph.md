# CodeGraph 使用规则（硬规则）

本仓已建立 CodeGraph 语义索引（`.codegraph/`，MCP 提供 `codegraph_*` 工具）。
**每次修改代码前**（包括只改几行、只改文案）都必须先走一遍：

1. **定位**：用 `codegraph_context`（任务级入口）或 `codegraph_search` 找到目标符号与文件。
2. **看影响**：修改公共函数、组件、接口、类型前，先查 `codegraph_callers <符号>` 和
   `codegraph_impact <符号>`，确认调用方与受影响范围。
3. **读源码**：需要看实现时用 `codegraph_explore`（一次取多个相关符号），
   不要先 grep 再逐个 Read。
4. **改完**：文件监听会自动同步；批量新增/删除文件后索引明显落后时，运行 `codegraph sync`。

- 例外：纯文档、配置、生成物不强制；codegraph 查不到时再回退到普通搜索。
- 不要因为「改动很小」就跳过；定位与影响面检查是改代码的固定第一步。
- **worktree**：`.claude/worktrees/` 下没有 `.codegraph/` 和 `.mcp.json`。进 worktree 干活前先 `codegraph init -i`，
  并把仓库根的 `.mcp.json` 复制过去（把 `--path` 改成 worktree 路径），否则 codegraph 工具不可用。
