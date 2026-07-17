use crate::config::{Config, Message};
use crate::executor::Executor;
use crate::planner::Planner;
use crate::verifier::Verifier;
use crate::workspace;
use anyhow::{anyhow, Result};
use reqwest::Client;
use std::sync::Arc;

pub struct Agent {
    config: Arc<Config>,
    executor: Executor,
    planner: Planner,
 messages: Vec<Message>,
    client: Client,
}

impl Agent {
    pub async fn new(config:) -> Result<Self> {
        let client = Client::new();
        // Build system prompt with workspace context
        let ws_sum = workspace::scan_project(&std::env::current_dir()?)?;
        let system_prompt = format!(
            "You are a coding agent. Current workspace:\n{}\n\nYou have tools: {}",
            ws_summary,
            crate::executor::tool_definitions()
        );
        let mut messages = Vec::new();
        if let Some) = &config.context {
            messages.extend(context.clone        }
        // Add system message
        messages.insert(0, Message { role: "system".into(), content: system_prompt });
        Ok(Agent {
            config: Arc::new(config),
            executor: Executor::new()?,
            planner: Planner::new(),
            messages,
            client,
 })
    }

    pub async fn run(&mut self, task:str) -> Result<()> {
        // Add user task
        self.messages.push(Message { role: "user".into(), content: task.to_string() });

        loop {
            // Plan
            let plan = self.planner.generate_(task, &self.m).await?;
            if plan.steps.is_empty() {
                return Ok(());
            }

            // Execute and verify each step
            let mut all_done = true;
            for step inplan.steps {
                let result = self.execute_step(step).await?;
                if let Err(e) = Verifier::verify(step, &result) {
                    self.messages.push(Message {
                        role: "assistant",
                        content: format!("Step failed: {}. Result: {}", e, result),
                    });
                    all_done = false;
                    break;
                }
 // Success
                self.messages.push(Message {
 role: "assistant                    content: format!("Completed: {} -> {}", step.description, result),
                });
            }
            if all_done {
                return Ok(            }
            // Replan with new context if needed
        }
    }

    fn execute_step(&, step: &crate::planner::PlanStep) -> Result<String>        // Parse tool calls from step (simple format: TOOL_NAME arg=val)
        let output = self.executor(&step.tool_calls).await?;
       (output)
    }

    pub fn get_messages(&) -> Vec<Message> {
        self.messages()
    }
}
