use crate::api::{ApiClient, Message};
use crate::skills;
use crate::state::WorkflowState;
use anyhow::Result;
use colored::*;
use regex::Regex;
use std::io::Write;
use std::process::Command;

pub async fn execute_plan(
    api: &ApiClient,
    state: &mut WorkflowState,
) -> Result<()> {
    let context = build_context();
    let system = build_system(&state.skills);

    println!();
    eprint!("  {}  Working...", "●".yellow());
    let _ = std::io::stderr().flush();

    let messages = vec![
        Message { role: "system".into(), content: system },
        Message {
            role: "user".into(),
            content: format!(
                "# Task\n{}\n\n# Plan\n{}\n\n# Project\n{}\n\n\
                 Implement changes. Put each file in a code block with the file path:\n\n\
                 ```rust:src/main.rs\n// your actual code here\n```\n\n\
                 Put shell commands in bash blocks:\n\n\
                 ```bash\ncargo check\n```\n\n\
                 IMPORTANT: Write REAL code, not placeholders. \
                 Never include system prompts or example text. \
                 When all files are written, reply DONE.",
                state.prompt, state.plan, context
            ),
        },
    ];

    let implementation = api.chat(messages).await?;
    eprint!("\r{}\r", " ".repeat(30));
    let _ = std::io::stderr().flush();

    // Parse and apply file blocks
    let written = apply_file_blocks(state, &implementation)?;
    let verified = run_bash_blocks(state, &implementation)?;

    if written == 0 && verified == 0 {
        println!("  {}  No changes detected", "→".dimmed());
        // Save raw response for debugging but don't treat as error
        let _ = std::fs::create_dir_all(".deepseek");
        let _ = std::fs::write(".deepseek/raw_response.txt", &implementation);
    } else {
        println!(
            "  {}  {} files written, {} checks run",
            "✓".green().bold(),
            written,
            verified
        );
    }

    state.log("Implementation complete");

    if std::path::Path::new("Cargo.toml").exists() && written > 0 {
        print!("  {}  cargo check ", "▶".yellow());
        let _ = std::io::stdout().flush();
        match Command::new("cargo").args(["check", "--message-format=short"]).output() {
            Ok(out) => {
                if out.status.success() {
                    println!("{}", "OK".green());
                } else {
                    println!("{}  — reverting changes", "FAILED".red());
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    for l in stderr.lines().take(5) {
                        if !l.trim().is_empty() { println!("    {}", l.trim().red()); }
                    }
                    // Revert via git if available
                    if std::path::Path::new(".git").exists() {
                        let _ = Command::new("git").args(["checkout", "--", "."]).output();
                        let _ = Command::new("git").args(["clean", "-fd"]).output();
                        println!("  {}  Changes reverted via git", "↩".cyan());
                        state.log("Changes reverted — cargo check failed");
                    }
                }
            }
            Err(_) => {}
        }
    }

    Ok(())
}

fn build_context() -> String {
    let mut ctx = String::new();
    if let Ok(cwd) = std::env::current_dir() {
        ctx.push_str(&format!("Directory: {}\n", cwd.display()));
    }

    // File listing
    if let Ok(entries) = std::fs::read_dir(".") {
        let mut items: Vec<String> = entries
            .flatten()
            .map(|e| {
                let n = e.file_name().to_string_lossy().to_string();
                let d = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
                format!("{}{}", n, if d { "/" } else { "" })
            })
            .filter(|s| !s.starts_with('.') && !s.contains("target") && !s.contains("node_modules"))
            .collect();
        items.sort();
        ctx.push_str("Files:\n");
        for item in items.iter().take(60) {
            ctx.push_str(&format!("  {}\n", item));
        }
    }

    // Key config files - read and include
    for f in &["Cargo.toml", "package.json", "go.mod", "pyproject.toml", "Makefile"] {
        if std::path::Path::new(f).exists() {
            if let Ok(c) = std::fs::read_to_string(f) {
                ctx.push_str(&format!("\n--- {} ---\n{}", f, c));
            }
        }
    }

    ctx
}

fn build_system(skills: &[crate::skills::Skill]) -> String {
    let base = skills::build_system_prompt(skills, "");
    format!(
        "{}\n\n\
         You write code that gets saved directly to files. \
         Every code block with a file path becomes a real file.\n\
         Format: ```lang:path/to/file.ext\n\n\
         Rules: Write COMPLETE files. No '// ... rest of file'. No placeholder text.",
        base
    )
}

fn apply_file_blocks(state: &mut WorkflowState, response: &str) -> Result<usize> {
    let re = Regex::new(
        r"```(?:[a-zA-Z0-9_+#-]+(?::| filename=| ))?(\S+?)[\r\n]+([\s\S]*?)```"
    ).unwrap();

    let mut count = 0;
    for cap in re.captures_iter(response) {
        if count >= 5 {
            state.log("Hit file write limit (5). Remaining blocks ignored.");
            break;
        }
        let maybe_path = cap.get(1).unwrap().as_str().trim();
        let content = cap.get(2).unwrap().as_str();

        // Skip non-file blocks
        let skip = [
            "bash", "sh", "shell", "zsh", "console", "terminal",
            "text", "plaintext", "json", "yaml", "yml", "toml", "xml",
            "diff", "log", "output",
        ];
        if skip.contains(&maybe_path.to_lowercase().as_str())
            || maybe_path.is_empty()
            || maybe_path.starts_with("write_file:")
            || maybe_path.starts_with("TOOL:")
            || maybe_path.starts_with("read_file:")
            || maybe_path.starts_with("run_command:")
            || maybe_path.starts_with("search_code:")
            || maybe_path.starts_with("list_dir:")
            || maybe_path.starts_with('<')  // template placeholder
            || maybe_path.starts_with("//") // comment
        {
            continue;
        }

        // Must look like a file path
        let looks_like_path = maybe_path.contains('/')
            || maybe_path.ends_with(".rs")
            || maybe_path.ends_with(".ts")
            || maybe_path.ends_with(".js")
            || maybe_path.ends_with(".jsx")
            || maybe_path.ends_with(".tsx")
            || maybe_path.ends_with(".py")
            || maybe_path.ends_with(".go")
            || maybe_path.ends_with(".java")
            || maybe_path.ends_with(".rb")
            || maybe_path.ends_with(".toml")
            || maybe_path.ends_with(".json")
            || maybe_path.ends_with(".yaml")
            || maybe_path.ends_with(".yml")
            || maybe_path.ends_with(".md")
            || maybe_path.ends_with(".html")
            || maybe_path.ends_with(".css")
            || maybe_path.ends_with(".sql")
            || maybe_path.ends_with(".sh")
            || maybe_path.ends_with(".env")
            || maybe_path.ends_with(".txt");

        if !looks_like_path || content.trim().is_empty() {
            continue;
        }

        // Reject content that looks like system prompt / instructions
        let content_preview = content.trim().chars().take(100).collect::<String>().to_lowercase();
        if content_preview.contains("write real code")
            || content_preview.contains("complete content")
            || content_preview.contains("system prompt")
            || content_preview.contains("file content here")
        {
            continue;
        }

        if let Some(parent) = std::path::Path::new(maybe_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        match std::fs::write(maybe_path, content) {
            Ok(_) => {
                println!("  {}  {} ({} bytes)", "✓".green().bold(), maybe_path.cyan(), content.len());
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
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
                continue;
            }
            // Strip shell prompt prefixes
            let cmd = line.strip_prefix("$ ").or(line.strip_prefix("> ")).unwrap_or(line);

            if should_skip_command(cmd) {
                println!("  {}  {} (skipped — unsafe)", "⊘".yellow(), cmd.dimmed());
                continue;
            }

            print!("  {}  {}", "▶".yellow(), cmd.dimmed());
            let _ = std::io::stdout().flush();

            match Command::new("sh").arg("-c").arg(cmd).output() {
                Ok(out) => {
                    let ok = out.status.success();
                    if ok {
                        println!("  {}", "OK".green());
                    } else {
                        println!("  {}", "FAILED".red());
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        if !stderr.trim().is_empty() {
                            for l in stderr.lines().take(3) {
                                println!("    {}", l.trim().red());
                            }
                        }
                    }
                    state.log(&format!("Ran `{}` → {}", cmd, if ok { "OK" } else { "FAIL" }));
                }
                Err(e) => println!("  {}", format!("ERR: {}", e).red()),
            }
            count += 1;
        }
    }

    Ok(count)
}

fn should_skip_command(cmd: &str) -> bool {
    let lower = cmd.to_lowercase();
    if lower.contains("cargo test") && lower.contains("--test") {
        if let Some(name) = cmd.split_whitespace()
            .skip_while(|w| *w != "--test")
            .nth(1)
        {
            if !std::path::Path::new(&format!("tests/{}.rs", name)).exists() {
                return true;
            }
        }
    }
    if (lower.starts_with("npm ") || lower.starts_with("npx ") || lower.starts_with("node "))
        && !std::path::Path::new("package.json").exists()
    {
        return true;
    }
    if lower.starts_with("pip ") && !std::path::Path::new("requirements.txt").exists()
        && !std::path::Path::new("pyproject.toml").exists()
    {
        return true;
    }
    false
}
