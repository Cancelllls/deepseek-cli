use crate::api::{ApiClient, Message};
use crate::skills;
use crate::state::WorkflowState;
use anyhow::Result;
use colored::*;
use regex::Regex;
use std::io::Write;
use std::process::Command;

#[derive(Debug)]
enum Tool {
    ReadFile { path: String },
    WriteFile { path: String, content: String },
    Bash { cmd: String },
    ListDir { path: String },
    Search { pattern: String },
}

pub async fn execute_plan(
    api: &ApiClient,
    state: &mut WorkflowState,
) -> Result<()> {
    let system = build_system(&state.skills);
    let context = build_context();
    let mut conversation: Vec<Message> = vec![
        Message { role: "system".into(), content: system },
        Message {
            role: "user".into(),
            content: format!(
                "Task: {}\n\nPlan:\n{}\n\nProject:\n{}\n\nStart by reading files you need, then implement.",
                state.prompt, state.plan, context
            ),
        },
    ];

    let mut total_files = 0u32;
    let max_turns = 12;

    for turn in 1..=max_turns {
        eprint!("  {}  Turn {}/{}", "●".yellow(), turn, max_turns);
        let _ = std::io::stderr().flush();

        let response = api.chat(conversation.clone()).await?;
        eprint!("\r{}\r", " ".repeat(30));
        let _ = std::io::stderr().flush();

        conversation.push(Message { role: "assistant".into(), content: response.clone() });

        let tools = parse_tools(&response);

        if tools.is_empty() {
            if response.to_uppercase().contains("DONE") {
                break;
            }
            conversation.push(Message {
                role: "user".into(),
                content: "Use XML tool tags:\n<read_file path=\"file\" />\n<write_file path=\"file\">code</write_file>\n<bash>command</bash>\n<list_dir path=\".\" />".into(),
            });
            continue;
        }

        let mut results = String::new();
        for tool in &tools {
            if matches!(tool, Tool::WriteFile { .. }) { total_files += 1; }
            match execute_tool(tool).await {
                Ok(result) => results.push_str(&format!("\n--- {} RESULT ---\n{}\n", tool_name(tool), result)),
                Err(e) => results.push_str(&format!("\n--- {} ERROR ---\n{}\n", tool_name(tool), e)),
            }
        }

        if !results.is_empty() {
            conversation.push(Message {
                role: "user".into(),
                content: format!("Results:{}", results),
            });
        }
    }

    if total_files > 0 {
        println!("  {}  {} files in {} turns", "✓".green().bold(), total_files, max_turns);
    }
    state.log(&format!("{} files", total_files));
    Ok(())
}

fn build_system(skills: &[crate::skills::Skill]) -> String {
    let base = skills::build_system_prompt(skills, "");
    format!(
        "{}\n\nTOOLS (use XML tags):\n\
         <read_file path=\"file.rs\" />\n\
         <write_file path=\"file.rs\">\ncode here\n</write_file>\n\
         <bash>cargo check</bash>\n\
         <list_dir path=\".\" />\n\
         <search pattern=\"keyword\" />\n\n\
         Read first, then write. Write COMPLETE files.\n\
         Reply DONE when finished.",
        base
    )
}

fn build_context() -> String {
    let mut ctx = String::new();
    if let Ok(cwd) = std::env::current_dir() {
        ctx.push_str(&format!("Dir: {}\n", cwd.display()));
    }
    if let Ok(entries) = std::fs::read_dir(".") {
        let mut items: Vec<String> = entries.flatten()
            .map(|e| {
                let n = e.file_name().to_string_lossy().to_string();
                let d = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
                format!("{}{}", n, if d { "/" } else { "" })
            })
            .filter(|s| !s.starts_with('.') && !s.contains("target") && !s.contains("node_modules"))
            .collect();
        items.sort();
        for item in items.iter().take(40) {
            ctx.push_str(&format!("  {}\n", item));
        }
    }
    for f in &["Cargo.toml", "package.json", "go.mod", "pyproject.toml"] {
        if std::path::Path::new(f).exists() {
            if let Ok(c) = std::fs::read_to_string(f) {
                ctx.push_str(&format!("\n--- {} ---\n{}", f, c));
            }
        }
    }
    ctx
}

fn parse_tools(response: &str) -> Vec<Tool> {
    let mut tools = Vec::new();

    let write_re = Regex::new(r#"<write_file\s+path="(.+?)">\s*([\s\S]*?)</write_file>"#).unwrap();
    for cap in write_re.captures_iter(response) {
        let path = cap.get(1).unwrap().as_str().to_string();
        let content = cap.get(2).unwrap().as_str().to_string();
        if !content.trim().is_empty() {
            tools.push(Tool::WriteFile { path, content });
        }
    }

    let read_re = Regex::new(r#"<read_file\s+path="(.+?)"\s*/>"#).unwrap();
    for cap in read_re.captures_iter(response) {
        tools.push(Tool::ReadFile { path: cap.get(1).unwrap().as_str().to_string() });
    }

    let bash_re = Regex::new(r"<bash>\s*([\s\S]*?)</bash>").unwrap();
    for cap in bash_re.captures_iter(response) {
        let cmd = cap.get(1).unwrap().as_str().trim().to_string();
        if !cmd.is_empty() {
            tools.push(Tool::Bash { cmd });
        }
    }

    let list_re = Regex::new(r#"<list_dir\s+path="(.+?)"\s*/>"#).unwrap();
    for cap in list_re.captures_iter(response) {
        tools.push(Tool::ListDir { path: cap.get(1).unwrap().as_str().to_string() });
    }

    let search_re = Regex::new(r#"<search\s+pattern="(.+?)"\s*/>"#).unwrap();
    for cap in search_re.captures_iter(response) {
        tools.push(Tool::Search { pattern: cap.get(1).unwrap().as_str().to_string() });
    }

    tools
}

fn tool_name(tool: &Tool) -> &str {
    match tool {
        Tool::ReadFile { .. } => "read_file",
        Tool::WriteFile { .. } => "write_file",
        Tool::Bash { .. } => "bash",
        Tool::ListDir { .. } => "list_dir",
        Tool::Search { .. } => "search",
    }
}

async fn execute_tool(tool: &Tool) -> Result<String> {
    match tool {
        Tool::ReadFile { path } => {
            match std::fs::read_to_string(path) {
                Ok(content) => {
                    println!("  {}  {}", "📖".dimmed(), path.cyan());
                    Ok(format!("{} ({}b):\n{}", path, content.len(), content))
                }
                Err(e) => {
                    println!("  {}  {} ({})", "✗".red(), path, e);
                    Ok(format!("Error: {}\nList directory to find correct path.", e))
                }
            }
        }
        Tool::WriteFile { path, content } => {
            if let Some(parent) = std::path::Path::new(path).parent() {
                std::fs::create_dir_all(parent).ok();
            }
            std::fs::write(path, content)?;
            println!("  {}  {} ({}b)", "✓".green().bold(), path.cyan(), content.len());
            Ok(format!("Written {}b to {}", content.len(), path))
        }
        Tool::Bash { cmd } => {
            print!("  {}  {}", "▶".yellow(), cmd.dimmed());
            let _ = std::io::stdout().flush();
            let output = Command::new("sh").arg("-c").arg(cmd).output()?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let ok = output.status.success();
            println!("  {}", if ok { "OK".green() } else { "FAILED".red() });
            if !ok && !stderr.trim().is_empty() {
                for l in stderr.lines().take(3) {
                    println!("    {}", l.trim().red());
                }
            }
            Ok(format!("EXIT: {}\nSTDOUT:\n{}\nSTDERR:\n{}",
                output.status.code().unwrap_or(-1), stdout, stderr))
        }
        Tool::ListDir { path } => {
            let entries = std::fs::read_dir(path)?;
            let mut items: Vec<String> = entries.flatten()
                .map(|e| {
                    let n = e.file_name().to_string_lossy().to_string();
                    let d = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
                    format!("{}{}", n, if d { "/" } else { "" })
                })
                .filter(|s| !s.starts_with('.'))
                .collect();
            items.sort();
            println!("  {}  {} ({} items)", "📁".dimmed(), path, items.len());
            Ok(format!("{}:\n{}", path, items.join("\n")))
        }
        Tool::Search { pattern } => {
            match Command::new("rg").arg("-n").arg("-i").arg("--no-heading").arg(pattern).output() {
                Ok(out) => {
                    let text = String::from_utf8_lossy(&out.stdout);
                    let lines: Vec<&str> = text.lines().take(20).collect();
                    println!("  {}  Search '{}' ({} results)", "🔍".dimmed(), pattern, lines.len());
                    Ok(if lines.is_empty() { format!("No matches for '{}'", pattern) } else { lines.join("\n") })
                }
                Err(e) => Ok(format!("Search failed: {}", e)),
            }
        }
    }
}
