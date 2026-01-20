//! Agent 执行器
//!
//! 负责调用底层 CLI 工具并处理执行结果

use std::process::Stdio;
use std::time::{Duration, Instant};

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::time::timeout;

use crate::types::{
    AgentConfig, AgentResult, AgentType, CliTool, ErrorDetail, ErrorKind, OmccError,
};

/// Agent 执行器
pub struct AgentExecutor {
    config: AgentConfig,
}

impl AgentExecutor {
    /// 创建新的执行器
    pub fn new(config: AgentConfig) -> Self {
        Self { config }
    }

    /// 执行 Agent 任务
    pub async fn execute(&self) -> AgentResult {
        let start_time = Instant::now();
        let max_retries = self.config.get_max_retries();
        let mut last_error: Option<OmccError> = None;

        for attempt in 0..=max_retries {
            if attempt > 0 {
                // 指数退避
                let delay = Duration::from_millis(500 * 2u64.pow(attempt - 1));
                tokio::time::sleep(delay).await;

                if self.config.log_metrics {
                    eprintln!("[OMCC] 重试第 {} 次...", attempt);
                }
            }

            match self.execute_once().await {
                Ok((session_id, result)) => {
                    return AgentResult::success(
                        self.config.agent_type,
                        session_id,
                        result,
                        start_time.elapsed(),
                    );
                }
                Err(e) => {
                    // 某些错误不应重试
                    if !self.should_retry(&e) {
                        return self.error_to_result(e);
                    }
                    last_error = Some(e);
                }
            }
        }

        // 所有重试都失败
        self.error_to_result(last_error.unwrap_or(OmccError::UnexpectedException(
            "未知错误".to_string(),
        )))
    }

    /// 执行一次 Agent 任务
    async fn execute_once(&self) -> Result<(String, String), OmccError> {
        let cli_tool = self.config.agent_type.cli_tool();
        let mut cmd = self.build_command(cli_tool)?;

        // 构建完整的 prompt（包含系统引导提示词）
        let full_prompt = self.build_full_prompt();

        // 根据 CLI 工具类型决定如何传递 prompt
        match cli_tool {
            CliTool::OpenCode => {
                // OpenCode CLI: prompt 作为命令行参数
                cmd.arg("--").arg(&full_prompt);
            }
            CliTool::Codex | CliTool::Claude => {
                // Codex/Claude CLI: prompt 通过 stdin 传递
            }
        }

        // 启动子进程
        let mut child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    OmccError::CommandNotFound(cli_tool.command().to_string())
                } else {
                    OmccError::IoError(e)
                }
            })?;

        // 对于 Codex/Claude CLI，通过 stdin 传递 prompt
        if cli_tool != CliTool::OpenCode {
            if let Some(stdin) = child.stdin.take() {
                let prompt = full_prompt.clone();
                tokio::spawn(async move {
                    use tokio::io::AsyncWriteExt;
                    let mut stdin = stdin;
                    let _ = stdin.write_all(prompt.as_bytes()).await;
                    let _ = stdin.shutdown().await;
                });
            }
        } else {
            // OpenCode CLI 不需要 stdin
            drop(child.stdin.take());
        }

        // 读取输出
        let stdout = child.stdout.take().ok_or(OmccError::EmptyResult)?;
        let stderr = child.stderr.take();

        let idle_timeout = Duration::from_secs(self.config.get_timeout());
        let max_duration = Duration::from_secs(self.config.get_max_duration());
        let start_time = Instant::now();

        let mut reader = BufReader::new(stdout).lines();
        let mut output_lines: Vec<String> = Vec::new();
        let mut session_id: Option<String> = None;

        loop {
            // 检查总时长
            if max_duration.as_secs() > 0 && start_time.elapsed() > max_duration {
                let _ = child.kill().await;
                return Err(OmccError::Timeout(max_duration.as_secs()));
            }

            // 带超时读取
            match timeout(idle_timeout, reader.next_line()).await {
                Ok(Ok(Some(line))) => {
                    // 尝试解析 JSON 响应
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                        // 提取 SESSION_ID
                        if let Some(sid) = json.get("session_id").and_then(|v| v.as_str()) {
                            session_id = Some(sid.to_string());
                        }
                        // 提取结果
                        if let Some(result) = json.get("result").and_then(|v| v.as_str()) {
                            output_lines.push(result.to_string());
                        } else if let Some(content) = json.get("content").and_then(|v| v.as_str()) {
                            output_lines.push(content.to_string());
                        } else if let Some(message) = json.get("message").and_then(|v| v.as_str()) {
                            output_lines.push(message.to_string());
                        }
                        // 检查错误
                        if let Some(error) = json.get("error").and_then(|v| v.as_str()) {
                            return Err(OmccError::UpstreamError(error.to_string()));
                        }
                    } else {
                        // 非 JSON 行，直接记录
                        output_lines.push(line);
                    }
                }
                Ok(Ok(None)) => {
                    // EOF
                    break;
                }
                Ok(Err(e)) => {
                    return Err(OmccError::IoError(e));
                }
                Err(_) => {
                    // 空闲超时
                    let _ = child.kill().await;
                    return Err(OmccError::IdleTimeout(idle_timeout.as_secs()));
                }
            }
        }

        // 等待进程结束
        let status = child.wait().await.map_err(OmccError::IoError)?;

        // 读取 stderr（如果有）
        let mut stderr_output = String::new();
        if let Some(stderr) = stderr {
            let mut stderr_reader = BufReader::new(stderr);
            let mut line = String::new();
            while let Ok(n) = stderr_reader.read_line(&mut line).await {
                if n == 0 {
                    break;
                }
                stderr_output.push_str(&line);
                line.clear();
            }
        }

        // 检查退出码
        if !status.success() {
            let exit_code = status.code().unwrap_or(-1);
            let last_lines: Vec<String> = output_lines
                .iter()
                .rev()
                .take(20)
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();

            return Err(OmccError::SubprocessError {
                exit_code,
                last_lines,
            });
        }

        // 检查结果
        let result = output_lines.join("\n");
        if result.is_empty() {
            return Err(OmccError::EmptyResult);
        }

        // 生成 SESSION_ID（如果没有从响应中获取）
        let session_id = session_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        Ok((session_id, result))
    }

    /// 构建命令
    fn build_command(&self, cli_tool: CliTool) -> Result<Command, OmccError> {
        let mut cmd = Command::new(cli_tool.command());

        // 设置工作目录
        cmd.current_dir(&self.config.working_dir);

        // 根据不同的 CLI 工具设置不同的参数格式
        match cli_tool {
            CliTool::Codex => {
                // codex exec --sandbox xxx --cd xxx --json
                cmd.arg("exec");
                cmd.arg("--sandbox").arg(self.config.sandbox.as_arg());
                cmd.arg("--cd").arg(&self.config.working_dir);
                cmd.arg("--json");

                // Codex 特定参数
                if self.config.skip_git_repo_check {
                    cmd.arg("--skip-git-repo-check");
                }
                if self.config.yolo {
                    cmd.arg("--yolo");
                }
                if let Some(ref profile) = self.config.profile {
                    cmd.arg("--profile").arg(profile);
                }
                // 图片
                for image in &self.config.images {
                    cmd.arg("--image").arg(image);
                }
                // 会话复用
                if let Some(ref session_id) = self.config.session_id {
                    cmd.arg("resume").arg(session_id);
                }
            }
            CliTool::OpenCode => {
                // opencode run --format json [--model xxx] message
                cmd.arg("run");
                cmd.arg("--format").arg("json");

                // 模型
                if let Some(ref model) = self.config.model {
                    cmd.arg("--model").arg(model);
                }

                // Looker 需要分析文件（通过 --image 参数）
                if self.config.agent_type == AgentType::Looker {
                    if let Some(ref file_path) = self.config.file_path {
                        cmd.arg("--image").arg(file_path);
                    }
                }
            }
            CliTool::Claude => {
                // claude -p --output-format stream-json --sandbox xxx
                cmd.arg("-p");
                cmd.arg("--output-format").arg("stream-json");
                cmd.arg("--sandbox").arg(self.config.sandbox.as_arg());

                // 模型
                if let Some(ref model) = self.config.model {
                    cmd.arg("--model").arg(model);
                }

                // 会话复用
                if let Some(ref session_id) = self.config.session_id {
                    cmd.arg("--resume").arg(session_id);
                }
            }
        }

        Ok(cmd)
    }

    /// 获取 Agent 的引导提示词（追加到用户 prompt 后面）
    fn get_guidance_prompt(&self) -> &'static str {
        match self.config.agent_type {
            AgentType::Reviewer => include_str!("../instructions/reviewer_system.txt"),
            AgentType::Advisor => include_str!("../instructions/advisor_system.txt"),
            AgentType::Chore => include_str!("../instructions/chore_system.txt"),
            AgentType::Researcher => include_str!("../instructions/researcher_system.txt"),
            AgentType::Looker => include_str!("../instructions/looker_system.txt"),
        }
    }

    /// 构建完整的 prompt（用户 prompt + 引导提示词）
    fn build_full_prompt(&self) -> String {
        let guidance = self.get_guidance_prompt();
        format!(
            "{}\n\n---\n\n**最终回复要求**：在你的最终回复中，必须包含完整的工作总结：\n\
            1. **执行过程**：简述你做了哪些操作\n\
            2. **关键决策**：解释为什么选择这种方案\n\
            3. **最终结果**：描述完成的效果或结论\n\
            4. **后续建议**：如有进一步优化空间，给出建议\n\n\
            这样做的原因：调用你的上层 AI 只能看到你的最终回复，无法看到中间的执行过程。\n\n\
            ---\n\n{}",
            self.config.prompt, guidance
        )
    }

    /// 判断是否应该重试
    fn should_retry(&self, error: &OmccError) -> bool {
        matches!(
            error,
            OmccError::IdleTimeout(_)
                | OmccError::Timeout(_)
                | OmccError::IoError(_)
                | OmccError::UpstreamError(_)
        )
    }

    /// 将错误转换为结果
    fn error_to_result(&self, error: OmccError) -> AgentResult {
        let error_kind = ErrorKind::from(&error);
        let error_detail = match &error {
            OmccError::SubprocessError {
                exit_code,
                last_lines,
            } => Some(ErrorDetail {
                message: error.to_string(),
                exit_code: Some(*exit_code),
                last_lines: last_lines.clone(),
                json_decode_errors: None,
                idle_timeout_s: None,
                max_duration_s: None,
                retries: Some(self.config.get_max_retries()),
            }),
            OmccError::IdleTimeout(secs) => Some(ErrorDetail {
                message: error.to_string(),
                exit_code: None,
                last_lines: vec![],
                json_decode_errors: None,
                idle_timeout_s: Some(*secs),
                max_duration_s: None,
                retries: Some(self.config.get_max_retries()),
            }),
            OmccError::Timeout(secs) => Some(ErrorDetail {
                message: error.to_string(),
                exit_code: None,
                last_lines: vec![],
                json_decode_errors: None,
                idle_timeout_s: None,
                max_duration_s: Some(*secs),
                retries: Some(self.config.get_max_retries()),
            }),
            _ => Some(ErrorDetail {
                message: error.to_string(),
                exit_code: None,
                last_lines: vec![],
                json_decode_errors: None,
                idle_timeout_s: None,
                max_duration_s: None,
                retries: Some(self.config.get_max_retries()),
            }),
        };

        AgentResult::failure(self.config.agent_type, error.to_string(), error_kind, error_detail)
    }
}
