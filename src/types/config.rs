//! 配置类型定义
//!
//! 定义 Agent 配置和运行时参数

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Agent 类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentType {
    /// 代码执行者
    Coder,
    /// 代码审核者（原 Codex）
    Reviewer,
    /// 高阶顾问（原 Gemini）
    Advisor,
    /// 前端/UI 专家
    Frontend,
    /// 杂务执行者
    Chore,
    /// 网络研究专家（原 Librarian）
    Researcher,
    /// 多模态分析专家
    Looker,
}

impl AgentType {
    /// 获取 Agent 名称
    pub fn name(&self) -> &'static str {
        match self {
            AgentType::Coder => "coder",
            AgentType::Reviewer => "reviewer",
            AgentType::Advisor => "advisor",
            AgentType::Frontend => "frontend",
            AgentType::Chore => "chore",
            AgentType::Researcher => "researcher",
            AgentType::Looker => "looker",
        }
    }

    /// 获取 Agent 中文名称
    pub fn display_name(&self) -> &'static str {
        match self {
            AgentType::Coder => "代码执行者",
            AgentType::Reviewer => "代码审核者",
            AgentType::Advisor => "高阶顾问",
            AgentType::Frontend => "前端/UI 专家",
            AgentType::Chore => "杂务执行者",
            AgentType::Researcher => "网络研究专家",
            AgentType::Looker => "多模态分析专家",
        }
    }

    /// 获取默认的沙箱策略
    pub fn default_sandbox(&self) -> SandboxPolicy {
        match self {
            AgentType::Coder => SandboxPolicy::WorkspaceWrite,
            AgentType::Reviewer => SandboxPolicy::ReadOnly,
            AgentType::Advisor => SandboxPolicy::WorkspaceWrite,
            AgentType::Frontend => SandboxPolicy::WorkspaceWrite,
            AgentType::Chore => SandboxPolicy::WorkspaceWrite,
            AgentType::Researcher => SandboxPolicy::ReadOnly,
            AgentType::Looker => SandboxPolicy::ReadOnly,
        }
    }

    /// 获取默认的最大重试次数
    pub fn default_max_retries(&self) -> u32 {
        match self {
            AgentType::Coder => 0,    // 有写入副作用，默认不重试
            AgentType::Reviewer => 1, // 只读，可安全重试
            AgentType::Advisor => 1,
            AgentType::Frontend => 1,
            AgentType::Chore => 0, // 有写入副作用，默认不重试
            AgentType::Researcher => 1,
            AgentType::Looker => 1,
        }
    }

    /// 获取默认的空闲超时时间（秒）
    pub fn default_timeout(&self) -> u64 {
        match self {
            AgentType::Coder => 300,
            AgentType::Reviewer => 300,
            AgentType::Advisor => 300,
            AgentType::Frontend => 180,
            AgentType::Chore => 120,
            AgentType::Researcher => 120,
            AgentType::Looker => 120,
        }
    }

    /// 获取默认的最大执行时长（秒）
    pub fn default_max_duration(&self) -> u64 {
        match self {
            AgentType::Coder => 3600,
            AgentType::Reviewer => 1800,
            AgentType::Advisor => 3600,
            AgentType::Frontend => 3600,
            AgentType::Chore => 600,
            AgentType::Researcher => 3600,
            AgentType::Looker => 3600,
        }
    }

    /// 获取使用的底层 CLI 工具
    pub fn cli_tool(&self) -> CliTool {
        match self {
            AgentType::Coder => CliTool::Claude,
            AgentType::Reviewer => CliTool::Codex,
            AgentType::Advisor => CliTool::Gemini,
            AgentType::Frontend => CliTool::Gemini,
            AgentType::Chore => CliTool::Claude,
            AgentType::Researcher => CliTool::Gemini,
            AgentType::Looker => CliTool::Gemini,
        }
    }
}

/// CLI 工具类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliTool {
    /// Claude CLI
    Claude,
    /// Codex CLI (OpenAI)
    Codex,
    /// Gemini CLI
    Gemini,
}

impl CliTool {
    /// 获取 CLI 命令名
    pub fn command(&self) -> &'static str {
        match self {
            CliTool::Claude => "claude",
            CliTool::Codex => "codex",
            CliTool::Gemini => "gemini",
        }
    }
}

/// 沙箱策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SandboxPolicy {
    /// 只读模式
    #[default]
    ReadOnly,
    /// 允许写工作区
    WorkspaceWrite,
    /// 完全访问（危险）
    DangerFullAccess,
}

impl SandboxPolicy {
    /// 转换为 CLI 参数值
    pub fn as_arg(&self) -> &'static str {
        match self {
            SandboxPolicy::ReadOnly => "read-only",
            SandboxPolicy::WorkspaceWrite => "workspace-write",
            SandboxPolicy::DangerFullAccess => "danger-full-access",
        }
    }
}

impl std::str::FromStr for SandboxPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "read-only" => Ok(SandboxPolicy::ReadOnly),
            "workspace-write" => Ok(SandboxPolicy::WorkspaceWrite),
            "danger-full-access" => Ok(SandboxPolicy::DangerFullAccess),
            _ => Err(format!("未知的沙箱策略: {}", s)),
        }
    }
}

/// Agent 运行配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent 类型
    pub agent_type: AgentType,

    /// 任务提示词
    pub prompt: String,

    /// 工作目录
    pub working_dir: PathBuf,

    /// 沙箱策略
    #[serde(default)]
    pub sandbox: SandboxPolicy,

    /// 会话 ID（用于多轮对话）
    #[serde(default)]
    pub session_id: Option<String>,

    /// 空闲超时（秒）
    #[serde(default)]
    pub timeout: Option<u64>,

    /// 最大执行时长（秒）
    #[serde(default)]
    pub max_duration: Option<u64>,

    /// 最大重试次数
    #[serde(default)]
    pub max_retries: Option<u32>,

    /// 是否返回完整消息
    #[serde(default)]
    pub return_all_messages: bool,

    /// 是否返回指标数据
    #[serde(default)]
    pub return_metrics: bool,

    /// 是否将指标输出到 stderr
    #[serde(default)]
    pub log_metrics: bool,

    /// 指定模型
    #[serde(default)]
    pub model: Option<String>,

    /// 附加图片（用于 Reviewer/Looker）
    #[serde(default)]
    pub images: Vec<PathBuf>,

    /// 要分析的文件路径（用于 Looker）
    #[serde(default)]
    pub file_path: Option<PathBuf>,

    /// 分析目标（用于 Looker）
    #[serde(default)]
    pub goal: Option<String>,

    /// 是否跳过 Git 仓库检查
    #[serde(default)]
    pub skip_git_repo_check: bool,

    /// 是否启用 YOLO 模式（跳过审批）
    #[serde(default)]
    pub yolo: bool,

    /// 配置文件名称
    #[serde(default)]
    pub profile: Option<String>,
}

impl AgentConfig {
    /// 创建新的 Agent 配置
    pub fn new(agent_type: AgentType, prompt: String, working_dir: PathBuf) -> Self {
        Self {
            agent_type,
            prompt,
            working_dir,
            sandbox: agent_type.default_sandbox(),
            session_id: None,
            timeout: None,
            max_duration: None,
            max_retries: None,
            return_all_messages: false,
            return_metrics: false,
            log_metrics: false,
            model: None,
            images: Vec::new(),
            file_path: None,
            goal: None,
            skip_git_repo_check: true,
            yolo: false,
            profile: None,
        }
    }

    /// 获取实际的超时时间
    pub fn get_timeout(&self) -> u64 {
        self.timeout.unwrap_or_else(|| self.agent_type.default_timeout())
    }

    /// 获取实际的最大执行时长
    pub fn get_max_duration(&self) -> u64 {
        self.max_duration
            .unwrap_or_else(|| self.agent_type.default_max_duration())
    }

    /// 获取实际的最大重试次数
    pub fn get_max_retries(&self) -> u32 {
        self.max_retries
            .unwrap_or_else(|| self.agent_type.default_max_retries())
    }
}
