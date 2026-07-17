use crate::api;
use crate::api::{ApiClient, Message};
use crate::state::WorkflowState;
use anyhow::Result;
#[allow(unused_imports)]
use futures::StreamExt;
use std::io::{self, Write};

pub async fn review_and_suggest(api: &ApiClient, state: &WorkflowState) -> Result<String> {
    let messages = vec![Message {
        role: "user".into(),
        content: format!(
            "Review these changes and list any issues.\n\n\
             Task: {}\n\nExecution log:\n{}\n\n\
             List only concrete issues: bugs, edge cases, missing tests, perf problems.\n\
             Be brief. One line per issue.",
            state.prompt,
            state.execution_log.join("\n")
        ),
    }];

    let mut stream = Box::pin(api.stream_chat(messages));
    let mut suggestions = String::new();

    while let Some(event) = stream.next().await {
        match event {
            api::StreamEvent::Content(text) => {
                suggestions.push_str(&text);
                print!("{}", text);
                let _ = io::stdout().flush();
            }
            api::StreamEvent::Done => break,
            api::StreamEvent::Error(e) => anyhow::bail!(e),
            _ => {}
        }
    }
    println!();

    Ok(suggestions)
}

pub async fn apply_optimizations(api: &ApiClient, state: &mut WorkflowState) -> Result<()> {
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
    state.plan =
        "Apply each suggested improvement one by one. Read files, write changes, run tests."
            .to_string();
    state.execution_log.clear();
    state.error_count = 0;

    crate::executor::execute_plan(api, state).await?;

    // Restore original state
    state.prompt = original_prompt;
    state.plan = original_plan;

    Ok(())
}
