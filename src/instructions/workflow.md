# OMCC 工作流指南

## 角色分工

| 角色 | 定位 | 用途 | 沙箱模式 | CLI 工具 | 默认重试 |
|------|------|------|----------|----------|----------|
| **Reviewer** | 代码审核者 | 代码 Review、质量把关 | read-only | codex | 1 |
| **Advisor** | 高阶顾问 | 架构设计、第二意见、代码执行 | workspace-write | gemini | 1 |
| **Chore** | 杂务执行者 | 批量操作、格式化 | workspace-write | claude | 0 |
| **Researcher** | 研究专家 | 文档查询、网络搜索 | read-only | gemini | 1 |
| **Looker** | 多模态分析 | PDF/图片/图表分析 | read-only | gemini | 1 |

## 代理选择指南

```
用户需求
    │
    ├─ 代码审核 ─────────────────── Reviewer
    │
    ├─ 研究/咨询 ────────────────┬─ 架构设计/第二意见 → Advisor
    │                            └─ 网络研究 → Researcher
    │
    ├─ 文件分析 ─────────────────── PDF/图片/图表 → Looker
    │
    └─ 杂务任务 ─────────────────── 批量操作 → Chore
```

## 核心流程

### 1. 选择代理

根据任务类型选择合适的代理。

### 2. 获取使用指南

调用前先获取对应的 skill 文档：

```bash
omcc --reviewer-instructions
omcc --advisor-instructions
omcc --chore-instructions
omcc --researcher-instructions
omcc --looker-instructions
```

### 3. 执行与验收

```
代理执行 → 主代理验收 → 有误则修复 → 阶段性完成后 Reviewer 审核
```

### 4. Reviewer 审核

阶段性开发完成后调用 Reviewer：
- ✅ 通过：继续下一阶段
- ⚠️ 优化：委托修复后继续
- ❌ 修改：必须修复后重新审核

## 会话管理

**SESSION_ID 规范**：
- 必须保存返回的 `SESSION_ID`
- 后续请求中携带 `-S` 参数保持上下文
- 各角色的 SESSION_ID 相互独立
- 严禁自创 ID 或混用不同角色的 ID

## 使用示例

### 调用 Reviewer 审核代码

```bash
omcc reviewer -C /path/to/project "
请 review 以下代码改动：

**改动文件**：src/auth/login.ts
**改动目的**：实现用户登录功能

**请检查**：
1. 代码质量
2. 潜在 Bug
3. 需求完成度
"
```

### 调用 Advisor 获取建议

```bash
omcc advisor -C /path/to/project "
请评估以下架构方案：

**当前状态**：单体应用
**目标**：拆分为微服务

**请给出**：
1. 可行性分析
2. 风险评估
3. 实施建议
"
```

### 调用 Researcher 查询文档

```bash
omcc researcher -C /path/to/project "
请查询 React useEffect 的最佳实践，
特别是清理副作用的正确方式。
"
```

### 调用 Looker 分析文件

```bash
omcc looker /path/to/architecture.png --goal "
解释这个系统架构图的组件关系和数据流向
"
```

### 调用 Chore 执行杂务

```bash
omcc chore -C /path/to/project "
将所有 .js 文件重命名为 .ts
"
```

## CLI 参数速查

| 参数 | 简写 | 说明 |
|------|------|------|
| `--cd` | `-C` | 工作目录 |
| `--sandbox` | `-s` | 沙箱策略 |
| `--session-id` | `-S` | 会话 ID |
| `--timeout` | `-t` | 空闲超时（秒）|
| `--max-duration` | `-d` | 最大执行时长（秒）|
| `--max-retries` | `-r` | 最大重试次数 |
| `--model` | `-m` | 指定模型 |
| `--stdin` | `-i` | 从 stdin 读取提示词 |
| `--file` | `-f` | 从文件读取提示词 |
| `--json` | `-j` | JSON 格式输出 |

## 底层 CLI 工具

OMCC 依赖以下已安装的 CLI 工具：

| Agent | CLI 工具 | 安装检查 |
|-------|----------|----------|
| Reviewer | codex | `which codex` |
| Advisor | gemini | `which gemini` |
| Chore | claude | `which claude` |
| Researcher | gemini | `which gemini` |
| Looker | gemini | `which gemini` |

确保这些工具已正确安装并配置好认证。
