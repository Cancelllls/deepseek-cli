use anyhow::{Context, Result};
use std::process::Command;

pub fn source_path() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

pub async fn rebuild_and_reinstall() -> Result<String> {
    let src = source_path();
    let mut output = String::new();

    output.push_str("  cargo build --release ... ");
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(src)
        .output()
        .context("failed to run cargo build")?;

    if !status.status.success() {
        let stderr = String::from_utf8_lossy(&status.stderr);
        output.push_str(&format!("\n  {}  BUILD FAILED:\n{}\n", "✗".red_term(), stderr));
        return Ok(output);
    }
    output.push_str("OK\n");

    output.push_str("  cargo check ... ");
    let status = Command::new("cargo")
        .args(["check"])
        .current_dir(src)
        .output()
        .context("failed to run cargo check")?;

    if !status.status.success() {
        let stderr = String::from_utf8_lossy(&status.stderr);
        output.push_str(&format!("\n  {}  CHECK FAILED:\n{}\n", "✗".red_term(), stderr));
        return Ok(output);
    }
    output.push_str("OK\n");

    output.push_str("  cargo install --path . ... ");
    let status = Command::new("cargo")
        .args(["install", "--path", "."])
        .current_dir(src)
        .output()
        .context("failed to run cargo install")?;

    if !status.status.success() {
        let stderr = String::from_utf8_lossy(&status.stderr);
        output.push_str(&format!("\n  {}  INSTALL FAILED:\n{}\n", "✗".red_term(), stderr));
        return Ok(output);
    }
    output.push_str("OK (binary at ~/.cargo/bin/deepseek-cli)\n");

    Ok(output)
}

pub fn run_diagnostics() -> String {
    let src = source_path();
    let mut out = String::new();

    // Clippy warnings
    out.push_str("  Running cargo clippy ...\n");
    if let Ok(status) = Command::new("cargo")
        .args([
            "clippy",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ])
        .current_dir(src)
        .output()
    {
        if status.status.success() {
            out.push_str("  ✓  No clippy warnings\n");
        } else {
            let stderr = String::from_utf8_lossy(&status.stderr);
            let lines: Vec<&str> = stderr
                .lines()
                .filter(|l| l.trim().starts_with("-->") || l.contains("warning:"))
                .take(15)
                .collect();
            if lines.is_empty() {
                out.push_str("  ✓  No issues detected\n");
            } else {
                out.push_str(&format!(
                    "  ⚠  {} warnings found:\n",
                    lines.len()
                ));
                for line in lines {
                    out.push_str(&format!("    {}\n", line.trim()));
                }
            }
        }
    } else {
        out.push_str("  ⚠  clippy not installed (rustup component add clippy)\n");
    }

    // Cargo fmt check
    out.push_str("\n  Checking formatting ...\n");
    if let Ok(status) = Command::new("cargo")
        .args(["fmt", "--all", "--", "--check"])
        .current_dir(src)
        .output()
    {
        if status.status.success() {
            out.push_str("  ✓  Code is formatted\n");
        } else {
            out.push_str("  ⚠  Formatting issues found (run /evolve fmt to fix)\n");
        }
    }

    // Outdated deps
    out.push_str("\n  Checking dependencies ...\n");
    if let Ok(status) = Command::new("cargo")
        .args(["outdated", "--depth", "1"])
        .current_dir(src)
        .output()
    {
        if status.status.success() {
            let stdout = String::from_utf8_lossy(&status.stdout);
            let lines: Vec<&str> = stdout.lines().filter(|l| !l.is_empty()).collect();
            if lines.len() <= 1 {
                out.push_str("  ✓  Dependencies are up to date\n");
            } else {
                out.push_str(&format!(
                    "  ⚠  {} outdated dependencies:\n",
                    lines.len().saturating_sub(1)
                ));
                for line in lines.iter().skip(1).take(10) {
                    out.push_str(&format!("    {}\n", line));
                }
            }
        }
    } else {
        out.push_str("  ⚠  cargo-outdated not installed (cargo install cargo-outdated)\n");
    }

    // Binary size
    if let Ok(meta) = std::fs::metadata(format!("{}/target/release/deepseek-cli", src)) {
        let mb = meta.len() as f64 / 1_048_576.0;
        out.push_str(&format!("\n  Binary size: {:.1} MB\n", mb));
        if mb > 50.0 {
            out.push_str("  ⚠  Binary is large (run /evolve strip to shrink)\n");
        }
    }

    out
}

pub fn auto_format() -> String {
    let src = source_path();
    match Command::new("cargo")
        .args(["fmt", "--all"])
        .current_dir(src)
        .output()
    {
        Ok(status) if status.status.success() => {
            "  ✓  Code formatted".to_string()
        }
        Ok(status) => {
            format!(
                "  ✗  Format failed: {}",
                String::from_utf8_lossy(&status.stderr)
            )
        }
        Err(e) => format!("  ✗  Failed to run cargo fmt: {}", e),
    }
}

pub fn auto_fix_clippy() -> String {
    let src = source_path();
    match Command::new("cargo")
        .args(["clippy", "--fix", "--allow-dirty", "--allow-staged"])
        .current_dir(src)
        .output()
    {
        Ok(status) => {
            let stderr = String::from_utf8_lossy(&status.stderr);
            format!(
                "  Clippy auto-fix ran. {}\n  {}",
                if status.status.success() {
                    "No fixable issues."
                } else {
                    "Some issues may remain."
                },
                stderr
                    .lines()
                    .filter(|l| l.contains("warning:"))
                    .take(5)
                    .collect::<Vec<_>>()
                    .join("\n  ")
            )
        }
        Err(e) => format!("  ✗  Failed to run clippy --fix: {}", e),
    }
}

pub fn strip_binary() -> String {
    let src = source_path();
    let path = format!("{}/target/release/deepseek-cli", src);
    match Command::new("strip").arg(&path).output() {
        Ok(status) if status.status.success() => {
            if let Ok(meta) = std::fs::metadata(&path) {
                format!(
                    "  ✓  Binary stripped (now {:.1} MB)",
                    meta.len() as f64 / 1_048_576.0
                )
            } else {
                "  ✓  Binary stripped".to_string()
            }
        }
        _ => {
            // strip might not be on the system, try cargo-strip
            match Command::new("cargo")
                .args(["strip", "--release"])
                .current_dir(src)
                .output()
            {
                Ok(s) if s.status.success() => "  ✓  Binary stripped (via cargo-strip)".to_string(),
                _ => "  ⚠  strip not available (install binutils or cargo-strip)".to_string(),
            }
        }
    }
}

pub fn self_improvement_prompt(user_request: &str) -> String {
    format!(
        "You are improving YOUR OWN source code at: {}\n\n\
         Source files:\n\
         - src/main.rs — CLI entry, REPL loop, state machine\n\
         - src/api.rs — DeepSeek API client with SSE streaming\n\
         - src/config.rs — Config loading\n\
         - src/state.rs — 8-phase WorkflowState machine\n\
         - src/planner.rs — Plan generation\n\
         - src/executor.rs — Tool execution + self-healing\n\
         - src/reviewer.rs — Review and optimization\n\
         - src/render.rs — Terminal rendering\n\
         - src/skills.rs — Skill struct + routing\n\
         - src/skills_data.rs — Auto-generated (47 skills)\n\
         - src/memory.rs — Memory, journaling, git\n\
         - src/evolve.rs — Self-improvement (this file)\n\
         - Cargo.toml — Dependencies\n\n\
         Task: {}\n\n\
         Rules:\n\
         1. Read relevant source files first\n\
         2. Make targeted, minimal changes\n\
         3. Keep everything working\n\
         4. Reply ALL_DONE when finished.\n\
         Make the changes now.",
        source_path(),
        user_request
    )
}

// Terminal-safe ANSI helpers (no colored crate dependency in this module)
trait TermColor {
    fn red_term(&self) -> String;
}
impl TermColor for &str {
    fn red_term(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", self)
    }
}
