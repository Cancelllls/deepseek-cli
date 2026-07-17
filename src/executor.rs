use crate::api::{ApiClient, Message};
use crate::skills;
use crate::state::{Phase, WorkflowState};
use anyhow::Result;
use colored::*;
use regex::Regex;
use std::process::Command;
use std::time::Instant;

#[derive(Debug)]
pub enum ToolCall {
    ReadFile { path: String },
    WriteFile { path: String, content: String },
    RunCommand { cmd: String },
    SearchCode { pattern: String },
    ListDir { path: String },
}

#[derive(Debug)]
pub struct ToolResult {
    pub tool: String,
    pub success: bool,
    pub output: String,
}

pub async fn execute_plan(
    api: &ApiClient,
    state: &mut WorkflowState,
) -> Result<()> {
    let system = build_executor_system_prompt(&state.skills);

    loop {
        let mut messages: Vec<Message> = vec![Message {
            role: "system".into(),
            content: system.clone(),
        }];

        messages.push(Message {
            role: "user".into(),
            content: format!(
                "## Task\n{}\n\n## Plan\n{}\n\nExecute the next step of the plan. \
                 Use tools to read files, write code, and run commands. \
                 After each tool call, wait for the result before continuing. \
                 When the task is fully complete, respond with 'ALL_DONE'.",
                state.prompt, state.plan
            ),
        });

        // Include execution log for context
        if !state.execution_log.is_empty() {
            let recent: String = state.execution_log.iter().rev().take(5).rev().map(|s| s.as_str()).collect::<Vec<_>>().join("\n");
            messages.push(Message {
                role: "user".into(),
                content: format!("Recent execution log:\n{}", recent),
            });
        }

        let start = Instant::now();
        let response = api.chat(messages).await?;

        if response.contains("ALL_DONE") {
            println!("  {}  Execution complete.", "✓".green().bold());
            return Ok(());
        }

        if let Some(tool) = parse_tool_call(&response) {
            print!("  {}  {} ... ", "⚙".cyan().bold(), describe_tool(&tool));
            let result = execute_tool(&tool).await;
            let success = result.success;

            if success {
                println!("{}", "OK".green().bold());
            } else {
                println!("{}", "FAILED".red().bold());
                state.error_count += 1;
            }

            state.log(&format!(
                "[{:?}] {} → {} ({}ms)",
                tool,
                if success { "OK" } else { "FAIL" },
                result.output.chars().take(200).collect::<String>(),
                start.elapsed().as_millis()
            ));

            if !success {
                if state.error_count >= state.max_retries {
                    anyhow::bail!(
                        "Too many failures ({}). Last error: {}",
                        state.error_count,
                        result.output
                    );
                }
                println!(
                    "  {}  Auto-fixing... (attempt {}/{})",
                    "↻".yellow().bold(),
                    state.error_count,
                    state.max_retries
                );
                state.transition(Phase::SelfHealing);
                let fix = attempt_fix(api, state, &result).await?;
                state.transition(Phase::Executing);
                state.log(&format!("Auto-fix applied: {}", fix));
            }
        } else {
            state.log(&format!("Response (no tool): {}", response));
            println!("  {}  {}", "→".dimmed(), response.chars().take(200).collect::<String>().dimmed());
            return Ok(());
        }
    }
}

fn build_executor_system_prompt(skills: &[crate::skills::Skill]) -> String {
    let base = skills::build_system_prompt(skills, "");

    format!(
        "{}\n\n\
         You are executing a plan step by step. Use the following tools:\n\n\
         ## Tools\n\n\
         To call a tool, use this exact format:\n\
         ```\n\
         TOOL: read_file path=\"relative/path\"\n\
         TOOL: write_file path=\"relative/path\"\n\
         content here...\n\
         /TOOL\n\
         TOOL: run_command cmd=\"command to run\"\n\
         TOOL: search_code pattern=\"regex pattern\"\n\
         TOOL: list_dir path=\"relative/path\"\n\
         ```\n\n\
         - Call ONE tool at a time\n\
         - After receiving a tool result, use it to decide the next action\n\
         - When the entire task is complete, reply ONLY: ALL_DONE\n\
         - If a command fails, analyze the error and try to fix it\n\
         - IMPORTANT: actually create/modify files, don't just describe what to do\n\n\
         Respond with a tool call OR ALL_DONE. Nothing else.",
        base
    )
}

fn parse_tool_call(response: &str) -> Option<ToolCall> {
    let re_tool = Regex::new(r"TOOL:\s*(\w+)\s*(.*?)(?:\nTOOL:|/TOOL|$)").ok()?;

    for cap in re_tool.captures_iter(response) {
        let tool_name = cap.get(1)?.as_str().trim();
        let args = cap.get(2)?.as_str();

        match tool_name {
            "read_file" => {
                if let Some(path) = extract_arg(args, "path") {
                    return Some(ToolCall::ReadFile { path });
                }
            }
            "write_file" => {
                if let Some(path) = extract_arg(args, "path") {
                    // Content is everything after the first line
                    let content = args
                        .lines()
                        .skip(1)
                        .collect::<Vec<_>>()
                        .join("\n")
                        .trim()
                        .to_string();
                    return Some(ToolCall::WriteFile { path, content });
                }
            }
            "run_command" => {
                if let Some(cmd) = extract_arg(args, "cmd") {
                    return Some(ToolCall::RunCommand { cmd });
                }
            }
            "search_code" => {
                if let Some(pattern) = extract_arg(args, "pattern") {
                    return Some(ToolCall::SearchCode { pattern });
                }
            }
            "list_dir" => {
                let path = extract_arg(args, "path").unwrap_or_else(|| ".".into());
                return Some(ToolCall::ListDir { path });
            }
            _ => {}
        }
    }

    None
}

fn extract_arg(args: &str, key: &str) -> Option<String> {
    let pattern = format!(r#"{}\s*=\s*"([^"]*)""#, key);
    let re = Regex::new(&pattern).ok()?;
    re.captures(args)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
}

fn describe_tool(tool: &ToolCall) -> String {
    match tool {
        ToolCall::ReadFile { path } => format!("Reading {}", path),
        ToolCall::WriteFile { path, content } => {
            format!("Writing {} ({} bytes)", path, content.len())
        }
        ToolCall::RunCommand { cmd } => format!("Running: {}", cmd),
        ToolCall::SearchCode { pattern } => format!("Searching: {}", pattern),
        ToolCall::ListDir { path } => format!("Listing: {}", path),
    }
}

async fn execute_tool(tool: &ToolCall) -> ToolResult {
    match tool {
        ToolCall::ReadFile { path } => {
            match std::fs::read_to_string(path) {
                Ok(content) => ToolResult {
                    tool: "read_file".into(),
                    success: true,
                    output: format!("File: {}\n{}", path, content),
                },
                Err(e) => ToolResult {
                    tool: "read_file".into(),
                    success: false,
                    output: format!("Error reading {}: {}", path, e),
                },
            }
        }
        ToolCall::WriteFile { path, content } => {
            // Create parent directories if needed
            if let Some(parent) = std::path::Path::new(path).parent() {
                std::fs::create_dir_all(parent).ok();
            }
            match std::fs::write(path, content) {
                Ok(_) => ToolResult {
                    tool: "write_file".into(),
                    success: true,
                    output: format!("Written {} bytes to {}", content.len(), path),
                },
                Err(e) => ToolResult {
                    tool: "write_file".into(),
                    success: false,
                    output: format!("Error writing {}: {}", path, e),
                },
            }
        }
        ToolCall::RunCommand { cmd } => {
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output();

            match output {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    let combined = format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr);
                    ToolResult {
                        tool: "run_command".into(),
                        success: out.status.success(),
                        output: combined,
                    }
                }
                Err(e) => ToolResult {
                    tool: "run_command".into(),
                    success: false,
                    output: format!("Failed to execute: {}", e),
                },
            }
        }
        ToolCall::SearchCode { pattern } => {
            let output = Command::new("rg")
                .arg("-n")
                .arg("--no-heading")
                .arg("-i")
                .arg(pattern)
                .output();

            match output {
                Ok(out) => {
                    let text = String::from_utf8_lossy(&out.stdout);
                    let result = if text.is_empty() {
                        format!("No matches found for '{}'", pattern)
                    } else {
                        let lines: Vec<&str> = text.lines().take(30).collect();
                        lines.join("\n")
                    };
                    ToolResult {
                        tool: "search_code".into(),
                        success: true,
                        output: result,
                    }
                }
                Err(e) => ToolResult {
                    tool: "search_code".into(),
                    success: false,
                    output: format!("Search failed (install ripgrep?): {}", e),
                },
            }
        }
        ToolCall::ListDir { path } => {
            match std::fs::read_dir(path) {
                Ok(entries) => {
                    let mut out = String::new();
                    for entry in entries.filter_map(|e| e.ok()) {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let is_dir = entry.file_type().ok().map(|t| t.is_dir()).unwrap_or(false);
                        out.push_str(&format!("{}  {}\n", if is_dir { "📁" } else { "📄" }, name));
                    }
                    ToolResult {
                        tool: "list_dir".into(),
                        success: true,
                        output: out,
                    }
                }
                Err(e) => ToolResult {
                    tool: "list_dir".into(),
                    success: false,
                    output: format!("Error listing {}: {}", path, e),
                },
            }
        }
    }
}

async fn attempt_fix(
    api: &ApiClient,
    state: &WorkflowState,
    last_result: &ToolResult,
) -> Result<String> {
    let messages = vec![Message {
        role: "user".into(),
        content: format!(
            "A tool call failed while executing this task:\n\n\
             Task: {}\n\n\
             Failed tool: {}\n\
             Error output:\n{}\n\n\
             Analyze the error and provide a fix. \
             Respond with the corrected tool call to retry, or explain the fix needed. \
             Be specific - reference exact file paths and code changes.",
            state.prompt, last_result.tool, last_result.output
        ),
    }];

    api.chat(messages).await
}
