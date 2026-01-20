//! 指导内容模块
//!
//! 包含所有 Agent 的使用指南和系统提示词

use crate::types::{AgentType, InstructionOutput, ParameterInfo};

/// 获取 Agent 的使用指南
pub fn get_instructions(agent_type: AgentType) -> InstructionOutput {
    match agent_type {
        AgentType::Coder => get_coder_instructions(),
        AgentType::Reviewer => get_reviewer_instructions(),
        AgentType::Advisor => get_advisor_instructions(),
        AgentType::Frontend => get_frontend_instructions(),
        AgentType::Chore => get_chore_instructions(),
        AgentType::Researcher => get_researcher_instructions(),
        AgentType::Looker => get_looker_instructions(),
    }
}

/// 获取工作流指南
pub fn get_workflow_instructions() -> String {
    include_str!("workflow.md").to_string()
}

/// 获取全局提示词
pub fn get_global_prompt() -> String {
    include_str!("global_prompt.md").to_string()
}

fn get_coder_instructions() -> InstructionOutput {
    InstructionOutput {
        agent: "coder".to_string(),
        display_name: "代码执行者".to_string(),
        role: "根据精确的 Prompt 生成或修改代码，执行批量代码任务".to_string(),
        use_cases: vec![
            "新增功能：根据需求生成代码".to_string(),
            "修复 Bug：根据问题描述修改代码".to_string(),
            "重构：根据目标进行代码重构".to_string(),
            "批量任务：执行大量相似的代码修改".to_string(),
        ],
        parameters: get_common_parameters(),
        prompt_template: r#"**任务目标**：[一句话说明]

**目标文件**：[文件路径]

**具体要求**：
1. [要求1]
2. [要求2]

**交付标准**：
- [ ] [可验证的条件]

完成后简要说明改动内容。"#
            .to_string(),
        return_format: r#"{
  "status": "success",
  "agent": "coder",
  "SESSION_ID": "uuid-string",
  "result": "执行结果",
  "duration": "1m30s"
}"#
        .to_string(),
        notes: vec![
            "Coder 需要写权限，默认 sandbox 为 workspace-write".to_string(),
            "任务失败的主要原因是规格不足，而非模型能力不足".to_string(),
            "提供完整的上下文、明确的步骤、清晰的交付标准".to_string(),
        ],
    }
}

fn get_reviewer_instructions() -> InstructionOutput {
    InstructionOutput {
        agent: "reviewer".to_string(),
        display_name: "代码审核者".to_string(),
        role: "检查代码质量、评估需求完成度、给出明确结论".to_string(),
        use_cases: vec![
            "阶段性开发完成后的代码审核".to_string(),
            "需要独立第三方视角时".to_string(),
            "代码合入前的最终检查".to_string(),
        ],
        parameters: get_common_parameters(),
        prompt_template: r#"请 review 以下代码改动：

**改动文件**：[文件列表]
**改动目的**：[简要描述]

**请检查**：
1. 代码质量（可读性、可维护性）
2. 潜在 Bug 或边界情况
3. 需求完成度

**请给出明确结论**：
- ✅ 通过：代码质量良好，可以合入
- ⚠️ 建议优化：[具体建议]
- ❌ 需要修改：[具体问题]"#
            .to_string(),
        return_format: r#"{
  "status": "success",
  "agent": "reviewer",
  "SESSION_ID": "uuid-string",
  "result": "审核结论",
  "duration": "0m45s"
}"#
        .to_string(),
        notes: vec![
            "Reviewer 仅审核，严禁修改代码".to_string(),
            "默认 sandbox 为 read-only".to_string(),
            "默认允许 1 次自动重试".to_string(),
        ],
    }
}

fn get_advisor_instructions() -> InstructionOutput {
    InstructionOutput {
        agent: "advisor".to_string(),
        display_name: "高阶顾问".to_string(),
        role: "架构设计、技术选型、复杂方案讨论、独立审核".to_string(),
        use_cases: vec![
            "用户明确要求使用 Advisor".to_string(),
            "需要第二意见或独立视角".to_string(),
            "架构设计和技术讨论".to_string(),
            "原型开发、功能实现".to_string(),
        ],
        parameters: get_common_parameters(),
        prompt_template: r#"请提供专业意见/执行以下任务：

**任务类型**：[咨询 / 审核 / 执行]
**背景信息**：[项目上下文]

**具体问题/任务**：
1. [问题/任务1]
2. [问题/任务2]

**期望输出**：
- [输出格式/内容要求]"#
            .to_string(),
        return_format: r#"{
  "status": "success",
  "agent": "advisor",
  "SESSION_ID": "uuid-string",
  "result": "顾问回复",
  "duration": "2m15s"
}"#
        .to_string(),
        notes: vec![
            "Advisor 权限灵活，默认 yolo=true".to_string(),
            "与主 Agent 同等级别的顶级 AI 专家".to_string(),
            "默认允许 1 次重试".to_string(),
        ],
    }
}

fn get_frontend_instructions() -> InstructionOutput {
    InstructionOutput {
        agent: "frontend".to_string(),
        display_name: "前端/UI 专家".to_string(),
        role: "界面设计、样式开发、响应式适配、UI 审查".to_string(),
        use_cases: vec![
            "新建页面或组件".to_string(),
            "样式优化和动效开发".to_string(),
            "UI 审查和改进建议".to_string(),
            "设计稿转代码".to_string(),
        ],
        parameters: get_common_parameters(),
        prompt_template: r#"创建一个 [页面类型] 页面：

- 风格：[极简/玻璃拟态/便当盒/...]
- 技术栈：[React/Vue/HTML+Tailwind]
- 要求：[响应式/暗色模式/动效]
- 特殊需求：[具体描述]"#
            .to_string(),
        return_format: r#"{
  "status": "success",
  "agent": "frontend",
  "SESSION_ID": "uuid-string",
  "result": "UI 实现结果",
  "duration": "3m00s"
}"#
        .to_string(),
        notes: vec![
            "设计师型开发者，关注间距、色彩、微交互".to_string(),
            "支持多技术栈：React/Vue/Svelte/HTML+Tailwind".to_string(),
            "开始编码前，先确定审美方向".to_string(),
        ],
    }
}

fn get_chore_instructions() -> InstructionOutput {
    InstructionOutput {
        agent: "chore".to_string(),
        display_name: "杂务执行者".to_string(),
        role: "处理简单重复任务，节省 token".to_string(),
        use_cases: vec![
            "文件批量重命名/移动".to_string(),
            "全局文本替换".to_string(),
            "代码格式化/lint 修复".to_string(),
            "依赖版本更新".to_string(),
            "配置文件批量修改".to_string(),
        ],
        parameters: get_common_parameters(),
        prompt_template: r#"将所有 .js 文件重命名为 .ts
将代码中所有 'var' 替换为 'let'
更新 package.json 中所有依赖到最新版本"#
            .to_string(),
        return_format: r#"{
  "status": "success",
  "agent": "chore",
  "SESSION_ID": "uuid-string",
  "result": "已完成：\n- [操作1]\n\n统计：\n- 处理文件数：X",
  "duration": "0m30s"
}"#
        .to_string(),
        notes: vec![
            "简单任务，不需要复杂设计".to_string(),
            "默认不重试（简单任务一次完成）".to_string(),
            "快速执行，120s 空闲超时".to_string(),
        ],
    }
}

fn get_researcher_instructions() -> InstructionOutput {
    InstructionOutput {
        agent: "researcher".to_string(),
        display_name: "网络研究专家".to_string(),
        role: "文档查询、网络搜索、代码搜索、深度研究".to_string(),
        use_cases: vec![
            "查询官方文档（context7）".to_string(),
            "搜索最新技术动态（Exa）".to_string(),
            "搜索 GitHub 代码/Issues/PRs".to_string(),
            "问题诊断和解决方案查找".to_string(),
        ],
        parameters: get_common_parameters(),
        prompt_template: r#"请帮我研究以下问题：

**问题**：[具体问题]
**技术栈**：[相关库/框架]
**期望**：[官方文档链接/代码示例/解决方案]"#
            .to_string(),
        return_format: r#"{
  "status": "success",
  "agent": "researcher",
  "SESSION_ID": "uuid-string",
  "result": "<analysis>...</analysis>\n<results>...</results>",
  "duration": "0m45s"
}"#
        .to_string(),
        notes: vec![
            "专注于外部信息检索，不负责本地代码库搜索".to_string(),
            "本地代码搜索请使用 Claude 的 Explore 代理".to_string(),
            "默认 sandbox 为 read-only".to_string(),
        ],
    }
}

fn get_looker_instructions() -> InstructionOutput {
    InstructionOutput {
        agent: "looker".to_string(),
        display_name: "多模态分析专家".to_string(),
        role: "分析 PDF/图片/图表/架构图/截图".to_string(),
        use_cases: vec![
            "分析 PDF 文档内容".to_string(),
            "描述 UI 截图中的元素".to_string(),
            "解释架构图或流程图".to_string(),
            "从图表中提取数据".to_string(),
        ],
        parameters: vec![
            ParameterInfo {
                name: "file_path".to_string(),
                param_type: "Path".to_string(),
                required: true,
                default: None,
                description: "要分析的媒体文件路径".to_string(),
            },
            ParameterInfo {
                name: "goal".to_string(),
                param_type: "string".to_string(),
                required: true,
                default: None,
                description: "分析目标描述".to_string(),
            },
        ],
        prompt_template: r#"file_path: "/path/to/document.pdf"
goal: "提取文档中关于用户认证的所有内容""#
            .to_string(),
        return_format: r#"{
  "status": "success",
  "agent": "looker",
  "SESSION_ID": "uuid-string",
  "file_analyzed": "/path/to/file",
  "result": "<analysis>...</analysis>",
  "duration": "0m20s"
}"#
        .to_string(),
        notes: vec![
            "仅分析文件，严禁修改".to_string(),
            "默认 sandbox 为 read-only".to_string(),
            "源代码或纯文本文件请使用 Read 工具".to_string(),
        ],
    }
}

fn get_common_parameters() -> Vec<ParameterInfo> {
    vec![
        ParameterInfo {
            name: "prompt".to_string(),
            param_type: "string".to_string(),
            required: true,
            default: None,
            description: "任务提示词".to_string(),
        },
        ParameterInfo {
            name: "cd".to_string(),
            param_type: "Path".to_string(),
            required: true,
            default: Some(".".to_string()),
            description: "工作目录".to_string(),
        },
        ParameterInfo {
            name: "sandbox".to_string(),
            param_type: "string".to_string(),
            required: false,
            default: Some("根据 Agent 类型".to_string()),
            description: "沙箱策略：read-only | workspace-write | danger-full-access".to_string(),
        },
        ParameterInfo {
            name: "session_id".to_string(),
            param_type: "string".to_string(),
            required: false,
            default: None,
            description: "会话 ID，用于多轮对话".to_string(),
        },
        ParameterInfo {
            name: "timeout".to_string(),
            param_type: "int".to_string(),
            required: false,
            default: Some("根据 Agent 类型".to_string()),
            description: "空闲超时（秒）".to_string(),
        },
        ParameterInfo {
            name: "max_duration".to_string(),
            param_type: "int".to_string(),
            required: false,
            default: Some("根据 Agent 类型".to_string()),
            description: "最大执行时长（秒）".to_string(),
        },
        ParameterInfo {
            name: "max_retries".to_string(),
            param_type: "int".to_string(),
            required: false,
            default: Some("根据 Agent 类型".to_string()),
            description: "最大重试次数".to_string(),
        },
    ]
}
