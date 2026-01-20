//! 输出类型定义
//!
//! 定义 Agent 执行结果的结构化输出

use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::config::AgentType;
use super::error::ErrorKind;

/// Agent 执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum AgentResult {
    /// 执行成功
    Success(SuccessResult),
    /// 执行失败
    Failure(FailureResult),
}

impl AgentResult {
    /// 创建成功结果
    pub fn success(
        agent: AgentType,
        session_id: String,
        result: String,
        duration: Duration,
    ) -> Self {
        AgentResult::Success(SuccessResult {
            agent: agent.name().to_string(),
            session_id,
            result,
            duration: format_duration(duration),
            metrics: None,
        })
    }

    /// 创建失败结果
    pub fn failure(
        agent: AgentType,
        error: String,
        error_kind: ErrorKind,
        error_detail: Option<ErrorDetail>,
    ) -> Self {
        AgentResult::Failure(FailureResult {
            agent: agent.name().to_string(),
            error,
            error_kind,
            error_detail,
        })
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        matches!(self, AgentResult::Success(_))
    }

    /// 获取 Agent 名称
    pub fn agent_name(&self) -> &str {
        match self {
            AgentResult::Success(r) => &r.agent,
            AgentResult::Failure(r) => &r.agent,
        }
    }
}

/// 成功结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResult {
    /// Agent 名称
    pub agent: String,

    /// 会话 ID
    #[serde(rename = "SESSION_ID")]
    pub session_id: String,

    /// 执行结果
    pub result: String,

    /// 执行时长
    pub duration: String,

    /// 指标数据（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
}

/// 失败结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureResult {
    /// Agent 名称
    pub agent: String,

    /// 错误摘要
    pub error: String,

    /// 错误类型
    pub error_kind: ErrorKind,

    /// 错误详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
}

/// 错误详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    /// 错误消息
    pub message: String,

    /// 退出码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,

    /// 最后几行输出
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub last_lines: Vec<String>,

    /// JSON 解析错误次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_decode_errors: Option<u32>,

    /// 空闲超时时间（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_s: Option<u64>,

    /// 最大执行时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_s: Option<u64>,

    /// 重试次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<u32>,
}

/// 执行指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    /// 总执行时长（毫秒）
    pub duration_ms: u64,

    /// 输入 token 数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u64>,

    /// 输出 token 数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<u64>,

    /// 重试次数
    pub retries: u32,
}

/// 格式化时长
fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let mins = secs / 60;
    let secs = secs % 60;
    if mins > 0 {
        format!("{}m{}s", mins, secs)
    } else {
        format!("{}s", secs)
    }
}

/// 指导内容输出（用于 --xxx-instructions）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionOutput {
    /// Agent 名称
    pub agent: String,

    /// Agent 中文名称
    pub display_name: String,

    /// 角色定位
    pub role: String,

    /// 使用场景
    pub use_cases: Vec<String>,

    /// 参数说明
    pub parameters: Vec<ParameterInfo>,

    /// Prompt 模板
    pub prompt_template: String,

    /// 返回值说明
    pub return_format: String,

    /// 注意事项
    pub notes: Vec<String>,
}

/// 参数信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfo {
    /// 参数名
    pub name: String,

    /// 参数类型
    #[serde(rename = "type")]
    pub param_type: String,

    /// 是否必填
    pub required: bool,

    /// 默认值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    /// 说明
    pub description: String,
}
