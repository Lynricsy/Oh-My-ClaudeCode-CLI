//! 指导内容模块
//!
//! 包含所有 Agent 的 skill 文档，用于告诉主 AI 如何使用对应 Agent

use crate::types::AgentType;

/// 获取 Agent 的 skill 文档（给主 AI 使用的指南）
pub fn get_agent_skill(agent_type: AgentType) -> String {
    match agent_type {
        AgentType::Reviewer => include_str!("skills/reviewer.md").to_string(),
        AgentType::Advisor => include_str!("skills/advisor.md").to_string(),
        AgentType::Chore => include_str!("skills/chore.md").to_string(),
        AgentType::Researcher => include_str!("skills/researcher.md").to_string(),
        AgentType::Looker => include_str!("skills/looker.md").to_string(),
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
