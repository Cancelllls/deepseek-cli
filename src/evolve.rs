use anyhow::{Context, Result};
use std::process::Command;

pub fn source_path() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

#[allow(dead_code)]
pub fn is_self_improving() -> bool {
    std::env::var("DEEPSEEK_SELF_IMPROVE").is_ok()
}

#[allow(dead_code)]
pub fn set_self_improving() {
    std::env::set_var("DEEPSEEK_SELF_IMPROVE", "1");
}

pub async fn rebuild_and_reinstall() -> Result<String> {
    let src = source_path();
    let mut output = String::new();

    // Build
    output.push_str("## Build\n\n");
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(src)
        .output()
        .context("failed to run cargo build")?;

    output.push_str(&String::from_utf8_lossy(&status.stdout));
    if !status.status.success() {
        output.push_str("\nBUILD FAILED:\n");
        output.push_str(&String::from_utf8_lossy(&status.stderr));
        return Ok(output);
    }
    output.push_str("Build: OK\n\n");

    // Run cargo test (quick check)
    let status = Command::new("cargo")
        .args(["check"])
        .current_dir(src)
        .output()
        .context("failed to run cargo check")?;

    if !status.status.success() {
        output.push_str("\nCHECK FAILED:\n");
        output.push_str(&String::from_utf8_lossy(&status.stderr));
        return Ok(output);
    }
    output.push_str("Check: OK\n\n");

    // Install
    let status = Command::new("cargo")
        .args(["install", "--path", "."])
        .current_dir(src)
        .output()
        .context("failed to run cargo install")?;

    if !status.status.success() {
        output.push_str("\nINSTALL FAILED:\n");
        output.push_str(&String::from_utf8_lossy(&status.stderr));
        return Ok(output);
    }
    output.push_str("Install: OK — binary updated at ~/.cargo/bin/deepseek-cli\n");

    Ok(output)
}

pub fn self_improvement_prompt(user_request: &str) -> String {
    format!(
        "You are now improving YOUR OWN source code. Your source code lives at: {}\n\n\
         Source files:\n\
         - src/main.rs — CLI entry, REPL loop, state machine orchestration\n\
         - src/api.rs — DeepSeek API client with SSE streaming\n\
         - src/config.rs — Config loading from env/file\n\
         - src/state.rs — 8-phase WorkflowState machine\n\
         - src/planner.rs — Implementation plan generation\n\
         - src/executor.rs — Tool execution loop with self-healing\n\
         - src/reviewer.rs — Post-execution review and optimization\n\
         - src/render.rs — Terminal rendering\n\
         - src/skills.rs — Skill struct and routing engine\n\
         - src/skills_data.rs — Auto-generated from 47 ag-kit skills\n\
         - src/memory.rs — Persistent memory, journaling, git auto-commit\n\
         - src/evolve.rs — Self-improvement system (you are here)\n\
         - Cargo.toml — Dependencies\n\n\
         Task: Improve yourself by: {}\n\n\
         Rules:\n\
         1. Read the relevant source files first\n\
         2. Make targeted, minimal changes\n\
         3. Keep existing functionality working\n\
         4. After changes, the system will automatically rebuild and reinstall\n\
         5. If the build fails, you get one chance to fix it\n\n\
         Respond with the tool calls to make changes. When done, say ALL_DONE.",
        source_path(),
        user_request
    )
}
