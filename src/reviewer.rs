use crate::api::{ApiClient, Message};
use crate::state::WorkflowState;
use anyhow::Result;

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

    let suggestions = api.chat(messages).await?;
    Ok(suggestions)
}

pub async fn apply_optimizations(
    api: &ApiClient,
    state: &WorkflowState,
) -> Result<String> {
    let messages = vec![Message {
        role: "user".into(),
        content: format!(
            "Apply the following improvements to the codebase. \
             Use tools to read files, write code, and run commands.\n\n\
             ## Original Task\n{}\n\n\
             ## Suggested Improvements\n{}\n\n\
             Execute each improvement. Update files, run tests, verify changes. \
             When done, reply ALL_DONE.",
            state.prompt, state.suggestions
        ),
    }];

    let result = api.chat(messages).await?;
    Ok(result)
}
