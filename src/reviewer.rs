use crate::api;
use crate::api::{ApiClient, Message};
use crate::state::WorkflowState;
use anyhow::Result;
#[allow(unused_imports)]
use futures::StreamExt;
use std::io::{self, Write};

pub async fn review_and_suggest(
    api: &ApiClient,
    state: &WorkflowState,
) -> Result<String> {
    let messages = vec![Message {
        role: "user".into(),
        content: format!(
            "Review the following completed task and suggest improvements.\n\n\
             ## Original Task\n{}\n\n\
             ## Plan\n{}\n\n\
             ## Execution Log\n{}\n\n\
             Identify:\n\
             1. Edge cases that might break the implementation\n\
             2. Performance optimizations that could be applied\n\
             3. Error handling improvements\n\
             4. Code quality improvements (better patterns, deduplication, etc.)\n\
             5. Missing tests or validations\n\n\
             For each suggestion, explain WHAT to change and WHY.\n\
             Group suggestions into categories: BUGS, OPTIMIZATIONS, and POLISH.\n\
             Be concise and actionable.",
            state.prompt,
            state.plan,
            state.execution_log.join("\n")
        ),
    }];

    let mut stream = Box::pin(api.stream_chat(messages));
    let mut suggestions = String::new();
    let mut stderr = io::stderr();

    print!("  ");
    let _ = io::stdout().flush();

    while let Some(event) = stream.next().await {
        match event {
            api::StreamEvent::Content(text) => {
                suggestions.push_str(&text);
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

    Ok(suggestions)
}

pub async fn apply_optimizations(
    api: &ApiClient,
    state: &mut WorkflowState,
) -> Result<()> {
    let original_prompt = state.prompt.clone();
    let original_plan = state.plan.clone();
    let suggestions = state.suggestions.clone();

    state.prompt = format!(
        "Apply the following improvements to the codebase. \
         Use tools to read files, write code, and run commands.\n\n\
         ## Original Task\n{}\n\n\
         ## Improvements to Apply\n{}\n\n\
         Execute each improvement. Update files, run tests, verify changes. \
         When done, reply ALL_DONE.",
        original_prompt, suggestions
    );
    state.plan = "Apply each suggested improvement one by one. Read files, write changes, run tests.".to_string();
    state.execution_log.clear();
    state.error_count = 0;

    crate::executor::execute_plan(api, state).await?;

    // Restore original state
    state.prompt = original_prompt;
    state.plan = original_plan;

    Ok(())
}
