use anyhow::{Context, Result};
use chrono::Local;
use std::path::PathBuf;
use std::process::Command;

pub fn deepseek_dir() -> Result<PathBuf> {
    let dir = PathBuf::from(".deepseek");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn journal_dir() -> Result<PathBuf> {
    let dir = deepseek_dir()?.join("journal");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn memory_path() -> Result<PathBuf> {
    Ok(deepseek_dir()?.join("MEMORY.md"))
}

pub fn load_memory() -> String {
    let path = memory_path().unwrap_or_default();
    if path.exists() {
        std::fs::read_to_string(&path).unwrap_or_default()
    } else {
        String::new()
    }
}

#[allow(dead_code)]
pub fn save_memory(content: &str) -> Result<()> {
    let path = memory_path()?;
    std::fs::write(&path, content)?;
    Ok(())
}

pub fn append_memory(entry: &str) -> Result<()> {
    let path = memory_path()?;
    let existing = load_memory();
    let updated = if existing.is_empty() {
        format!("# DeepSeek CLI — Project Memory\n\n{}", entry)
    } else {
        format!("{}\n\n{}", existing, entry)
    };
    std::fs::write(&path, updated)?;
    Ok(())
}

pub fn save_journal(prompt: &str, plan: &str, log: &[String], suggestions: &str) -> Result<String> {
    let now = Local::now();
    let filename = now.format("%Y%m%d-%H%M%S.md").to_string();
    let path = journal_dir()?.join(&filename);

    let content = format!(
        "# Session: {}\n\n\
         ## Prompt\n\n{}\n\n\
         ## Plan\n\n{}\n\n\
         ## Execution Log\n\n{}\n\n\
         ## Suggestions\n\n{}\n",
        now.format("%Y-%m-%d %H:%M:%S"),
        prompt,
        plan,
        log.join("\n"),
        suggestions
    );

    std::fs::write(&path, &content)?;
    Ok(filename)
}

pub fn load_journals() -> Vec<String> {
    let dir = journal_dir().unwrap_or_default();
    let mut entries: Vec<String> = Vec::new();

    if let Ok(entries_iter) = std::fs::read_dir(&dir) {
        for entry in entries_iter.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "md") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let summary: String = content.lines().take(8).collect::<Vec<_>>().join("\n");
                    entries.push(summary);
                }
            }
        }
    }

    entries.sort();
    entries.reverse();
    entries.truncate(5);
    entries
}

pub fn git_context() -> String {
    let mut ctx = String::new();

    // Recent git log
    if let Ok(output) = Command::new("git")
        .args(["log", "--oneline", "-n", "10"])
        .output()
    {
        if output.status.success() {
            let log = String::from_utf8_lossy(&output.stdout);
            if !log.trim().is_empty() {
                ctx.push_str("## Recent Git History\n\n```\n");
                ctx.push_str(&log);
                ctx.push_str("```\n\n");
            }
        }
    }

    // Current branch
    if let Ok(output) = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
    {
        if output.status.success() {
            let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !branch.is_empty() {
                ctx.push_str(&format!("Current branch: `{}`\n", branch));
            }
        }
    }

    // Uncommitted changes
    if let Ok(output) = Command::new("git").args(["status", "--short"]).output() {
        if output.status.success() {
            let status = String::from_utf8_lossy(&output.stdout);
            if !status.trim().is_empty() {
                ctx.push_str("\n## Uncommitted Changes\n\n```\n");
                ctx.push_str(&status);
                ctx.push_str("```\n\n");
            }
        }
    }

    ctx
}

pub fn git_commit(message: &str) -> Result<()> {
    // Check if there are changes to commit
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .context("failed to run git status")?;

    if String::from_utf8_lossy(&output.stdout).trim().is_empty() {
        return Ok(());
    }

    let output = Command::new("git")
        .args(["add", "-A"])
        .output()
        .context("failed to run git add")?;

    if !output.status.success() {
        anyhow::bail!(
            "git add failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()
        .context("failed to run git commit")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("nothing to commit") {
            return Ok(());
        }
        anyhow::bail!("git commit failed: {}", stderr);
    }

    Ok(())
}

pub fn full_context() -> String {
    let mut ctx = String::new();

    let memory = load_memory();
    if !memory.is_empty() {
        ctx.push_str("## Project Memory\n\n");
        ctx.push_str(&memory);
        ctx.push_str("\n\n");
    }

    let journals = load_journals();
    if !journals.is_empty() {
        ctx.push_str("## Recent Sessions\n\n");
        for (i, j) in journals.iter().enumerate() {
            ctx.push_str(&format!("### Session {}\n{}\n\n", i + 1, j));
        }
    }

    ctx.push_str(&git_context());

    ctx
}

pub fn update_memory_after_run(prompt: &str, plan_summary: &str, suggestions: &str) -> Result<()> {
    let now = Local::now();
    let entry = format!(
        "## {} — {}\n\n\
         **What was asked:** {}\n\n\
         **What was done:** {}\n\n\
         **Improvements suggested:** {}\n",
        now.format("%Y-%m-%d %H:%M"),
        prompt.lines().next().unwrap_or("No prompt"),
        prompt,
        plan_summary.lines().take(3).collect::<Vec<_>>().join("; "),
        suggestions.lines().take(2).collect::<Vec<_>>().join("; ")
    );
    append_memory(&entry)
}
