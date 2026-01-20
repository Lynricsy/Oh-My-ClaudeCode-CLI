//! OMCC CLI 主入口
//!
//! Oh-My-ClaudeCode 命令行工具

use std::io::{self, Read};
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use omcc::agents::AgentExecutor;
use omcc::cli::{
    AdvisorArgs, ChoreArgs, Cli, Commands, CommonAgentArgs, LookerArgs, ResearcherArgs,
    ReviewerArgs,
};
use omcc::instructions::{get_agent_skill, get_global_prompt, get_workflow_instructions};
use omcc::types::{AgentConfig, AgentResult, AgentType};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // 处理指导内容输出（skill 文档，给主 AI 使用）
    if cli.reviewer_instructions {
        println!("{}", get_agent_skill(AgentType::Reviewer));
        return Ok(());
    }
    if cli.advisor_instructions {
        println!("{}", get_agent_skill(AgentType::Advisor));
        return Ok(());
    }
    if cli.chore_instructions {
        println!("{}", get_agent_skill(AgentType::Chore));
        return Ok(());
    }
    if cli.researcher_instructions {
        println!("{}", get_agent_skill(AgentType::Researcher));
        return Ok(());
    }
    if cli.looker_instructions {
        println!("{}", get_agent_skill(AgentType::Looker));
        return Ok(());
    }
    if cli.workflow {
        println!("{}", get_workflow_instructions());
        return Ok(());
    }
    if cli.global_prompt {
        println!("{}", get_global_prompt());
        return Ok(());
    }

    // 处理子命令
    match cli.command {
        Some(Commands::Reviewer(args)) => {
            execute_agent(
                AgentType::Reviewer,
                build_reviewer_config(args)?,
                cli.json_output,
            )
            .await
        }
        Some(Commands::Advisor(args)) => {
            execute_agent(
                AgentType::Advisor,
                build_advisor_config(args)?,
                cli.json_output,
            )
            .await
        }
        Some(Commands::Chore(args)) => {
            execute_agent(AgentType::Chore, build_chore_config(args)?, cli.json_output).await
        }
        Some(Commands::Researcher(args)) => {
            execute_agent(
                AgentType::Researcher,
                build_researcher_config(args)?,
                cli.json_output,
            )
            .await
        }
        Some(Commands::Looker(args)) => {
            execute_agent(
                AgentType::Looker,
                build_looker_config(args)?,
                cli.json_output,
            )
            .await
        }
        Some(Commands::List) => {
            print_agent_list(cli.json_output);
            Ok(())
        }
        Some(Commands::Info) => {
            print_info(cli.json_output);
            Ok(())
        }
        None => {
            // 没有子命令时显示帮助
            println!("{}", get_global_prompt());
            Ok(())
        }
    }
}

/// 执行 Agent 任务
async fn execute_agent(
    _agent_type: AgentType,
    config: AgentConfig,
    json_output: bool,
) -> Result<()> {
    let executor = AgentExecutor::new(config);
    let result = executor.execute().await;

    output_result(&result, json_output);

    if result.is_success() {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

/// 输出结果
fn output_result(result: &AgentResult, json_output: bool) {
    if json_output {
        println!("{}", serde_json::to_string_pretty(result).unwrap());
    } else {
        match result {
            AgentResult::Success(success) => {
                println!("[{}] 执行成功 ({})", success.agent, success.duration);
                println!("SESSION_ID: {}", success.session_id);
                println!();
                println!("{}", success.result);
            }
            AgentResult::Failure(failure) => {
                eprintln!("[{}] 执行失败", failure.agent);
                eprintln!("错误类型: {:?}", failure.error_kind);
                eprintln!("错误信息: {}", failure.error);
                if let Some(detail) = &failure.error_detail {
                    if let Some(exit_code) = detail.exit_code {
                        eprintln!("退出码: {}", exit_code);
                    }
                    if !detail.last_lines.is_empty() {
                        eprintln!("最后输出:");
                        for line in &detail.last_lines {
                            eprintln!("  {}", line);
                        }
                    }
                }
            }
        }
    }
}

/// 打印 Agent 列表
fn print_agent_list(json_output: bool) {
    let agents = vec![
        AgentType::Reviewer,
        AgentType::Advisor,
        AgentType::Chore,
        AgentType::Researcher,
        AgentType::Looker,
    ];

    if json_output {
        let list: Vec<_> = agents
            .iter()
            .map(|a| {
                serde_json::json!({
                    "name": a.name(),
                    "display_name": a.display_name(),
                    "sandbox": a.default_sandbox().as_arg(),
                    "max_retries": a.default_max_retries(),
                    "timeout": a.default_timeout(),
                    "max_duration": a.default_max_duration(),
                    "cli_tool": a.cli_tool().command(),
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&list).unwrap());
    } else {
        println!("可用的 Agent 列表：");
        println!();
        println!(
            "{:<12} {:<16} {:<18} {:<10} {:<8}",
            "名称", "中文名", "沙箱模式", "底层CLI", "重试"
        );
        println!("{}", "-".repeat(70));
        for agent in agents {
            println!(
                "{:<12} {:<16} {:<18} {:<10} {:<8}",
                agent.name(),
                agent.display_name(),
                agent.default_sandbox().as_arg(),
                agent.cli_tool().command(),
                agent.default_max_retries()
            );
        }
    }
}

/// 打印版本和配置信息
fn print_info(json_output: bool) {
    let info = serde_json::json!({
        "name": "omcc",
        "version": env!("CARGO_PKG_VERSION"),
        "description": env!("CARGO_PKG_DESCRIPTION"),
        "authors": env!("CARGO_PKG_AUTHORS"),
        "repository": env!("CARGO_PKG_REPOSITORY"),
    });

    if json_output {
        println!("{}", serde_json::to_string_pretty(&info).unwrap());
    } else {
        println!("OMCC - Oh-My-ClaudeCode CLI");
        println!("版本: {}", env!("CARGO_PKG_VERSION"));
        println!("描述: {}", env!("CARGO_PKG_DESCRIPTION"));
        println!("作者: {}", env!("CARGO_PKG_AUTHORS"));
        println!("仓库: {}", env!("CARGO_PKG_REPOSITORY"));
    }
}

/// 读取提示词
fn read_prompt(
    prompt: Option<String>,
    from_stdin: bool,
    from_file: Option<PathBuf>,
) -> Result<String> {
    if from_stdin {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else if let Some(file_path) = from_file {
        Ok(std::fs::read_to_string(file_path)?)
    } else if let Some(prompt) = prompt {
        Ok(prompt)
    } else {
        anyhow::bail!("必须提供提示词：通过参数、--stdin 或 --file")
    }
}

/// 构建 Reviewer 配置
fn build_reviewer_config(args: ReviewerArgs) -> Result<AgentConfig> {
    let prompt = read_prompt(args.prompt, args.from_stdin, args.from_file)?;
    let working_dir = args.common.working_dir.clone();
    let mut config = AgentConfig::new(AgentType::Reviewer, prompt, working_dir);
    apply_common_args(&mut config, &args.common);
    config.images = args.images;
    config.skip_git_repo_check = args.skip_git_repo_check;
    config.yolo = args.yolo;
    config.profile = args.profile;
    Ok(config)
}

/// 构建 Advisor 配置
fn build_advisor_config(args: AdvisorArgs) -> Result<AgentConfig> {
    let prompt = read_prompt(args.prompt, args.from_stdin, args.from_file)?;
    let working_dir = args.common.working_dir.clone();
    let mut config = AgentConfig::new(AgentType::Advisor, prompt, working_dir);
    apply_common_args(&mut config, &args.common);
    Ok(config)
}

/// 构建 Chore 配置
fn build_chore_config(args: ChoreArgs) -> Result<AgentConfig> {
    let prompt = read_prompt(args.prompt, args.from_stdin, args.from_file)?;
    let working_dir = args.common.working_dir.clone();
    let mut config = AgentConfig::new(AgentType::Chore, prompt, working_dir);
    apply_common_args(&mut config, &args.common);
    Ok(config)
}

/// 构建 Researcher 配置
fn build_researcher_config(args: ResearcherArgs) -> Result<AgentConfig> {
    let prompt = read_prompt(args.prompt, args.from_stdin, args.from_file)?;
    let working_dir = args.common.working_dir.clone();
    let mut config = AgentConfig::new(AgentType::Researcher, prompt, working_dir);
    apply_common_args(&mut config, &args.common);
    Ok(config)
}

/// 构建 Looker 配置
fn build_looker_config(args: LookerArgs) -> Result<AgentConfig> {
    let goal = if args.from_stdin {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else if let Some(file_path) = args.from_file {
        std::fs::read_to_string(file_path)?
    } else if let Some(goal) = args.goal {
        goal
    } else {
        anyhow::bail!("必须提供分析目标：通过 --goal、--stdin 或 --file")
    };

    let working_dir = args.common.working_dir.clone();
    let mut config = AgentConfig::new(AgentType::Looker, goal.clone(), working_dir);
    apply_common_args(&mut config, &args.common);
    config.file_path = Some(args.file_path);
    config.goal = Some(goal);
    Ok(config)
}

/// 应用通用参数
fn apply_common_args(config: &mut AgentConfig, args: &CommonAgentArgs) {
    if let Some(sandbox) = args.sandbox {
        config.sandbox = sandbox.into();
    }
    config.session_id = args.session_id.clone();
    config.timeout = args.timeout;
    config.max_duration = args.max_duration;
    config.max_retries = args.max_retries;
    config.return_all_messages = args.return_all_messages;
    config.return_metrics = args.return_metrics;
    config.log_metrics = args.log_metrics;
    config.model = args.model.clone();
}
