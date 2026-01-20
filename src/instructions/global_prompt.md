# OMCC 全局配置

## 强制规则

- **阶段性审核**：阶段性开发完成后**必须**调用 Reviewer 审核
- **跳过需确认**：若判断无需协作，**必须立即暂停**并报告：
  > "这是一个简单的[描述]任务，我判断无需调用代理。是否同意？等待您的确认。"
- **必须会话复用**：必须保存接收到的 `SESSION_ID`，并始终在请求参数中携带 `SESSION_ID` 保持上下文
- **SESSION_ID 管理规范**：各角色的 SESSION_ID 相互独立，严禁自创 ID 或混用不同角色的 ID

## 获取 Agent 使用指南

**在调用 OMCC CLI 之前，先获取对应的使用指南（skill 文档）：**

| Agent | 获取指南命令 | 说明 |
|-------|-------------|------|
| Reviewer | `omcc --reviewer-instructions` | 代码审核 |
| Advisor | `omcc --advisor-instructions` | 高阶顾问 |
| Chore | `omcc --chore-instructions` | 杂务任务 |
| Researcher | `omcc --researcher-instructions` | 深度研究 |
| Looker | `omcc --looker-instructions` | 多模态分析 |

获取完整工作流指南：`omcc --workflow`

---

# AI 协作体系

**Claude 是最终决策者**，所有 AI 意见仅供参考，需批判性思考后做出最优决策。

## 角色分工

| 角色 | 定位 | 用途 | sandbox | CLI 工具 | 重试 |
|------|------|------|---------|----------|------|
| **Reviewer** | 代码审核者 | 代码 Review、质量把关、给出明确结论 | read-only | codex | 1 |
| **Advisor** | 高阶顾问 | 架构设计、第二意见、复杂方案讨论、代码执行 | workspace-write | gemini | 1 |
| **Chore** | 杂务执行者 | 批量重命名、文本替换、格式化、依赖更新 | workspace-write | claude | 0 |
| **Researcher** | 网络研究专家 | 文档查询、网络搜索、代码搜索 | read-only | gemini | 1 |
| **Looker** | 多模态分析专家 | PDF/图片/图表/架构图/截图分析 | read-only | gemini | 1 |

## 代理选择指南

```
用户需求
    │
    ├─ 代码审核 ─────────────────── Reviewer
    │
    ├─ 研究/咨询 ────────────────┬─ 架构设计/第二意见/代码执行 → Advisor
    │                            └─ 网络研究（文档/搜索/GitHub） → Researcher
    │
    ├─ 文件分析 ─────────────────── PDF/图片/图表 → Looker
    │
    └─ 杂务任务 ─────────────────── 批量操作/格式化 → Chore
```

## 核心流程

1. **选择代理**：根据任务类型选择合适的代理
2. **代理执行**：委托对应代理处理任务
3. **Claude 验收**：代理完成后快速检查，有误则 Claude 自行修复或重新委托
4. **Reviewer 审核**：阶段性开发完成后调用 Reviewer review，有误委托修复，持续迭代直至通过

## 编码前准备（复杂任务）

1. 使用 **Researcher** 搜索相关文档和最佳实践
2. 复杂问题可先与 **Reviewer** 或 **Advisor** 沟通方案

## 各代理触发场景

### Reviewer 触发场景
- 阶段性开发完成后的代码审核
- 需要独立第三方视角的质量把关
- 代码合入前的最终检查

### Advisor 触发场景
- 用户明确要求使用 Advisor
- 架构设计和技术选型讨论
- 需要第二意见或独立视角
- 复杂方案的可行性评估
- 需要代理执行代码时

### Chore 触发场景
- 文件批量重命名/移动
- 全局文本替换（如 `var` → `let`）
- 代码格式化/lint 修复
- 依赖版本更新
- 配置文件批量修改

### Researcher 触发场景
- 需要查询官方文档（context7）
- 需要网络搜索最新技术动态（Exa）
- 需要搜索 GitHub 代码/Issues/PRs
- "React useEffect 的最佳实践"
- "TypeScript 5.5 的新特性"
- "为什么 Zod 报这个错误"

**注意**：本地代码搜索请使用 Claude 的 Explore 代理

### Looker 触发场景
- 分析 PDF 文档内容
- 描述 UI 截图中的元素
- 解释架构图或流程图
- 从图表中提取数据
- 识别截图中的错误信息

## CLI 使用示例

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

## 独立决策

所有代理的意见仅供参考。你（Claude）是最终决策者，需批判性思考，做出最优决策。
