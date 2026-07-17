use crate::api::{ApiClient, Message};
use crate::skills;
use crate::state::WorkflowState;
use anyhow::Result;
use colored::*;
use regex::Regex;
use std::io::Write;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub async fn execute_plan(
    api: &ApiClient,
    state: &mut WorkflowState,
) -> Result<()> {
    let context = build_context();
    let system = build_system(&state.skills);

    println!();
    let spinner = spinner("Implementing");

    let messages = vec![
        Message { role: "system".into(), content: system },
        Message {
            role: "user".into(),
            content: format!(
                "## Task\n{}\n\n## Plan\n{}\n\n## Project Context\n{}\n\n\
                 Implement ALL changes. For each file output:\n\
                 ```lang:path/to/file\n<complete content>\n```\n\
                 Verification as: ```bash\ncmd\n```\nReply DONE when done.",
                state.prompt, state.plan, context
            ),
        },
    ];

    let implementation = api.chat(messages).await?;
    spinner.store(true, Ordering::Relaxed);
    tokio::time::sleep(Duration::from_millis(150)).await;

    let written = apply_file_blocks(state, &implementation)?;
    let verified = run_bash_blocks(state, &implementation)?;

    if written == 0 && verified == 0 {
        println!("  {}  No changes detected", "→".dimmed());
        apply_raw_response(state, &implementation)?;
    } else {
        println!("  {}  {} files, {} checks", "✓".green().bold(), written, verified);
    }

    state.log("Done");
    Ok(())
}

fn build_context() -> String {
    let mut ctx = String::new();
    if let Ok(cwd) = std::env::current_dir() {
        ctx.push_str(&format!("Working dir: {}\n\n", cwd.display()));
    }
    ctx.push_str("Project files:\n");
    if let Ok(entries) = std::fs::read_dir(".") {
        let mut items: Vec<String> = entries
            .flatten()
            .map(|e| {
                let n = e.file_name().to_string_lossy().to_string();
                let d = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
                format!("  {}{}", n, if d { "/" } else { "" })
            })
            .filter(|s| !s.contains("/.") && !s.contains("target") && !s.contains("node_modules"))
            .collect();
        items.sort();
        for item in items.iter().take(50) {
            ctx.push_str(&format!("{}\n", item));
        }
    }
    for f in &["Cargo.toml", "package.json", "go.mod", "pyproject.toml"] {
        if std::path::Path::new(f).exists() {
            if let Ok(content) = std::fs::read_to_string(f) {
                ctx.push_str(&format!("\n--- {} ---\n{}\n", f, content));
            }
        }
    }
    ctx
}

fn build_system(skills: &[crate::skills::Skill]) -> String {
    let base = skills::build_system_prompt(skills, "");
    format!(
        "{}\n\nWrite files as:\n```lang:path/to/file\n<complete content>\n```\nCommands as:\n```bash\ncmd\n```",
        base
    )
}

fn apply_file_blocks(state: &mut WorkflowState, response: &str) -> Result<usize> {
    let re = Regex::new(r"```(?:[a-zA-Z0-9_+#-]+(?::| filename=| ))?(.+?)[\r\n]+([\s\S]*?)```").unwrap();
    let mut count = 0;

    for cap in re.captures_iter(response) {
        let maybe_path = cap.get(1).unwrap().as_str().trim();
        let content = cap.get(2).unwrap().as_str();

        if maybe_path == "bash" || maybe_path == "sh" || maybe_path == "shell"
            || maybe_path == "zsh" || maybe_path == "console" || maybe_path == "terminal"
            || maybe_path.is_empty() || maybe_path == "text" || maybe_path == "plaintext"
            || maybe_path.starts_with("write_file:") || maybe_path.starts_with("TOOL:")
            || maybe_path.starts_with("read_file:") || maybe_path.starts_with("run_command:")
        {
            continue;
        }

        let is_path = maybe_path.contains('/') || maybe_path.contains(".rs")
            || maybe_path.contains(".ts") || maybe_path.contains(".js")
            || maybe_path.contains(".py") || maybe_path.contains(".go")
            || maybe_path.contains(".toml") || maybe_path.contains(".json")
            || maybe_path.contains(".yaml") || maybe_path.contains(".yml")
            || maybe_path.contains(".md") || maybe_path.contains(".html")
            || maybe_path.contains(".css") || maybe_path.contains(".sql")
            || maybe_path.contains(".sh") || maybe_path.contains(".txt");

        if !is_path || content.trim().is_empty() {
            continue;
        }

        if let Some(parent) = std::path::Path::new(maybe_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        match std::fs::write(maybe_path, content) {
            Ok(_) => {
                println!("  {}  {}", "✓".green().bold(), maybe_path.cyan());
                state.log(&format!("Wrote {}", maybe_path));
                count += 1;
            }
            Err(e) => println!("  {}  {}: {}", "✗".red(), maybe_path, e),
        }
    }
    Ok(count)
}

fn run_bash_blocks(state: &mut WorkflowState, response: &str) -> Result<usize> {
    let re = Regex::new(r"```(?:bash|sh|shell|zsh|console)\s*\n([\s\S]*?)```").unwrap();
    let mut count = 0;
    for cap in re.captures_iter(response) {
        let script = cap.get(1).unwrap().as_str().trim();
        for line in script.lines() {
            let cmd = line.trim();
            if cmd.is_empty() || cmd.starts_with('#') || cmd.starts_with("//") { continue; }
            let actual = cmd.strip_prefix("$ ").or(cmd.strip_prefix("> ")).unwrap_or(cmd);
            print!("  {}  {}", "▶".yellow().bold(), actual.dimmed());
            match Command::new("sh").arg("-c").arg(actual).output() {
                Ok(out) => {
                    let ok = out.status.success();
                    println!("  {}", if ok { "OK".green() } else { "FAILED".red() });
                    if !ok {
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        if !stderr.trim().is_empty() { println!("    {}", stderr.trim().red()); }
                    }
                    state.log(&format!("Ran `{}` → {}", actual, if ok { "OK" } else { "FAIL" }));
                }
                Err(e) => println!("  {}", format!("ERR: {}", e).red()),
            }
            count += 1;
        }
    }
    Ok(count)
}

fn apply_raw_response(state: &mut WorkflowState, response: &str) -> Result<()> {
    if !response.contains("```") && response.len() > 50 {
        let _ = std::fs::create_dir_all(".deepseek");
        std::fs::write(".deepseek/raw_response.txt", response)?;
        println!("  {}  Saved raw response", "📝".dimmed());
    }
    Ok(())
}

fn spinner(label: &str) -> Arc<AtomicBool> {
    let chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let done = Arc::new(AtomicBool::new(false));
    let d = done.clone();
    let l = label.to_string();
    std::thread::spawn(move || {
        let mut i = 0;
        let mut stderr = std::io::stderr();
        while !d.load(Ordering::Relaxed) {
            let _ = write!(stderr, "\r  {}  {}", chars[i % chars.len()].yellow(), l);
            let _ = stderr.flush();
            std::thread::sleep(Duration::from_millis(120));
            i += 1;
        }
        let _ = write!(stderr, "\r{}\r", " ".repeat(l.len() + 6));
        let _ = stderr.flush();
    });
    done
}
