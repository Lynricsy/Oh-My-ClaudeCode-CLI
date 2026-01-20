//! 错误类型定义
//!
//! 定义 OMCC CLI 中所有可能的错误类型

use thiserror::Error;

/// OMCC CLI 错误类型
#[derive(Error, Debug)]
pub enum OmccError {
    /// 空闲超时错误（无输出）
    #[error("空闲超时：{0} 秒内无输出")]
    IdleTimeout(u64),

    /// 总时长超时错误
    #[error("执行超时：超过最大时长 {0} 秒")]
    Timeout(u64),

    /// 命令未找到错误
    #[error("命令未找到：{0}")]
    CommandNotFound(String),

    /// 上游错误（CLI 返回错误）
    #[error("上游错误：{0}")]
    UpstreamError(String),

    /// JSON 解析失败
    #[error("JSON 解析失败：{0}")]
    JsonDecode(String),

    /// 协议错误：缺少 SESSION_ID
    #[error("协议错误：未获取 SESSION_ID")]
    ProtocolMissingSession,

    /// 空响应错误
    #[error("空响应：Agent 未返回任何内容")]
    EmptyResult,

    /// 子进程错误
    #[error("子进程错误：退出码 {exit_code}")]
    SubprocessError {
        exit_code: i32,
        last_lines: Vec<String>,
    },

    /// 配置错误
    #[error("配置错误：{0}")]
    ConfigError(String),

    /// 文件未找到
    #[error("文件未找到：{0}")]
    FileNotFound(String),

    /// IO 错误
    #[error("IO 错误：{0}")]
    IoError(#[from] std::io::Error),

    /// 未预期的异常
    #[error("未预期的异常：{0}")]
    UnexpectedException(String),
}

/// 错误类型枚举（用于 JSON 输出）
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    /// 空闲超时
    IdleTimeout,
    /// 总时长超时
    Timeout,
    /// 命令未找到
    CommandNotFound,
    /// 上游错误
    UpstreamError,
    /// JSON 解析失败
    JsonDecode,
    /// 协议错误：缺少 SESSION_ID
    ProtocolMissingSession,
    /// 空响应
    EmptyResult,
    /// 子进程错误
    SubprocessError,
    /// 配置错误
    ConfigError,
    /// 文件未找到
    FileNotFound,
    /// IO 错误
    IoError,
    /// 未预期的异常
    UnexpectedException,
}

impl From<&OmccError> for ErrorKind {
    fn from(err: &OmccError) -> Self {
        match err {
            OmccError::IdleTimeout(_) => ErrorKind::IdleTimeout,
            OmccError::Timeout(_) => ErrorKind::Timeout,
            OmccError::CommandNotFound(_) => ErrorKind::CommandNotFound,
            OmccError::UpstreamError(_) => ErrorKind::UpstreamError,
            OmccError::JsonDecode(_) => ErrorKind::JsonDecode,
            OmccError::ProtocolMissingSession => ErrorKind::ProtocolMissingSession,
            OmccError::EmptyResult => ErrorKind::EmptyResult,
            OmccError::SubprocessError { .. } => ErrorKind::SubprocessError,
            OmccError::ConfigError(_) => ErrorKind::ConfigError,
            OmccError::FileNotFound(_) => ErrorKind::FileNotFound,
            OmccError::IoError(_) => ErrorKind::IoError,
            OmccError::UnexpectedException(_) => ErrorKind::UnexpectedException,
        }
    }
}
