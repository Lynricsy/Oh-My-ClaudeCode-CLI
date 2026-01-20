//! OMCC - Oh-My-ClaudeCode CLI
//!
//! AI 多代理协作命令行工具
//!
//! ## 功能特性
//!
//! - **多代理协作**：Reviewer、Advisor、Chore、Researcher、Looker
//! - **结构化输入输出**：JSON 格式，便于程序集成
//! - **会话管理**：支持多轮对话，保持上下文
//! - **灵活配置**：沙箱策略、超时控制、重试机制
//!
//! ## 使用示例
//!
//! ```bash
//! # 调用 Reviewer 审核代码
//! omcc reviewer -C /path/to/project "请 review 代码改动"
//!
//! # 调用 Advisor 获取建议
//! omcc advisor -C /path/to/project "评估架构方案"
//!
//! # 获取使用指南
//! omcc --reviewer-instructions
//! omcc --workflow
//! ```

pub mod agents;
pub mod cli;
pub mod instructions;
pub mod types;

pub use agents::AgentExecutor;
pub use cli::{Cli, Commands};
pub use types::*;
