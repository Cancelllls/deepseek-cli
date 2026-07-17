mod api;
mod config;
mod executor;
mod memory;
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

    /// Skip project context and memory loading
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

    if !args.bare {
        let mem = memory::load_memory();
        if !mem.is_empty() {
            println!(
                "  {}  Memory loaded ({} bytes) | Journal: {} entries",
                "📝".dimmed(),
                mem.len(),
                memory::load_journals().len()
            );
        }

        if is_git_repo() {
            let branch = current_branch();
            println!(
                "  {}  Git repo detected {}",
                "🔀".dimmed(),
                if !branch.is_empty() {
                    format!("(branch: {})", branch)
                } else {
                    String::new()
                }
            );
        }
    }

    let api = api::ApiClient::new(cfg)?;

    if let Some(prompt) = args.prompt {
        run_task(&api, &prompt, args.yes, args.max_retries, args.bare).await?;
    } else {
        interactive_loop(&api, args.yes, args.max_retries, args.bare).await?;
    }

    Ok(())
}

async fn interactive_loop(
    api: &api::ApiClient,
    auto_yes: bool,
    max_retries: u32,
    bare: bool,
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
            "/memory" | "/m" => {
                show_memory();
                continue;
            }
            "/git" | "/g" => {
                show_git();
                continue;
            }
            "/remember" => {
                prompt_memory();
                continue;
            }
            s if s.starts_with('/') => {
                let cmd = s.trim_start_matches('/');
                println!(
                    "  {}  Unknown command: /{}. Type /help for commands.",
                    "✗".red(),
                    cmd
                );
                continue;
            }
            _ => {}
        }

        if let Err(e) = run_task(api, &input, auto_yes, max_retries, bare).await {
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
    bare: bool,
) -> anyhow::Result<()> {
    let matched_skills = skills::route_skills(prompt);
    render::print_skill_activation(&matched_skills);

    let mut state = WorkflowState::new(prompt.to_string(), matched_skills);
    state.max_retries = max_retries;

    // Load context
    let context = if bare {
        String::new()
    } else {
        memory::full_context()
    };
    state.add_message("system", &context);

    // Phase 1: Planning
    state.transition(Phase::Planning);
    println!("\n  {}  Thinking...\n", "⟳".yellow().bold());
    let plan = planner::generate_plan(api, &mut state, &context).await?;
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

    // Save session artifacts
    state.transition(Phase::Done);
    if !bare {
        let journal = memory::save_journal(
            prompt,
            &state.plan,
            &state.execution_log,
            &state.suggestions,
        )
        .unwrap_or_default();
        if !journal.is_empty() {
            println!("  {}  Journal saved: .deepseek/journal/{}", "📓".dimmed(), journal);
        }

        let plan_summary: String = state.plan.lines().take(3).collect::<Vec<_>>().join("; ");
        let _ = memory::update_memory_after_run(prompt, &plan_summary, &state.suggestions);
        println!("  {}  Memory updated", "🧠".dimmed());

        if is_git_repo() {
            match memory::git_commit(&format!(
                "deepseek: {}",
                prompt.chars().take(72).collect::<String>()
            )) {
                Ok(_) => println!("  {}  Changes committed to git", "🔀".dimmed()),
                Err(e) => println!("  {}  Git: {}", "⚠".yellow(), e),
            }
        }
    }

    println!("\n  {}  Task complete.", "🎯".green().bold());
    Ok(())
}

fn confirm(message: &str, auto_yes: bool) -> anyhow::Result<bool> {
    if auto_yes {
        println!("  {}  {} (auto-approved)", "?".cyan().bold(), message);
        return Ok(true);
    }

    println!();
    println!("  {}  {} [Y/n]", "?".cyan().bold(), message);
    print!("  > ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_lowercase();
    println!();
    Ok(trimmed.is_empty() || trimmed == "y" || trimmed == "yes")
}

fn is_git_repo() -> bool {
    std::path::Path::new(".git").exists()
}

fn current_branch() -> String {
    std::process::Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

fn show_memory() {
    let mem = memory::load_memory();
    if mem.is_empty() {
        println!("\n  {}  No memory yet. It builds up as you run tasks.\n", "📝".dimmed());
        return;
    }
    println!();
    println!("{}", "┌─ PROJECT MEMORY ────────────────────".cyan().bold());
    for line in mem.lines().take(40) {
        println!("{} {}", "│".cyan().dimmed(), line);
    }
    if mem.lines().count() > 40 {
        println!(
            "{} {}",
            "│".cyan().dimmed(),
            format!(
                "... and {} more lines (see .deepseek/MEMORY.md)",
                mem.lines().count() - 40
            )
            .dimmed()
        );
    }
    println!("{}", "└────────────────────────────────────".cyan().bold());
    println!();
}

fn show_git() {
    if !is_git_repo() {
        println!("\n  {}  Not a git repository.\n", "📝".dimmed());
        return;
    }
    let ctx = memory::git_context();
    println!();
    println!("{}", "┌─ GIT CONTEXT ───────────────────────".cyan().bold());
    for line in ctx.lines() {
        println!("{} {}", "│".cyan().dimmed(), line);
    }
    println!("{}", "└────────────────────────────────────".cyan().bold());
    println!();
}

fn prompt_memory() {
    println!("  {}  What should I remember about this project?", "🧠".cyan());
    print!("  {}  ", "   ".cyan());
    let _ = io::stdout().flush();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() && !input.trim().is_empty() {
        let _ = memory::append_memory(&format!(
            "## Manual entry — {}\n\n{}",
            chrono::Local::now().format("%Y-%m-%d %H:%M"),
            input.trim()
        ));
        println!("  {}  Saved to memory.", "✓".green());
    }
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
    println!("  {:<16} {}", "/help, /h".cyan(), "Show this help");
    println!("  {:<16} {}", "/exit, /q".cyan(), "Exit the session");
    println!("  {:<16} {}", "/clear, /c".cyan(), "Clear the screen");
    println!("  {:<16} {}", "/memory, /m".cyan(), "View project memory");
    println!("  {:<16} {}", "/git, /g".cyan(), "View git context");
    println!("  {:<16} {}", "/remember".cyan(), "Add a manual memory entry");
    println!();
    println!(
        "  {}",
        "Flags (one-shot mode only)".bright_white().bold().underline()
    );
    println!("  {:<16} {}", "--yes".cyan(), "Auto-approve all checkpoints");
    println!("  {:<16} {}", "--model".cyan(), "Override the default model");
    println!("  {:<16} {}", "-p <task>".cyan(), "Run a single task (no REPL)");
    println!("  {:<16} {}", "--bare".cyan(), "Skip memory and git integration");
    println!();
}
