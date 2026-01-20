# OMCC - Oh-My-ClaudeCode CLI

> AI 多代理协作命令行工具 🤖

OMCC 是一个将 AI 多代理协作系统从 MCP 迁移到 CLI 的工具，提供结构化的输入输出，便于与各种 AI 客户端集成。

## ✨ 特性

- **多代理协作**：5 个专业 Agent 协同工作
  - **Reviewer** - 代码审核者（底层调用 codex CLI）
  - **Advisor** - 高阶顾问（底层调用 opencode CLI）
  - **Chore** - 杂务执行者（底层调用 opencode CLI）
  - **Researcher** - 网络研究专家（底层调用 opencode CLI）
  - **Looker** - 多模态分析专家（底层调用 opencode CLI）

- **结构化输入输出**：JSON 格式，便于程序集成
- **会话管理**：支持多轮对话，保持上下文
- **灵活配置**：沙箱策略、超时控制、重试机制
- **内置 Skill 文档**：通过 `--xxx-instructions` 获取使用指南，供主 AI 学习如何使用

## 📦 安装

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/Lynricsy/Oh-My-ClaudeCode-CLI.git
cd Oh-My-ClaudeCode-CLI

# 编译 release 版本
cargo build --release

# 安装到系统路径（可选）
cargo install --path .
```

### 依赖

OMCC 需要以下 CLI 工具（根据使用的 Agent）：

| Agent | 底层 CLI | 说明 |
|-------|----------|------|
| Reviewer | [codex](https://github.com/openai/codex) | OpenAI Codex CLI |
| Advisor | [opencode](https://opencode.ai) | OpenCode CLI |
| Chore | [opencode](https://opencode.ai) | OpenCode CLI |
| Researcher | [opencode](https://opencode.ai) | OpenCode CLI |
| Looker | [opencode](https://opencode.ai) | OpenCode CLI |

## 🚀 快速开始

### 基本用法

```bash
# 查看帮助
omcc --help

# 列出所有可用的 Agent
omcc list

# 获取 Agent 使用指南（skill 文档）
omcc --reviewer-instructions
omcc --advisor-instructions
omcc --workflow

# 获取全局提示词（用于 AI 客户端配置）
omcc --global-prompt
```

### 调用 Agent

```bash
# 调用 Reviewer 审核代码
omcc reviewer -C /path/to/project "请 review src/auth/ 目录的改动"

# 调用 Advisor 获取架构建议
omcc advisor -C /path/to/project "评估微服务拆分方案"

# 调用 Researcher 查询文档
omcc researcher -C /path/to/project "React useEffect 最佳实践"

# 调用 Looker 分析图片
omcc looker /path/to/screenshot.png --goal "描述 UI 布局"

# 调用 Chore 执行杂务
omcc chore -C /path/to/project "格式化 src 目录下所有文件"

# 从 stdin 读取提示词
echo "任务描述..." | omcc reviewer -C /path/to/project --stdin

# JSON 格式输出
omcc reviewer -C /path/to/project --json "审核任务..."

# 会话复用
omcc reviewer -C /path/to/project -S "previous-session-id" "继续审核..."
```

## 📖 Agent 说明

| Agent | 角色 | 用途 | 沙箱模式 | 底层 CLI | 默认重试 |
|-------|------|------|----------|----------|----------|
| **reviewer** | 代码审核者 | 代码 Review、质量把关 | read-only | codex | 1 |
| **advisor** | 高阶顾问 | 架构设计、第二意见、代码执行 | workspace-write | opencode | 1 |
| **chore** | 杂务执行者 | 批量操作、格式化 | workspace-write | opencode | 0 |
| **researcher** | 研究专家 | 文档查询、网络搜索 | read-only | opencode | 1 |
| **looker** | 多模态分析 | PDF/图片/图表分析 | read-only | opencode | 1 |

## ⚙️ 参数说明

### 通用参数

| 参数 | 简写 | 说明 |
|------|------|------|
| `--cd` | `-C` | 工作目录 |
| `--sandbox` | `-s` | 沙箱策略：read-only / workspace-write / danger-full-access |
| `--session-id` | `-S` | 会话 ID（用于多轮对话）|
| `--timeout` | `-t` | 空闲超时（秒）|
| `--max-duration` | `-d` | 最大执行时长（秒）|
| `--max-retries` | `-r` | 最大重试次数 |
| `--model` | `-m` | 指定模型 |
| `--stdin` | `-i` | 从 stdin 读取提示词 |
| `--file` | `-f` | 从文件读取提示词 |
| `--json` | `-j` | JSON 格式输出 |

### Skill 文档输出参数

| 参数 | 说明 |
|------|------|
| `--reviewer-instructions` | 输出 Reviewer 使用指南 |
| `--advisor-instructions` | 输出 Advisor 使用指南 |
| `--chore-instructions` | 输出 Chore 使用指南 |
| `--researcher-instructions` | 输出 Researcher 使用指南 |
| `--looker-instructions` | 输出 Looker 使用指南 |
| `--workflow` | 输出完整工作流指南 |
| `--global-prompt` | 输出全局提示词（用于 AI 客户端配置）|

## 📤 输出格式

### 成功响应

```json
{
  "status": "success",
  "agent": "reviewer",
  "SESSION_ID": "uuid-string",
  "result": "执行结果内容",
  "duration": "0m45s"
}
```

### 失败响应

```json
{
  "status": "failure",
  "agent": "reviewer",
  "error": "错误摘要",
  "error_kind": "idle_timeout",
  "error_detail": {
    "message": "错误详情",
    "exit_code": 1,
    "last_lines": ["最后几行输出..."],
    "idle_timeout_s": 300,
    "retries": 1
  }
}
```

## 🔧 与 AI 客户端集成

OMCC 设计为易于与各种 AI 客户端集成。

### 获取全局提示词

```bash
omcc --global-prompt > system_prompt.md
```

将此提示词添加到你的 AI 客户端的系统提示中，AI 将了解如何使用 OMCC CLI 进行多代理协作。

### 获取 Skill 文档

```bash
# 获取特定 Agent 的 skill 文档
omcc --reviewer-instructions > reviewer_skill.md
```

将 skill 文档提供给主 AI，让它学习如何正确调用对应的 Agent。

## 🔗 与原有 MCP 环境兼容

OMCC CLI 与原有的 Oh-My-ClaudeCode MCP 使用相同的底层 CLI 工具：

- 如果你已经配置好了 `codex`、`gemini`、`claude` CLI，OMCC 可以直接使用
- 无需重新配置认证或 API Key
- 会话管理和沙箱策略与 MCP 版本一致

## 📜 License

MIT License

## 🙏 致谢

- 原项目 [Oh-My-ClaudeCode](https://github.com/Lynricsy/Oh-My-ClaudeCode)
- [OpenCode CLI](https://opencode.ai)
- [Codex CLI](https://github.com/openai/codex)
