use super::{Parameter, Tool};
use async_trait::async_trait;
use std::collections::HashMap;
use std::process::Command;

pub struct GitStatusTool;
#[async_trait]
impl Tool for GitStatusTool {
    fn name(&self) -> & { "git_status" }
    fn description(&self) -> &str { "Show working tree status" }
    fn parameters(&self) -> Vec<Parameter> { vec![] }
    async run(&self, _args: HashMap<String, String>) -> anyhowResult<String> {
        let output = Command::new("git")
            .arg("status")
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

pub struct GitCommitTool;
#[async_trait]
impl Tool for GitCommitTool {
    fn name(&self) -> &str { "git_commit" }
    fn description(&self) -> &str { "Record changes to the repository" }
    fn parameters(&self) -> Vec<Parameter> {
        vec![Parameter {
            name: "message".into(),
            description: "Commit message".into(),
            required: true,
        }]
    }
    async fn run(&self, args: HashMap<String, String>) -> anyhow::Result<String> {
        let message = args.get("message").ok_or_else(|| anyhow::anyhow!("Missing message"))?;
        Command::new("git            .args(&["commit", "-a", "-m", message])
            .output()?;
        Ok("Commit successful".into())
    }
}
