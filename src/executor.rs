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
    let mut consecutive_no_tool = 0u32;
    let mut conversation: Vec<Message> = Vec::new();

    loop {
        let mut messages: Vec<Message> = vec![Message {
            role: "system".into(),
            content: system.clone(),
        }];

        messages.push(Message {
            role: "user".into(),
            content: format!(
                "## Task\n{}\n\n## Plan\n{}\n\n\
                 Execute ONE step. Use a TOOL: call. Reply ALL_DONE when complete.",
                state.prompt, state.plan
            ),
        });

        // Include recent conversation context
        for msg in conversation.iter().rev().take(4) {
            messages.push(msg.clone());
        }

        if consecutive_no_tool > 0 {
            messages.push(Message {
                role: "user".into(),
                content: "You MUST use a TOOL: call right now. Do not describe — act.".into(),
            });
        }

        let start = Instant::now();
        let response = api.chat(messages).await?;

        if response.contains("ALL_DONE") {
            println!("  {}  Execution complete.", "✓".green().bold());
            return Ok(());
        }

        if let Some(tool) = parse_tool_call(&response) {
            consecutive_no_tool = 0;
            conversation.push(Message {
                role: "assistant".into(),
                content: response.clone(),
            });

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
                "[{:?}] → {} ({}ms)\n{}",
                tool,
                if success { "OK" } else { "FAIL" },
                start.elapsed().as_millis(),
                result.output.chars().take(300).collect::<String>()
            ));

            conversation.push(Message {
                role: "user".into(),
                content: format!(
                    "Result of tool {}: {}",
                    if success { "OK" } else { "FAILED" },
                    result.output
                ),
            });

            if !success {
                if state.error_count >= state.max_retries {
                    anyhow::bail!(
                        "Too many failures ({}). Last: {}",
                        state.error_count,
                        result.output.chars().take(200).collect::<String>()
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
                state.log(&format!("Auto-fix: {}", fix));
            }
        } else {
            consecutive_no_tool += 1;
            conversation.push(Message {
                role: "assistant".into(),
                content: response.clone(),
            });
            state.log(&format!(
                "Response (no tool, attempt {}): {}",
                consecutive_no_tool,
                response.chars().take(150).collect::<String>()
            ));
            println!(
                "  {}  No tool in response (retry {}/3)",
                "→".dimmed(),
                consecutive_no_tool
            );

            if consecutive_no_tool >= 3 {
                anyhow::bail!(
                    "Model stopped producing tool calls after 3 attempts. Last response: {}",
                    response.chars().take(200).collect::<String>()
                );
            }
            // Loop continues — model gets another chance with stronger nudge
        }
    }
}

fn build_executor_system_prompt(skills: &[crate::skills::Skill]) -> String {
    let base = skills::build_system_prompt(skills, "");
    let cwd = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "unknown".into());

    format!(
        "{}\n\n\
         You are in: {}\n\n\
         ## Tools (use EXACTLY this format — one tool per response, no explanation):\n\n\
         TOOL: read_file path=\"relative/path\"\n\
         TOOL: write_file path=\"relative/path\"\n\
         <file content on next lines>\n\
         TOOL: run_command cmd=\"shell command\"\n\
         TOOL: search_code pattern=\"regex\"\n\
         TOOL: list_dir path=\"relative/path\"\n\n\
         RULES:\n\
         - ONE tool per response. NOTHING else — no description, no greeting.\n\
         - For write_file: first line is TOOL, rest is file content.\n\
         - Reply ALL_DONE only when all steps complete.\n\
         - If a file read fails, check the error output — it lists actual files.\n\
         - Do not describe tools — USE them immediately.",
        base, cwd
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
            let full = std::fs::canonicalize(path).unwrap_or_else(|_| std::path::PathBuf::from(path));
            match std::fs::read_to_string(path) {
                Ok(content) => ToolResult {
                    tool: "read_file".into(),
                    success: true,
                    output: format!("Contents of {} ({} bytes):\n{}", full.display(), content.len(), content),
                },
                Err(e) => {
                    // Try to help the model by listing what's actually there
                    let dir = std::path::Path::new(path).parent().unwrap_or(std::path::Path::new("."));
                    let mut hint = format!("Error: {}\n", e);
                    if let Ok(entries) = std::fs::read_dir(dir) {
                        hint.push_str(&format!("Files in {}:\n", dir.display()));
                        for entry in entries.flatten() {
                            let name = entry.file_name().to_string_lossy().to_string();
                            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                            hint.push_str(&format!("  {}{}\n", name, if is_dir { "/" } else { "" }));
                        }
                    }
                    ToolResult {
                        tool: "read_file".into(),
                        success: false,
                        output: hint,
                    }
                }
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
                    let mut out = format!("Contents of {}:\n", path);
                    let mut items: Vec<String> = Vec::new();
                    for entry in entries.flatten() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if name.starts_with('.') { continue; }
                        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                        items.push(format!("  {}{}", name, if is_dir { "/" } else { "" }));
                    }
                    items.sort();
                    out.push_str(&items.join("\n"));
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
