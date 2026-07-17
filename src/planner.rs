use crate::api;
use crate::api::{ApiClient, Message};
use crate::skills;
use crate::state::WorkflowState;
use anyhow::Result;
use futures::StreamExt;
use std::io::{self, Write};

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

    let mut messages: Vec<Message> = vec![
        Message {
            role: "system".into(),
            content: base,
        },
    ];

    messages.push(Message {
        role: "user".into(),
        content: format!(
            "Generate a detailed implementation plan for the following task. \
             Break it into numbered, sequential steps. For each step, specify:\n\
             - What needs to be done\n\
             - Which files will be created or modified\n\
             - Any shell commands to run (e.g., tests, lints)\n\
             - Expected outcome of the step\n\n\
             Task: {}\n\n\
             Output ONLY the plan. No code yet, just the plan.",
            state.prompt
        ),
    });

#[allow(unused_imports)]
use futures::StreamExt;
    let mut stream = Box::pin(api.stream_chat(messages));
    let mut plan = String::new();
    let mut stderr = io::stderr();

    print!("  ");
    let _ = io::stdout().flush();

    while let Some(event) = stream.next().await {
        match event {
            api::StreamEvent::Content(text) => {
                plan.push_str(&text);
                // Stream visible output
                if text.contains('\n') {
                    let lines: Vec<&str> = text.split('\n').collect();
                    for (i, chunk) in lines.iter().enumerate() {
                        if i > 0 {
                            eprint!("\n  ");
                        }
                        eprint!("{}", chunk);
                    }
                } else {
                    eprint!("{}", text);
                }
                let _ = stderr.flush();
            }
            api::StreamEvent::Reasoning(text) => {
                let _ = stderr.write_all(format!("\x1b[2m{}\x1b[0m", text).as_bytes());
            }
            api::StreamEvent::Done => break,
            api::StreamEvent::Error(e) => {
                eprintln!();
                anyhow::bail!(e);
            }
        }
    }
    eprintln!();

    state.plan = plan.clone();
    state.add_message("assistant", &plan);

    Ok(plan)
}

pub fn gather_project_context() -> String {
    let mut ctx = String::new();

    // Detect language/framework from project files
    let entries = [
        ("package.json", "Node.js/TypeScript project"),
        ("Cargo.toml", "Rust project"),
        ("go.mod", "Go project"),
        ("requirements.txt", "Python project"),
        ("pyproject.toml", "Python project"),
        ("Gemfile", "Ruby project"),
        ("tsconfig.json", "TypeScript project"),
        ("next.config", "Next.js project"),
        ("Dockerfile", "Dockerized project"),
        ("docker-compose.yml", "Docker Compose project"),
        ("Makefile", "Project with Makefile"),
    ];

    for (file, desc) in &entries {
        if std::path::Path::new(file).exists() {
            ctx.push_str(&format!("- This is a {} ({})\n", desc, file));
        }
    }

    // List top-level directory for context
    if let Ok(entries) = std::fs::read_dir(".") {
        let mut files: Vec<String> = entries
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                if name.starts_with('.') || name == "target" || name == "node_modules" {
                    None
                } else {
                    let is_dir = e.file_type().ok()?.is_dir();
                    Some(if is_dir {
                        format!("{}/", name)
                    } else {
                        name
                    })
                }
            })
            .collect();
        files.sort();
        if !files.is_empty() {
            ctx.push_str("\nProject root files:\n");
            for f in files.iter().take(50) {
                ctx.push_str(&format!("  {}\n", f));
            }
            if files.len() > 50 {
                ctx.push_str(&format!("  ... and {} more files\n", files.len() - 50));
            }
        }
    }

    ctx
}
