use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub when_to_use: String,
    pub content: String,
}

impl Skill {
    pub fn matches(&self, text: &str) -> bool {
        let lower = text.to_lowercase();
        self.keywords
            .iter()
            .any(|k| lower.contains(&k.to_lowercase()))
    }

    pub fn instruction_body(&self) -> String {
        if let Some(idx) = self.content.find("\n---\n") {
            self.content[idx + 5..].trim().to_string()
        } else if let Some(idx) = self.content.find("---\n") {
            if let Some(end) = self.content[idx + 4..].find("\n---\n") {
                self.content[idx + 4 + end + 5..].trim().to_string()
            } else {
                self.content.clone()
            }
        } else {
            self.content.clone()
        }
    }
}

include!("skills_data.rs");

pub fn route_skills(prompt: &str) -> Vec<Skill> {
    let all = all_skills();
    let mut matches: Vec<&Skill> = all.iter().filter(|s| s.matches(prompt)).collect();
    matches.sort_by_key(|s| {
        let lower = prompt.to_lowercase();
        -(s.keywords
            .iter()
            .filter(|k| lower.contains(&k.to_lowercase()))
            .count() as i32)
    });
    matches.into_iter().take(5).cloned().collect()
}

pub fn build_system_prompt(skills: &[Skill], project_context: &str) -> String {
    let mut prompt = String::new();

    prompt.push_str("You are an expert software engineer working inside a terminal. ");
    prompt.push_str("You have access to filesystem tools (read_file, write_file, run_command, search_code, list_dir). ");
    prompt.push_str("Respond with precise, actionable code. ");
    prompt.push_str("When planning, break work into numbered steps. ");
    prompt.push_str("When executing, make actual file changes. ");
    prompt.push_str("Always verify your work by running tests or checking syntax.\n\n");

    if !project_context.is_empty() {
        prompt.push_str("## Project Context\n\n");
        prompt.push_str(project_context);
        prompt.push_str("\n\n");
    }

    if !skills.is_empty() {
        prompt.push_str("## Activated Domain Skills\n\n");
        for skill in skills {
            prompt.push_str(&format!(
                "### {} — {}\n{}\n\n",
                skill.name,
                skill.description,
                skill.instruction_body()
            ));
        }
    }

    prompt.push_str("## Code Conventions\n\n");
    prompt.push_str("- Write idiomatic code for the target language/framework\n");
    prompt.push_str("- Follow existing project conventions\n");
    prompt.push_str("- Keep changes minimal and focused\n");
    prompt.push_str("- Do NOT add unnecessary comments\n");
    prompt.push_str("- Use existing libraries and utilities in the project\n");

    prompt
}
