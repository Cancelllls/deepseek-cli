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

    let entries = [
        ("package.json", "Node.js/TypeScript"),
        ("Cargo.toml", "Rust"),
        ("go.mod", "Go"),
        ("requirements.txt", "Python"),
        ("pyproject.toml", "Python"),
        ("Gemfile", "Ruby"),
        ("tsconfig.json", "TypeScript"),
        ("next.config", "Next.js"),
        ("Dockerfile", "Docker"),
    ];

    for (file, desc) in &entries {
        if std::path::Path::new(file).exists() {
            ctx.push_str(&format!("- {} project ({})\n", desc, file));
        }
    }

    ctx
}
