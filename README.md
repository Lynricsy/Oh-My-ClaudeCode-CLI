# OMCC - Oh-My-ClaudeCode CLI

> AI 多代理协作命令行工具 🤖

OMCC 是一个将 AI 多代理协作系统从 MCP 迁移到 CLI 的工具，提供结构化的输入输出，便于与各种 AI 客户端集成。

## ✨ 特性

- **多代理协作**：7 个专业 Agent 协同工作
  - **Coder** - 代码执行者
  - **Reviewer** - 代码审核者（原 Codex）
  - **Advisor** - 高阶顾问（原 Gemini）
  - **Frontend** - 前端/UI 专家
  - **Chore** - 杂务执行者
  - **Researcher** - 网络研究专家（原 Librarian）
  - **Looker** - 多模态分析专家

- **结构化输入输出**：JSON 格式，便于程序集成
- **会话管理**：支持多轮对话，保持上下文
- **灵活配置**：沙箱策略、超时控制、重试机制
- **内置指南**：通过 `--xxx-instructions` 获取使用指南

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

OMCC 需要以下 CLI 工具之一（根据使用的 Agent）：

- [claude](https://github.com/anthropics/claude-code) - Coder、Chore
- [codex](https://github.com/openai/codex) - Reviewer
- [gemini](https://github.com/google-gemini/gemini-cli) - Advisor、Frontend、Researcher、Looker

## 🚀 快速开始

### 基本用法

```bash
# 查看帮助
omcc --help

# 列出所有可用的 Agent
omcc list

# 获取 Agent 使用指南
omcc --coder-instructions
omcc --reviewer-instructions
omcc --workflow

# 获取全局提示词（用于 AI 客户端配置）
omcc --global-prompt
```

### 调用 Agent

```bash
# 调用 Coder 执行代码任务
omcc coder -C /path/to/project "实现用户登录功能"

# 调用 Reviewer 审核代码
omcc reviewer -C /path/to/project "请 review src/auth/ 目录的改动"

# 调用 Researcher 查询文档
omcc researcher -C /path/to/project "React useEffect 最佳实践"

# 从 stdin 读取提示词
echo "任务描述..." | omcc coder -C /path/to/project --stdin

# 从文件读取提示词
omcc coder -C /path/to/project --file task.md

# JSON 格式输出
omcc coder -C /path/to/project --json "任务描述..."

# 会话复用
omcc coder -C /path/to/project -S "previous-session-id" "继续上次的任务..."
```

### 调用 Looker 分析文件

```bash
# 分析 PDF 文档
omcc looker /path/to/document.pdf --goal "提取文档中关于用户认证的内容"

# 分析图片
omcc looker /path/to/screenshot.png --goal "描述 UI 界面的布局"
```

## 📖 Agent 说明

| Agent | 角色 | 用途 | 沙箱模式 | 默认重试 |
|-------|------|------|----------|----------|
| **coder** | 代码执行者 | 生成/修改代码、批量任务 | workspace-write | 0 |
| **reviewer** | 代码审核者 | 代码 Review、质量把关 | read-only | 1 |
| **advisor** | 高阶顾问 | 架构设计、第二意见 | workspace-write | 1 |
| **frontend** | 前端/UI 专家 | 界面设计、样式动效 | workspace-write | 1 |
| **chore** | 杂务执行者 | 批量操作、格式化 | workspace-write | 0 |
| **researcher** | 研究专家 | 文档查询、网络搜索 | read-only | 1 |
| **looker** | 多模态分析 | PDF/图片/图表分析 | read-only | 1 |

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

### 指南输出参数

| 参数 | 说明 |
|------|------|
| `--coder-instructions` | 输出 Coder 使用指南 |
| `--reviewer-instructions` | 输出 Reviewer 使用指南 |
| `--advisor-instructions` | 输出 Advisor 使用指南 |
| `--frontend-instructions` | 输出 Frontend 使用指南 |
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
  "agent": "coder",
  "SESSION_ID": "uuid-string",
  "result": "执行结果内容",
  "duration": "1m30s"
}
```

### 失败响应

```json
{
  "status": "failure",
  "agent": "coder",
  "error": "错误摘要",
  "error_kind": "idle_timeout",
  "error_detail": {
    "message": "错误详情",
    "exit_code": 1,
    "last_lines": ["最后几行输出..."],
    "idle_timeout_s": 300,
    "retries": 0
  }
}
```

## 🔧 与 AI 客户端集成

OMCC 设计为易于与各种 AI 客户端集成。获取全局提示词：

```bash
omcc --global-prompt > system_prompt.md
```

将此提示词添加到你的 AI 客户端的系统提示中，AI 将了解如何使用 OMCC CLI 进行多代理协作。

## 📜 License

MIT License

## 🙏 致谢

- 原项目 [Oh-My-ClaudeCode](https://github.com/Lynricsy/Oh-My-ClaudeCode)
- [Claude Code](https://github.com/anthropics/claude-code)
- [Gemini CLI](https://github.com/google-gemini/gemini-cli)
