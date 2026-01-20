//! CLI 参数定义
//!
//! 使用 clap 定义命令行参数和子命令

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Oh-My-ClaudeCode CLI - AI 多代理协作命令行工具
#[derive(Parser, Debug)]
#[command(name = "omcc")]
#[command(author = "Wine Fox <fox@ling.plus>")]
#[command(version)]
#[command(about = "OMCC - AI 多代理协作命令行工具", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// 子命令
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// 输出 Coder Agent 使用指南
    #[arg(long = "coder-instructions", global = true)]
    pub coder_instructions: bool,

    /// 输出 Reviewer Agent 使用指南
    #[arg(long = "reviewer-instructions", global = true)]
    pub reviewer_instructions: bool,

    /// 输出 Advisor Agent 使用指南
    #[arg(long = "advisor-instructions", global = true)]
    pub advisor_instructions: bool,

    /// 输出 Frontend Agent 使用指南
    #[arg(long = "frontend-instructions", global = true)]
    pub frontend_instructions: bool,

    /// 输出 Chore Agent 使用指南
    #[arg(long = "chore-instructions", global = true)]
    pub chore_instructions: bool,

    /// 输出 Researcher Agent 使用指南
    #[arg(long = "researcher-instructions", global = true)]
    pub researcher_instructions: bool,

    /// 输出 Looker Agent 使用指南
    #[arg(long = "looker-instructions", global = true)]
    pub looker_instructions: bool,

    /// 输出完整工作流指南
    #[arg(long = "workflow", global = true)]
    pub workflow: bool,

    /// 输出全局提示词模板
    #[arg(long = "global-prompt", global = true)]
    pub global_prompt: bool,

    /// 以 JSON 格式输出
    #[arg(long = "json", short = 'j', global = true)]
    pub json_output: bool,
}

/// 子命令定义
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 调用 Coder 执行代码生成或修改任务
    #[command(name = "coder")]
    Coder(CoderArgs),

    /// 调用 Reviewer 进行代码审核
    #[command(name = "reviewer")]
    Reviewer(ReviewerArgs),

    /// 调用 Advisor 进行技术咨询或代码执行
    #[command(name = "advisor")]
    Advisor(AdvisorArgs),

    /// 调用 Frontend 进行前端/UI 开发
    #[command(name = "frontend")]
    Frontend(FrontendArgs),

    /// 调用 Chore 执行杂务任务
    #[command(name = "chore")]
    Chore(ChoreArgs),

    /// 调用 Researcher 进行网络研究
    #[command(name = "researcher")]
    Researcher(ResearcherArgs),

    /// 调用 Looker 进行多模态分析
    #[command(name = "looker")]
    Looker(LookerArgs),

    /// 列出所有可用的 Agent
    #[command(name = "list")]
    List,

    /// 显示版本和配置信息
    #[command(name = "info")]
    Info,
}

/// 沙箱策略枚举
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SandboxArg {
    /// 只读模式
    #[value(name = "read-only")]
    ReadOnly,
    /// 允许写工作区
    #[value(name = "workspace-write")]
    WorkspaceWrite,
    /// 完全访问（危险）
    #[value(name = "danger-full-access")]
    DangerFullAccess,
}

impl From<SandboxArg> for crate::types::SandboxPolicy {
    fn from(arg: SandboxArg) -> Self {
        match arg {
            SandboxArg::ReadOnly => crate::types::SandboxPolicy::ReadOnly,
            SandboxArg::WorkspaceWrite => crate::types::SandboxPolicy::WorkspaceWrite,
            SandboxArg::DangerFullAccess => crate::types::SandboxPolicy::DangerFullAccess,
        }
    }
}

/// 通用 Agent 参数
#[derive(Args, Debug, Clone)]
pub struct CommonAgentArgs {
    /// 工作目录
    #[arg(short = 'C', long = "cd", default_value = ".")]
    pub working_dir: PathBuf,

    /// 沙箱策略
    #[arg(long = "sandbox", short = 's')]
    pub sandbox: Option<SandboxArg>,

    /// 会话 ID（用于多轮对话）
    #[arg(long = "session-id", short = 'S', env = "OMCC_SESSION_ID")]
    pub session_id: Option<String>,

    /// 空闲超时（秒）
    #[arg(long = "timeout", short = 't')]
    pub timeout: Option<u64>,

    /// 最大执行时长（秒）
    #[arg(long = "max-duration", short = 'd')]
    pub max_duration: Option<u64>,

    /// 最大重试次数
    #[arg(long = "max-retries", short = 'r')]
    pub max_retries: Option<u32>,

    /// 返回完整消息
    #[arg(long = "return-all-messages")]
    pub return_all_messages: bool,

    /// 返回指标数据
    #[arg(long = "return-metrics")]
    pub return_metrics: bool,

    /// 将指标输出到 stderr
    #[arg(long = "log-metrics")]
    pub log_metrics: bool,

    /// 指定模型
    #[arg(long = "model", short = 'm')]
    pub model: Option<String>,
}

/// Coder Agent 参数
#[derive(Args, Debug)]
pub struct CoderArgs {
    /// 任务提示词（从 stdin 读取时可省略）
    #[arg(value_name = "PROMPT")]
    pub prompt: Option<String>,

    /// 从 stdin 读取提示词
    #[arg(long = "stdin", short = 'i')]
    pub from_stdin: bool,

    /// 从文件读取提示词
    #[arg(long = "file", short = 'f')]
    pub from_file: Option<PathBuf>,

    #[command(flatten)]
    pub common: CommonAgentArgs,
}

/// Reviewer Agent 参数
#[derive(Args, Debug)]
pub struct ReviewerArgs {
    /// 审核任务描述（从 stdin 读取时可省略）
    #[arg(value_name = "PROMPT")]
    pub prompt: Option<String>,

    /// 从 stdin 读取提示词
    #[arg(long = "stdin", short = 'i')]
    pub from_stdin: bool,

    /// 从文件读取提示词
    #[arg(long = "file", short = 'f')]
    pub from_file: Option<PathBuf>,

    /// 附加图片文件
    #[arg(long = "image", short = 'I')]
    pub images: Vec<PathBuf>,

    /// 跳过 Git 仓库检查
    #[arg(long = "skip-git-check")]
    pub skip_git_repo_check: bool,

    /// YOLO 模式（跳过审批）
    #[arg(long = "yolo")]
    pub yolo: bool,

    /// 配置文件名称
    #[arg(long = "profile")]
    pub profile: Option<String>,

    #[command(flatten)]
    pub common: CommonAgentArgs,
}

/// Advisor Agent 参数
#[derive(Args, Debug)]
pub struct AdvisorArgs {
    /// 任务提示词（从 stdin 读取时可省略）
    #[arg(value_name = "PROMPT")]
    pub prompt: Option<String>,

    /// 从 stdin 读取提示词
    #[arg(long = "stdin", short = 'i')]
    pub from_stdin: bool,

    /// 从文件读取提示词
    #[arg(long = "file", short = 'f')]
    pub from_file: Option<PathBuf>,

    #[command(flatten)]
    pub common: CommonAgentArgs,
}

/// Frontend Agent 参数
#[derive(Args, Debug)]
pub struct FrontendArgs {
    /// 前端/UI 任务描述（从 stdin 读取时可省略）
    #[arg(value_name = "PROMPT")]
    pub prompt: Option<String>,

    /// 从 stdin 读取提示词
    #[arg(long = "stdin", short = 'i')]
    pub from_stdin: bool,

    /// 从文件读取提示词
    #[arg(long = "file", short = 'f')]
    pub from_file: Option<PathBuf>,

    #[command(flatten)]
    pub common: CommonAgentArgs,
}

/// Chore Agent 参数
#[derive(Args, Debug)]
pub struct ChoreArgs {
    /// 杂务任务描述（从 stdin 读取时可省略）
    #[arg(value_name = "PROMPT")]
    pub prompt: Option<String>,

    /// 从 stdin 读取提示词
    #[arg(long = "stdin", short = 'i')]
    pub from_stdin: bool,

    /// 从文件读取提示词
    #[arg(long = "file", short = 'f')]
    pub from_file: Option<PathBuf>,

    #[command(flatten)]
    pub common: CommonAgentArgs,
}

/// Researcher Agent 参数
#[derive(Args, Debug)]
pub struct ResearcherArgs {
    /// 研究任务描述（从 stdin 读取时可省略）
    #[arg(value_name = "PROMPT")]
    pub prompt: Option<String>,

    /// 从 stdin 读取提示词
    #[arg(long = "stdin", short = 'i')]
    pub from_stdin: bool,

    /// 从文件读取提示词
    #[arg(long = "file", short = 'f')]
    pub from_file: Option<PathBuf>,

    #[command(flatten)]
    pub common: CommonAgentArgs,
}

/// Looker Agent 参数
#[derive(Args, Debug)]
pub struct LookerArgs {
    /// 要分析的文件路径
    #[arg(value_name = "FILE")]
    pub file_path: PathBuf,

    /// 分析目标描述
    #[arg(long = "goal", short = 'g')]
    pub goal: Option<String>,

    /// 从 stdin 读取分析目标
    #[arg(long = "stdin", short = 'i')]
    pub from_stdin: bool,

    /// 从文件读取分析目标
    #[arg(long = "file", short = 'f')]
    pub from_file: Option<PathBuf>,

    #[command(flatten)]
    pub common: CommonAgentArgs,
}
