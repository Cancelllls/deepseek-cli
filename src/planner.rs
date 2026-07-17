use crate::api::{ApiClient, Message};
use crate::skills;
use crate::state::WorkflowState;
use anyhow::Result;

pub async fn generate_plan(
    api: &ApiClient,
    state: &mut WorkflowState,
) -> Result<String> {
    let system = skills::build_system_prompt(&state.skills, &gather_project_context());

    let mut messages: Vec<Message> = vec![
        Message {
            role: "system".into(),
            content: system,
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

    let plan = api.chat(messages).await?;
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
