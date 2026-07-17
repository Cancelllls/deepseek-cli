use crate::api;
use crate::api::{ApiClient, Message};
use crate::skills;
use crate::state::WorkflowState;
use anyhow::Result;
use futures::StreamExt;
use std::io::{self, Write};
use std::time::Instant;

pub async fn generate_plan(
    api: &ApiClient,
    state: &mut WorkflowState,
    context: &str,
) -> Result<String> {
    let mut base = skills::build_system_prompt(&state.skills, &gather_project_context());
    if !context.is_empty() {
        base.push_str("\n\n## Historical Context\n\n");
        base.push_str(context);
    }

    let mut messages: Vec<Message> = vec![Message {
        role: "system".into(),
        content: format!(
            "{}\n\n\
             Available tools: read_file, write_file, run_command, search_code, list_dir.\n\
             You will execute the plan yourself using these tools. Be specific about which files.",
            base
        ),
    }];

    messages.push(Message {
        role: "user".into(),
        content: format!(
            "Task: {}\n\n\
             Write a numbered implementation plan. For each step specify:\n\
             - What code to write/modify and where\n\
             - What command to run to verify\n\
             Keep it brief. Output ONLY the plan.",
            state.prompt
        ),
    });

    let mut stream = Box::pin(api.stream_chat(messages));
    let mut plan = String::new();
    let mut last_print = Instant::now();

    print!("  ");
    let _ = io::stdout().flush();

    while let Some(event) = stream.next().await {
        match event {
            api::StreamEvent::Content(text) => {
                plan.push_str(&text);
                print!("{}", text);
                if last_print.elapsed().as_millis() > 50 {
                    let _ = io::stdout().flush();
                    last_print = Instant::now();
                }
            }
            api::StreamEvent::Done => break,
            api::StreamEvent::Error(e) => anyhow::bail!(e),
            _ => {}
        }
    }
    let _ = io::stdout().flush();

    state.plan = plan.clone();
    state.add_message("assistant", &plan);

    Ok(plan)
}

pub fn gather_project_context() -> String {
    let mut ctx = String::new();

    // Working directory
    if let Ok(cwd) = std::env::current_dir() {
        ctx.push_str(&format!("Working directory: {}\n", cwd.display()));
    }

    // Top-level listing
    ctx.push_str("\nProject structure:\n");
    if let Ok(entries) = std::fs::read_dir(".") {
        let mut items: Vec<String> = Vec::new();
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') || name == "target" {
                continue;
            }
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                items.push(format!("{}/", name));
            } else {
                items.push(name);
            }
        }
        items.sort();
        for item in items.iter().take(40) {
            ctx.push_str(&format!("  {}\n", item));
        }
        if items.len() > 40 {
            ctx.push_str(&format!("  ... and {} more\n", items.len() - 40));
        }
    }

    // Key files
    let key_files = [
        "Cargo.toml",
        "package.json",
        "go.mod",
        "requirements.txt",
        "pyproject.toml",
        "Makefile",
        "Dockerfile",
        "tsconfig.json",
    ];
    ctx.push_str("\nKey files found:\n");
    for f in &key_files {
        if std::path::Path::new(f).exists() {
            let content = std::fs::read_to_string(f).unwrap_or_default();
            let summary: String = content.lines().take(5).collect::<Vec<_>>().join("\n");
            ctx.push_str(&format!("\n--- {} ---\n{}\n", f, summary));
        }
    }

    ctx
}
