use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub instructions: String,
}

impl Skill {
    pub fn matches(&self, text: &str) -> bool {
        let lower = text.to_lowercase();
        self.keywords.iter().any(|k| lower.contains(&k.to_lowercase()))
    }
}

pub fn all_skills() -> Vec<Skill> {
    vec![
        Skill {
            name: "rust".into(),
            description: "Rust systems programming".into(),
            keywords: vec!["rust", "cargo", "tokio", "serde", "trait", "lifetime", "ownership",
                "async", "clap", "actix", "axum", "struct", "impl", "borrow",
            ].into_iter().map(Into::into).collect(),
            instructions: include_str!("skills/rust.md").into(),
        },
        Skill {
            name: "python".into(),
            description: "Python development".into(),
            keywords: vec!["python", "django", "flask", "fastapi", "pip", "pytest", "numpy",
                "pandas", "asyncio", "pydantic", "sqlalchemy",
            ].into_iter().map(Into::into).collect(),
            instructions: include_str!("skills/python.md").into(),
        },
        Skill {
            name: "react".into(),
            description: "React & Next.js frontend".into(),
            keywords: vec!["react", "nextjs", "next.js", "jsx", "tsx", "component", "hook",
                "useState", "useEffect", "tailwind", "shadcn", "vercel", "vite",
            ].into_iter().map(Into::into).collect(),
            instructions: include_str!("skills/react.md").into(),
        },
        Skill {
            name: "api".into(),
            description: "API design and patterns".into(),
            keywords: vec!["api", "rest", "graphql", "endpoint", "route", "authentication",
                "jwt", "oauth", "middleware", "cors", "rate limit", "pagination",
            ].into_iter().map(Into::into).collect(),
            instructions: include_str!("skills/api.md").into(),
        },
        Skill {
            name: "database".into(),
            description: "Database design and optimization".into(),
            keywords: vec!["database", "sql", "postgres", "mysql", "mongodb", "sqlite",
                "prisma", "migration", "index", "query", "schema", "orm", "redis",
            ].into_iter().map(Into::into).collect(),
            instructions: include_str!("skills/database.md").into(),
        },
        Skill {
            name: "testing".into(),
            description: "Testing and quality assurance".into(),
            keywords: vec!["test", "testing", "unit test", "integration test", "jest", "vitest",
                "playwright", "coverage", "mock", "assert", "fixture",
            ].into_iter().map(Into::into).collect(),
            instructions: include_str!("skills/testing.md").into(),
        },
        Skill {
            name: "security".into(),
            description: "Security auditing".into(),
            keywords: vec!["security", "vulnerability", "xss", "csrf", "sql injection", "auth",
                "encrypt", "hash", "password", "secret", "token", "ssl", "tls",
            ].into_iter().map(Into::into).collect(),
            instructions: include_str!("skills/security.md").into(),
        },
        Skill {
            name: "devops".into(),
            description: "DevOps and deployment".into(),
            keywords: vec!["deploy", "docker", "kubernetes", "ci/cd", "github actions", "aws",
                "nginx", "server", "monitoring", "logging", "backup",
            ].into_iter().map(Into::into).collect(),
            instructions: include_str!("skills/devops.md").into(),
        },
        Skill {
            name: "mobile".into(),
            description: "Mobile development".into(),
            keywords: vec!["mobile", "ios", "android", "react native", "flutter", "swift",
                "kotlin", "expo", "app store", "play store",
            ].into_iter().map(Into::into).collect(),
            instructions: include_str!("skills/mobile.md").into(),
        },
        Skill {
            name: "performance".into(),
            description: "Performance optimization".into(),
            keywords: vec!["performance", "optimize", "slow", "latency", "cache", "bundle size",
                "lazy load", "memo", "benchmark", "profile", "bottleneck",
            ].into_iter().map(Into::into).collect(),
            instructions: include_str!("skills/performance.md").into(),
        },
    ]
}

pub fn route_skills(prompt: &str) -> Vec<Skill> {
    let all = all_skills();
    let mut matches: Vec<&Skill> = all.iter().filter(|s| s.matches(prompt)).collect();
    // Sort by number of keyword matches (most relevant first)
    matches.sort_by_key(|s| {
        let lower = prompt.to_lowercase();
        -(s.keywords
            .iter()
            .filter(|k| lower.contains(&k.to_lowercase()))
            .count() as i32)
    });
    matches.into_iter().take(3).cloned().collect()
}

pub fn build_system_prompt(skills: &[Skill], project_context: &str) -> String {
    let mut prompt = String::new();

    prompt.push_str("You are an expert software engineer working inside a terminal. ");
    prompt.push_str("You have access to filesystem tools. ");
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
        prompt.push_str("## Activated Skills\n\n");
        for skill in skills {
            prompt.push_str(&format!(
                "### {}\n\n{}\n\n",
                skill.name, skill.instructions
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
