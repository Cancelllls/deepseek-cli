use crate::api::Message;
use crate::skills::Skill;
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Phase {
    Planning,
    AwaitingApproval,
    Executing,
    SelfHealing,
    Reviewing,
    AwaitingOptimizeApproval,
    Optimizing,
    Done,
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Phase::Planning => write!(f, "Planning"),
            Phase::AwaitingApproval => write!(f, "Awaiting Approval"),
            Phase::Executing => write!(f, "Executing"),
            Phase::SelfHealing => write!(f, "Self-Healing"),
            Phase::Reviewing => write!(f, "Reviewing"),
            Phase::AwaitingOptimizeApproval => write!(f, "Awaiting Optimize Approval"),
            Phase::Optimizing => write!(f, "Optimizing"),
            Phase::Done => write!(f, "Done"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkflowState {
    pub phase: Phase,
    pub prompt: String,
    pub skills: Vec<Skill>,
    pub chat_history: Vec<Message>,
    pub plan: String,
    pub execution_log: Vec<String>,
    pub suggestions: String,
    pub error_count: u32,
    pub max_retries: u32,
}

impl WorkflowState {
    pub fn new(prompt: String, skills: Vec<Skill>) -> Self {
        Self {
            phase: Phase::Planning,
            prompt,
            skills,
            chat_history: Vec::new(),
            plan: String::new(),
            execution_log: Vec::new(),
            suggestions: String::new(),
            error_count: 0,
            max_retries: 3,
        }
    }

    pub fn add_message(&mut self, role: &str, content: &str) {
        self.chat_history.push(Message {
            role: role.into(),
            content: content.into(),
        });
    }

    pub fn log(&mut self, msg: &str) {
        self.execution_log.push(msg.to_string());
    }

    pub fn transition(&mut self, phase: Phase) {
        let old = self.phase.clone();
        self.phase = phase.clone();
        println!(
            "  {}  {} → {}",
            "[STATE]".yellow().bold(),
            old.to_string().dimmed(),
            phase.to_string().green().bold(),
        );
    }

    #[allow(dead_code)]
    pub fn phase_banner(&self) -> String {
        match self.phase {
            Phase::Planning => "Generating implementation plan...".into(),
            Phase::AwaitingApproval => "Review the plan above. Proceed?".into(),
            Phase::Executing => "Executing plan...".into(),
            Phase::SelfHealing => "Detected issues — attempting auto-fix...".into(),
            Phase::Reviewing => "Reviewing results, looking for improvements...".into(),
            Phase::AwaitingOptimizeApproval => "Apply the suggested improvements?".into(),
            Phase::Optimizing => "Applying optimizations...".into(),
            Phase::Done => "All done.".into(),
        }
    }
}
