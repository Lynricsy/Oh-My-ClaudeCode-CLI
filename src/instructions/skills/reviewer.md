# Reviewer 代码审核者

## 角色定位

**Reviewer**（原 Codex）是代码审核者：
- 检查代码质量（可读性、可维护性、潜在 Bug）
- 评估需求完成度
- 给出明确结论：✅ 通过 / ⚠️ 建议优化 / ❌ 需要修改

**注意**：Reviewer 仅审核，**严禁修改代码**，默认 sandbox 为 read-only

## 触发场景

- 阶段性开发完成后的代码审核
- 需要独立第三方视角的质量把关
- 代码合入前的最终检查
- 复杂方案的架构评审

## CLI 调用方式

```bash
omcc reviewer -C <工作目录> "<审核任务描述>"

# 示例
omcc reviewer -C /path/to/project "请 review src/auth/ 目录的改动"

# 带图片审核
omcc reviewer -C /path/to/project --image screenshot.png "请 review 这个 UI 实现"

# 从文件读取任务
omcc reviewer -C /path/to/project --file review_task.md
```

## 参数说明

| 参数 | 默认值 | 说明 |
|------|--------|------|
| `-C, --cd` | `.` | 工作目录 |
| `-s, --sandbox` | `read-only` | 沙箱策略（**必须**保持 read-only） |
| `-S, --session-id` | - | 会话 ID，用于多轮对话 |
| `-I, --image` | - | 附加图片文件（可多次使用）|
| `--skip-git-check` | false | 跳过 Git 仓库检查 |
| `--yolo` | false | 跳过审批（不推荐）|
| `--profile` | - | 配置文件名称 |
| `-t, --timeout` | 300 | 空闲超时（秒）|
| `-d, --max-duration` | 1800 | 最大执行时长（秒）|
| `-r, --max-retries` | 1 | 最大重试次数（只读可安全重试）|

## Prompt 模板

```markdown
请 review 以下代码改动：

**改动文件**：[文件列表]
**改动目的**：[简要描述]

**请检查**：
1. 代码质量（可读性、可维护性）
2. 潜在 Bug 或边界情况
3. 需求完成度

**请给出明确结论**：
- ✅ 通过：代码质量良好，可以合入
- ⚠️ 建议优化：[具体建议]
- ❌ 需要修改：[具体问题]
```

## 返回值格式

```json
{
  "status": "success",
  "agent": "reviewer",
  "SESSION_ID": "uuid-string",
  "result": "审核结论...",
  "duration": "0m45s"
}
```

## 会话复用

保存返回的 `SESSION_ID`，在后续请求中使用 `-S` 参数保持上下文：

```bash
# 首次审核
omcc reviewer -C /project "请 review src/auth/"
# 返回 SESSION_ID: abc-123

# 继续对话
omcc reviewer -C /project -S abc-123 "请详细说明第 2 点的安全问题"
```

## 独立决策

Reviewer 的意见仅供参考。你（Claude）是最终决策者，需批判性思考，做出最优决策。
