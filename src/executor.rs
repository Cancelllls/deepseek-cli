use crate::api::{ApiClient, Message};
use crate::skills;
use crate::state::{Phase, WorkflowState};
use anyhow::Result;
use colored::*;
use regex::Regex;
use std::process::Command;
use std::time::Instant;

#[derive(Debug)]
enum ToolCall {
    ReadFile { path: String },
    RunCommand { cmd: String },
    SearchCode { pattern: String },
    ListDir { path: String },
}

struct ToolResult {
    success: bool,
    output: String,
}

pub async fn execute_plan(
    api: &ApiClient,
    state: &mut WorkflowState,
) -> Result<()> {
    let system = build_executor_system_prompt(&state.skills);
    let mut context = String::new();

    for turn in 0..8 {
        let messages = vec![
            Message { role: "system".into(), content: system.clone() },
            Message {
                role: "user".into(),
                content: format!(
                    "Task: {}\n\nPlan:\n{}\n\nContext gathered:\n{}\n\n\
                     Read files or run commands. Reply READY when done reading.",
                    state.prompt, state.plan, context
                ),
            },
        ];

        let start = Instant::now();
        let response = api.chat(messages).await?;

        if response.to_uppercase().contains("READY") {
            break;
        }

        if let Some(tool) = parse_tool(&response) {
            print!("  {}  {} ... ", "⚙".cyan().bold(), describe_tool(&tool));
            let result = execute_tool(&tool).await;
            println!("{}", if result.success { "OK" } else { "FAILED" });
            state.log(&format!("[{:?}] ({}ms)", tool, start.elapsed().as_millis()));
            context.push_str(&format!("\n--- {} ---\n{}\n", describe_tool(&tool), result.output));
        }
    }

    println!("\n  {}  Writing files...\n", "🔨".yellow().bold());
    let impl_messages = vec![
        Message { role: "system".into(), content: system.clone() },
        Message {
            role: "user".into(),
            content: format!(
                "Task: {}\n\nPlan:\n{}\n\nContext:\n{}\n\n\
                 Write ALL changed files using this format:\n\
                 ===FILE path/to/file.ext\n\
                 <complete file content>\n\
                 ===END\n\
                 Put ===RUN commands for tests. Output DONE when finished.",
                state.prompt, state.plan, context
            ),
        },
    ];

    let implementation = api.chat(impl_messages).await?;
    apply_implementation(state, &implementation)?;
    run_verification(state, &implementation)?;
    state.log("Done");
    Ok(())
}

fn build_executor_system_prompt(skills: &[crate::skills::Skill]) -> String {
    let base = skills::build_system_prompt(skills, "");
    let cwd = std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default();
    format!(
        "{}\n\nWorking dir: {}\n\nTools: TOOL: read_file | run_command | search_code | list_dir path=\"path\"\nWrite files with: ===FILE path ===END",
        base, cwd
    )
}

fn parse_tool(response: &str) -> Option<ToolCall> {
    let re = Regex::new(r"TOOL:\s*(\w+)\s*(.*)").ok()?;
    let cap = re.captures(response)?;
    let name = cap.get(1)?.as_str().trim();
    let args = cap.get(2)?.as_str();

    match name {
        "read_file" => extract_quoted(args, "path").map(|path| ToolCall::ReadFile { path }),
        "run_command" => extract_quoted(args, "cmd").map(|cmd| ToolCall::RunCommand { cmd }),
        "search_code" => extract_quoted(args, "pattern").map(|pattern| ToolCall::SearchCode { pattern }),
        "list_dir" => {
            let path = extract_quoted(args, "path").unwrap_or_else(|| ".".into());
            Some(ToolCall::ListDir { path })
        }
        _ => None,
    }
}

fn extract_quoted(args: &str, key: &str) -> Option<String> {
    let re = Regex::new(&format!(r#"{}\s*=\s*"([^"]*)""#, key)).ok()?;
    re.captures(args)?.get(1).map(|m| m.as_str().to_string())
}

fn describe_tool(tool: &ToolCall) -> String {
    match tool {
        ToolCall::ReadFile { path } => format!("Reading {}", path),
        ToolCall::RunCommand { cmd } => format!("Running: {}", cmd),
        ToolCall::SearchCode { pattern } => format!("Searching: {}", pattern),
        ToolCall::ListDir { path } => format!("Listing: {}", path),
    }
}

async fn execute_tool(tool: &ToolCall) -> ToolResult {
    match tool {
        ToolCall::ReadFile { path } => {
            match std::fs::read_to_string(path) {
                Ok(content) => ToolResult { success: true, output: format!("{} ({} bytes):\n{}", path, content.len(), content) },
                Err(e) => {
                    let dir = std::path::Path::new(path).parent().unwrap_or(".".as_ref());
                    let mut hint = format!("Error: {}\nFiles here:\n", e);
                    if let Ok(entries) = std::fs::read_dir(dir) {
                        for entry in entries.flatten() {
                            let n = entry.file_name().to_string_lossy().to_string();
                            if !n.starts_with('.') { hint.push_str(&format!("  {}\n", n)); }
                        }
                    }
                    ToolResult { success: false, output: hint }
                }
            }
        }
        ToolCall::RunCommand { cmd } => {
            match Command::new("sh").arg("-c").arg(cmd).output() {
                Ok(out) => ToolResult {
                    success: out.status.success(),
                    output: format!("{}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)),
                },
                Err(e) => ToolResult { success: false, output: format!("Error: {}", e) },
            }
        }
        ToolCall::SearchCode { pattern } => {
            match Command::new("rg").arg("-n").arg("-i").arg("--no-heading").arg(pattern).output() {
                Ok(out) => {
                    let text = String::from_utf8_lossy(&out.stdout);
                    let lines: Vec<&str> = text.lines().take(30).collect();
                    ToolResult { success: true, output: if lines.is_empty() { format!("No matches for '{}'", pattern) } else { lines.join("\n") } }
                }
                Err(e) => ToolResult { success: false, output: format!("Search failed: {}", e) },
            }
        }
        ToolCall::ListDir { path } => {
            match std::fs::read_dir(path) {
                Ok(entries) => {
                    let mut items: Vec<String> = entries.flatten()
                        .map(|e| {
                            let n = e.file_name().to_string_lossy().to_string();
                            let d = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
                            format!("  {}{}", n, if d { "/" } else { "" })
                        })
                        .filter(|s| !s.contains("/."))
                        .collect();
                    items.sort();
                    ToolResult { success: true, output: format!("{} contents:\n{}", path, items.join("\n")) }
                }
                Err(e) => ToolResult { success: false, output: format!("Error: {}", e) },
            }
        }
    }
}

fn apply_implementation(state: &mut WorkflowState, response: &str) -> Result<()> {
    let file_re = Regex::new(r"===FILE\s+(.+?)\n((?s).*?)===END").unwrap();
    let mut count = 0;

    for cap in file_re.captures_iter(response) {
        let path = cap.get(1).unwrap().as_str().trim();
        let content = cap.get(2).unwrap().as_str().trim();
        if content.is_empty() { continue; }
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        println!("  {}  {}", "✓".green().bold(), path.cyan());
        state.log(&format!("Wrote {} ({} bytes)", path, content.len()));
        count += 1;
    }

    if count == 0 {
        println!("  {}  No ===FILE blocks found", "⚠".yellow());
    }
    Ok(())
}

fn run_verification(state: &mut WorkflowState, response: &str) -> Result<()> {
    let run_re = Regex::new(r"===RUN\s+(.+)").unwrap();
    for cap in run_re.captures_iter(response) {
        let cmd = cap.get(1).unwrap().as_str().trim();
        print!("  {}  {}", "▶".yellow().bold(), cmd.dimmed());
        match Command::new("sh").arg("-c").arg(cmd).output() {
            Ok(out) => {
                let ok = out.status.success();
                println!("  {}", if ok { "OK".green() } else { "FAILED".red() });
                state.log(&format!("===RUN {} → {}", cmd, if ok { "OK" } else { "FAIL" }));
            }
            Err(e) => println!("  {}", format!("ERR: {}", e).red()),
        }
    }
    Ok(())
}
