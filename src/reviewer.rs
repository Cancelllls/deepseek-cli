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
            "Review this work. List only concrete issues (bugs, missing tests, edge cases). Be brief.\n\n\
             Task: {}\nLog:\n{}\n",
            state.prompt,
            state.execution_log.join("\n")
        ),
    }];

    let suggestions = api.chat(messages).await?;
    Ok(suggestions)
}

pub async fn apply_optimizations(
    api: &ApiClient,
    state: &mut WorkflowState,
) -> Result<()> {
    let original = state.prompt.clone();
    let suggestions = state.suggestions.clone();

    state.prompt = format!(
        "Apply these improvements:\n{}\n\nUse code blocks with file paths. Reply DONE when done.",
        suggestions
    );
    state.plan = "Fix each issue listed above.".to_string();
    state.execution_log.clear();
    state.error_count = 0;

    crate::executor::execute_plan(api, state).await?;
    state.prompt = original;
    Ok(())
}
