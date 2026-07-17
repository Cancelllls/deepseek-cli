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
    /// The task to execute in one-shot mode (no REPL)
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

    let mut cfg = config::Config::load()?;
    if let Some(ref model) = args.model {
        cfg.model = model.clone();
    }
    println!(
        "  {}  {}  |  {}",
        "⟡".bright_blue().bold(),
        format!("Model: {}", cfg.model).cyan(),
        "Type /help for commands, /exit to quit".dimmed()
    );

    let api = api::ApiClient::new(cfg)?;

    if let Some(prompt) = args.prompt {
        run_task(&api, &prompt, args.yes, args.max_retries).await?;
    } else {
        interactive_loop(&api, args.yes, args.max_retries).await?;
    }

    Ok(())
}

async fn interactive_loop(
    api: &api::ApiClient,
    auto_yes: bool,
    max_retries: u32,
) -> anyhow::Result<()> {
    loop {
        print!("\n  {}  ", "⟡".bright_blue().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_string();

        if input.is_empty() {
            continue;
        }

        match input.as_str() {
            "/exit" | "/quit" | "/q" => {
                println!("  {}  Goodbye.", "👋".dimmed());
                break;
            }
            "/help" | "/h" | "/?" => {
                print_help();
                continue;
            }
            "/clear" | "/c" => {
                print!("\x1B[2J\x1B[H");
                io::stdout().flush()?;
                render::print_banner();
                continue;
            }
            s if s.starts_with('/') => {
                let cmd = s.trim_start_matches('/');
                println!("  {}  Unknown command: /{}. Type /help for commands.", "✗".red(), cmd);
                continue;
            }
            _ => {}
        }

        if let Err(e) = run_task(api, &input, auto_yes, max_retries).await {
            println!("  {}  Error: {}", "✗".red().bold(), e);
        }
    }
    Ok(())
}

async fn run_task(
    api: &api::ApiClient,
    prompt: &str,
    auto_yes: bool,
    max_retries: u32,
) -> anyhow::Result<()> {
    let matched_skills = skills::route_skills(prompt);
    render::print_skill_activation(&matched_skills);

    let mut state = WorkflowState::new(prompt.to_string(), matched_skills);
    state.max_retries = max_retries;

    // Phase 1: Planning
    state.transition(Phase::Planning);
    println!("\n  {}  Thinking...\n", "⟳".yellow().bold());
    let plan = planner::generate_plan(api, &mut state).await?;
    render::print_plan_summary(&plan);

    // Phase 2: Awaiting Approval
    state.transition(Phase::AwaitingApproval);
    if !confirm("Proceed with this plan?", auto_yes)? {
        println!("  {}  Cancelled.", "✗".red());
        return Ok(());
    }

    // Phase 3: Executing
    state.transition(Phase::Executing);
    println!();
    executor::execute_plan(api, &mut state).await?;

    // Phase 4: Reviewing
    state.transition(Phase::Reviewing);
    println!("\n  {}  Analyzing results...\n", "⟳".yellow().bold());
    let suggestions = reviewer::review_and_suggest(api, &state).await?;
    state.suggestions = suggestions.clone();
    render::print_suggestions(&suggestions);

    // Phase 5: Awaiting optimization approval
    state.transition(Phase::AwaitingOptimizeApproval);
    if confirm("Apply these improvements?", auto_yes)? {
        // Phase 6: Optimizing
        state.transition(Phase::Optimizing);
        println!();
        let result = reviewer::apply_optimizations(api, &state).await?;
        println!("  {}  Optimizations applied.", "✓".green().bold());
        if !result.is_empty() && !result.contains("ALL_DONE") {
            println!("  {}", result.dimmed());
        }
    } else {
        println!("  {}  Skipping optimizations.", "→".dimmed());
    }

    state.transition(Phase::Done);
    println!(
        "\n  {}  Task complete.",
        "🎯".green().bold()
    );

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

fn print_help() {
    println!();
    println!(
        "  {}",
        "Commands".bright_white().bold().underline()
    );
    println!(
        "  {}  Type any task to run the full agent pipeline",
        "→".dimmed()
    );
    println!(
        "  {}    plan → approve → execute → self-heal → suggest → approve → optimize",
        " ".dimmed()
    );
    println!();
    println!("  {:<12} {}", "/help, /h".cyan(), "Show this help");
    println!("  {:<12} {}", "/exit, /q".cyan(), "Exit the session");
    println!("  {:<12} {}", "/clear, /c".cyan(), "Clear the screen");
    println!();
    println!(
        "  {}",
        "Flags (one-shot mode only)".bright_white().bold().underline()
    );
    println!("  {:<12} {}", "--yes".cyan(), "Auto-approve all checkpoints");
    println!(
        "  {:<12} {}",
        "--model".cyan(),
        "Override the default model"
    );
    println!(
        "  {:<12} {}",
        "-p <task>".cyan(),
        "Run a single task (no REPL)"
    );
    println!();
}
