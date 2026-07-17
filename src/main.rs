mod api;
mod config;
mod executor;
mod planner;
mod render;
mod reviewer;
mod skills;
mod state;

use clap::Parser;
use colored::*;
use state::{Phase, WorkflowState};
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "deepseek", about = "Autonomous DeepSeek coding agent")]
struct Args {
    /// The task to execute (leave empty for interactive mode)
    #[arg(short = 'p', long)]
    prompt: Option<String>,

    /// Auto-approve all phases (no user confirmation)
    #[arg(long)]
    yes: bool,

    /// Maximum self-healing retry attempts
    #[arg(long, default_value = "3")]
    max_retries: u32,

    /// DeepSeek model to use
    #[arg(long)]
    model: Option<String>,

    /// Skip project context gathering
    #[arg(long)]
    bare: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    render::print_banner();

    // Load config
    let mut cfg = config::Config::load()?;
    if let Some(ref model) = args.model {
        cfg.model = model.clone();
    }
    println!("  {}  Model: {}", "⚡".dimmed(), cfg.model.cyan());

    // Create API client
    let api = api::ApiClient::new(cfg)?;

    // Get the prompt
    let prompt = match args.prompt {
        Some(p) => p,
        None => {
            print!("  {}  What should I build? ", "?".cyan().bold());
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    if prompt.is_empty() {
        anyhow::bail!("No task provided.");
    }

    // Route skills based on the prompt
    let matched_skills = skills::route_skills(&prompt);
    render::print_skill_activation(&matched_skills);

    // Initialize state machine
    let mut state = WorkflowState::new(prompt, matched_skills);
    state.max_retries = args.max_retries;

    // Phase 1: Planning
    state.transition(Phase::Planning);
    println!("\n  {}  Thinking...\n", "⟳".yellow().bold());
    let plan = planner::generate_plan(&api, &mut state).await?;
    render::print_plan_summary(&plan);

    // Phase 2: Awaiting Approval
    state.transition(Phase::AwaitingApproval);
    if !confirm("Proceed with this plan?", args.yes)? {
        println!("  {}  Cancelled.", "✗".red());
        return Ok(());
    }

    // Phase 3: Executing
    state.transition(Phase::Executing);
    println!();
    executor::execute_plan(&api, &mut state).await?;

    // Phase 4: Reviewing
    state.transition(Phase::Reviewing);
    println!("\n  {}  Analyzing results...\n", "⟳".yellow().bold());
    let suggestions = reviewer::review_and_suggest(&api, &state).await?;
    state.suggestions = suggestions.clone();
    render::print_suggestions(&suggestions);

    // Phase 5: Awaiting optimization approval
    state.transition(Phase::AwaitingOptimizeApproval);
    if confirm("Apply these improvements?", args.yes)? {
        // Phase 6: Optimizing
        state.transition(Phase::Optimizing);
        println!();
        let result = reviewer::apply_optimizations(&api, &state).await?;
        println!("  {}  Optimizations applied.", "✓".green().bold());
        if !result.is_empty() && !result.contains("ALL_DONE") {
            println!("  {}", result.dimmed());
        }
    } else {
        println!("  {}  Skipping optimizations.", "→".dimmed());
    }

    // Done
    state.transition(Phase::Done);
    println!();
    println!(
        "  {}  Task complete. {}",
        "🎯".green().bold(),
        "Ready for your next command.".dimmed()
    );
    println!();

    Ok(())
}

fn confirm(message: &str, auto_yes: bool) -> anyhow::Result<bool> {
    if auto_yes {
        println!("  {}  {} (auto-approved)", "?".cyan().bold(), message);
        return Ok(true);
    }

    print!("  {}  {} [Y/n] ", "?".cyan().bold(), message);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let trimmed = input.trim().to_lowercase();
    Ok(trimmed.is_empty() || trimmed == "y" || trimmed == "yes")
}
