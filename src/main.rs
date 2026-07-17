mod api;
mod config;
mod evolve;
mod executor;
mod memory;
mod planner;
mod render;
mod reviewer;
mod skills;
mod state;

use clap::Parser;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RlResult};
use state::{Phase, WorkflowState};
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "deepseek", about = "Autonomous DeepSeek coding agent")]
struct Args {
    #[arg(short = 'p', long)]
    prompt: Option<String>,

    #[arg(long)]
    yes: bool,

    #[arg(long, default_value = "3")]
    max_retries: u32,

    #[arg(long)]
    model: Option<String>,

    #[arg(long)]
    bare: bool,
}

fn history_path() -> std::path::PathBuf {
    config::Config::config_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("history")
}

fn create_editor() -> RlResult<DefaultEditor> {
    let mut rl = DefaultEditor::new()?;
    let _ = rl.load_history(&history_path());
    Ok(rl)
}

fn read_line(rl: &mut DefaultEditor, prompt: &str) -> Option<String> {
    match rl.readline(prompt) {
        Ok(line) => {
            let trimmed = line.trim().to_string();
            if !trimmed.is_empty() {
                let _ = rl.add_history_entry(&trimmed);
            }
            Some(trimmed)
        }
        Err(ReadlineError::Interrupted) => {
            println!("^C");
            None
        }
        Err(ReadlineError::Eof) => None,
        Err(e) => {
            eprintln!("  {}  Read error: {}", "✗".red(), e);
            None
        }
    }
}

fn save_history(rl: &mut DefaultEditor) {
    let _ = rl.save_history(&history_path());
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
        "  {}  {}  |  {}  |  {}",
        "⟡".bright_blue().bold(),
        format!("{}", cfg.model).cyan(),
        "Type /help for commands, /exit to quit".dimmed(),
        if cfg.model == "deepseek-chat" {
            "reasoning: auto".dimmed().to_string()
        } else {
            String::new()
        }
    );

    if !args.bare {
        let mem = memory::load_memory();
        if !mem.is_empty() {
            println!(
                "  {}  Memory: {} bytes | Journal: {} entries",
                "📝".dimmed(),
                mem.len(),
                memory::load_journals().len()
            );
        }
        if is_git_repo() {
            let branch = current_branch();
            if !branch.is_empty() {
                println!("  {}  Git: {}", "🔀".dimmed(), branch.cyan());
            }
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
    let mut rl = create_editor()?;

    loop {
        let input = match read_line(&mut rl, &format!("\n  {}  ", "⟡".bright_blue().bold())) {
            Some(line) => line,
            None => {
                save_history(&mut rl);
                println!("\n  {}  Goodbye.", "👋".dimmed());
                break;
            }
        };

        if input.is_empty() {
            continue;
        }

        match input.as_str() {
            "/exit" | "/quit" | "/q" => {
                save_history(&mut rl);
                println!("  {}  Goodbye.", "👋".dimmed());
                break;
            }
            "/help" | "/h" | "/?" => {
                print_help();
                continue;
            }
            "/clear" | "/c" => {
                print!("\x1B[2J\x1B[H");
                let _ = io::stdout().flush();
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
                prompt_memory(&mut rl);
                continue;
            }
            "/evolve" => {
                evolve_dispatch(api, max_retries, bare).await;
                continue;
            }
            s if s.starts_with("/evolve ") => {
                let rest = s.splitn(2, ' ').nth(1).unwrap_or("");
                evolve_dispatch_with(api, max_retries, bare, rest).await;
                continue;
            }
            s if s.starts_with('/') => {
                let cmd = s.trim_start_matches('/');
                println!(
                    "  {}  Unknown: /{}.  Type /help for commands.",
                    "✗".red(),
                    cmd
                );
                continue;
            }
            _ => {}
        }

        match run_task(api, &input, auto_yes, max_retries, bare).await {
            Ok(_) => save_history(&mut rl),
            Err(e) => {
                println!("\n  {}  {}", "✗".red().bold(), e.to_string().red());
                save_history(&mut rl);
            }
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

    let context = if bare {
        String::new()
    } else {
        memory::full_context()
    };
    state.add_message("system", &context);

    // ── Phase 1: Planning ──
    state.transition(Phase::Planning);
    print!("  {}  ", "⟳".yellow().bold());
    let _ = io::stdout().flush();
    let plan = planner::generate_plan(api, &mut state, &context).await?;
    render::print_plan_summary(&plan);

    // ── Phase 2: Approval ──
    state.transition(Phase::AwaitingApproval);
    if !confirm(api, "Proceed with this plan?", auto_yes).await? {
        println!("  {}  Cancelled.", "✗".red());
        return Ok(());
    }

    // ── Phase 3: Executing ──
    state.transition(Phase::Executing);
    println!();
    executor::execute_plan(api, &mut state).await?;

    // ── Phase 4: Reviewing ──
    state.transition(Phase::Reviewing);
    print!("  {}  ", "⟳".yellow().bold());
    let _ = io::stdout().flush();
    let suggestions = reviewer::review_and_suggest(api, &state).await?;
    state.suggestions = suggestions.clone();
    render::print_suggestions(&suggestions);

    // ── Phase 5: Optimize approval ──
    state.transition(Phase::AwaitingOptimizeApproval);
    if confirm(api, "Apply these improvements?", auto_yes).await? {
        state.transition(Phase::Optimizing);
        println!();
        if let Err(e) = reviewer::apply_optimizations(api, &mut state).await {
            println!("  {}  {}", "⚠".yellow(), e);
        } else {
            println!("  {}  Optimizations applied.", "✓".green().bold());
        }
    } else {
        println!("  {}  Skipping optimizations.", "→".dimmed());
    }

    // ── Save artifacts ──
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
            println!(
                "  {}  Journal: .deepseek/journal/{}",
                "📓".dimmed(),
                journal
            );
        }

        let plan_summary: String = state.plan.lines().take(3).collect::<Vec<_>>().join("; ");
        let _ = memory::update_memory_after_run(prompt, &plan_summary, &state.suggestions);
        println!("  {}  Memory updated", "🧠".dimmed());

        if is_git_repo() {
            match memory::git_commit(&format!(
                "deepseek: {}",
                prompt.chars().take(72).collect::<String>()
            )) {
                Ok(_) => println!("  {}  Committed", "🔀".dimmed()),
                Err(e) => println!("  {}  Git: {}", "⚠".yellow(), e),
            }
        }
    }

    println!();
    println!(
        "  {}  Done. {}",
        "🎯".green().bold(),
        "Type another task or /exit".dimmed()
    );
    Ok(())
}

async fn confirm(_api: &api::ApiClient, message: &str, auto_yes: bool) -> anyhow::Result<bool> {
    if auto_yes {
        println!("  {}  {} (auto)", "?".cyan().bold(), message.dimmed());
        return Ok(true);
    }
    println!();
    print!("  {}  {}  [Y/n] ", "?".cyan().bold(), message);
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let trimmed = input.trim().to_lowercase();
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
                "... and {} more lines (.deepseek/MEMORY.md)",
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

fn prompt_memory(rl: &mut DefaultEditor) {
    println!("  {}  What should I remember?", "🧠".cyan());
    if let Some(input) = read_line(rl, "  > ") {
        if !input.is_empty() {
            let _ = memory::append_memory(&format!(
                "## Manual entry — {}\n\n{}",
                chrono::Local::now().format("%Y-%m-%d %H:%M"),
                input
            ));
            println!("  {}  Saved.", "✓".green());
        }
    }
}

async fn evolve_dispatch(_api: &api::ApiClient, _max_retries: u32, _bare: bool) {
    println!();
    println!("  {}  Self-improvement", "🧬".bright_magenta().bold());
    println!(
        "  {}  Source: {}",
        " ".dimmed(),
        evolve::source_path().dimmed()
    );
    println!();
    println!("  {:<22} {}", "/evolve check".cyan(), "Diagnostics (clippy, fmt, deps, size)");
    println!("  {:<22} {}", "/evolve fmt".cyan(), "Auto-format with cargo fmt + rebuild");
    println!("  {:<22} {}", "/evolve fix".cyan(), "Auto-fix clippy warnings + rebuild");
    println!("  {:<22} {}", "/evolve strip".cyan(), "Strip debug symbols");
    println!("  {:<22} {}", "/evolve upgrade".cyan(), "Update deps + rebuild");
    println!("  {:<22} {}", "/evolve build".cyan(), "Rebuild and reinstall");
    println!(
        "  {:<22} {}",
        "/evolve <task>".cyan(),
        "AI-powered modification (needs API key)"
    );
    println!();
}

async fn evolve_dispatch_with(
    api: &api::ApiClient,
    max_retries: u32,
    bare: bool,
    subcommand: &str,
) {
    let cmd = subcommand.trim();
    match cmd {
        "check" | "diagnose" | "diag" => {
            println!("\n  {}  Diagnostics\n", "🔍".yellow().bold());
            println!("{}", evolve::run_diagnostics());
        }
        "fmt" | "format" => {
            println!("\n  {}  Formatting\n", "🎨".yellow().bold());
            println!("{}", evolve::auto_format());
            rebuild_and_log().await;
        }
        "fix" | "clippy" => {
            println!("\n  {}  Auto-fixing\n", "🔧".yellow().bold());
            println!("{}", evolve::auto_fix_clippy());
            rebuild_and_log().await;
        }
        "strip" => {
            println!("\n  {}  Stripping\n", "📦".yellow().bold());
            println!("{}", evolve::strip_binary());
        }
        "upgrade" | "update" => {
            println!("\n  {}  Updating deps\n", "⬆".yellow().bold());
            let src = evolve::source_path();
            match std::process::Command::new("cargo")
                .args(["update"])
                .current_dir(src)
                .output()
            {
                Ok(out) => {
                    if out.status.success() {
                        println!("  {}  Dependencies updated", "✓".green());
                    } else {
                        println!(
                            "  {}  {}",
                            "✗".red(),
                            String::from_utf8_lossy(&out.stderr)
                        );
                    }
                }
                Err(e) => println!("  {}  {}", "✗".red(), e),
            }
            rebuild_and_log().await;
        }
        "build" | "rebuild" => {
            println!("\n  {}  Rebuilding\n", "🔨".yellow().bold());
            rebuild_and_log().await;
        }
        _ => {
            println!(
                "\n  {}  AI self-modification: {}",
                "🧬".bright_magenta().bold(),
                cmd.cyan()
            );
            let prompt = evolve::self_improvement_prompt(cmd);
            match run_task(api, &prompt, false, max_retries, bare).await {
                Ok(_) => rebuild_and_log().await,
                Err(e) => println!("  {}  {}", "✗".red(), e),
            }
        }
    }
}

async fn rebuild_and_log() {
    match evolve::rebuild_and_reinstall().await {
        Ok(log) => println!("{}", log),
        Err(e) => println!("  {}  {}", "✗".red(), e),
    }
}

fn print_help() {
    println!();
    println!("{}", "  Commands".bright_white().bold().underline());
    println!(
        "  {}  Type any task — runs plan→execute→self-heal→review→optimize",
        "→".dimmed()
    );
    println!();
    println!("  {:<20} {}", "/help, /h".cyan(), "Show this help");
    println!("  {:<20} {}", "/exit, /q".cyan(), "Exit session");
    println!("  {:<20} {}", "/clear, /c".cyan(), "Clear screen");
    println!("  {:<20} {}", "/memory, /m".cyan(), "View project memory");
    println!("  {:<20} {}", "/git, /g".cyan(), "View git context");
    println!("  {:<20} {}", "/remember".cyan(), "Add manual memory entry");
    println!("  {:<20} {}", "/evolve".cyan(), "Self-improvement menu");
    println!();
    println!(
        "{}",
        "  Flags".bright_white().bold().underline()
    );
    println!("  {:<20} {}", "-p <task>".cyan(), "One-shot task (no REPL)");
    println!("  {:<20} {}", "--yes".cyan(), "Skip all confirmations");
    println!("  {:<20} {}", "--model <name>".cyan(), "Override model");
    println!("  {:<20} {}", "--bare".cyan(), "Skip memory + git");
    println!();
}
