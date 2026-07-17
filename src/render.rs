use colored::*;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub fn print_banner() {
    println!();
    println!(
        "  {}  {}",
        "⟡".bright_blue().bold(),
        "DeepSeek CLI".bright_blue().bold()
    );
    println!("  {}", "Autonomous Code Agent".dimmed());
    println!();
}

pub fn print_plan_summary(plan: &str) {
    println!();
    println!("{}", "┌─ PLAN ─────────────────────────────".cyan().bold());
    for line in plan.lines() {
        if line.trim().is_empty() {
            println!("{}", "│".cyan().dimmed());
        } else if line.trim().starts_with(|c: char| c.is_ascii_digit()) && line.contains('.') {
            println!("{} {}", "│".cyan().dimmed(), line.bright_white().bold());
        } else if line.trim().starts_with('-') || line.trim().starts_with('*') {
            println!("{}   {}", "│".cyan().dimmed(), line.trim().bright_white());
        } else {
            println!("{} {}", "│".cyan().dimmed(), line.trim().white());
        }
    }
    println!("{}", "└────────────────────────────────────".cyan().bold());
    println!();
}

pub fn print_suggestions(suggestions: &str) {
    println!();
    println!(
        "{}",
        "┌─ SUGGESTED IMPROVEMENTS ────────────".yellow().bold()
    );
    for line in suggestions.lines() {
        if line.trim().is_empty() {
            println!("{}", "│".yellow().dimmed());
        } else if line.to_uppercase().contains("BUG:") || line.to_uppercase().contains("BUGS:") {
            println!("{} {}", "│".yellow().dimmed(), line.trim().red().bold());
        } else if line.to_uppercase().contains("OPTIMIZ") {
            println!("{} {}", "│".yellow().dimmed(), line.trim().yellow().bold());
        } else if line.trim().starts_with(|c: char| c.is_ascii_digit()) {
            println!(
                "{} {}",
                "│".yellow().dimmed(),
                line.trim().bright_white().bold()
            );
        } else {
            println!("{} {}", "│".yellow().dimmed(), line.trim().white());
        }
    }
    println!(
        "{}",
        "└────────────────────────────────────".yellow().bold()
    );
    println!();
}

#[allow(dead_code)]
pub fn print_code_block(language: &str, code: &str) {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps
        .find_syntax_by_extension(language)
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    println!("{}", "┌────".dimmed());
    for line in LinesWithEndings::from(code) {
        let ranges = h.highlight_line(line, &ps).unwrap_or_default();
        let escaped = as_24_bit_terminal_escaped(&ranges, false);
        print!("{} │ {}", "│".dimmed(), escaped);
    }
    println!("{}", "└────".dimmed());
}

#[allow(dead_code)]
pub fn print_token_counter(tokens_in: u64, tokens_out: u64) {
    let cost_per_m_in = 0.27;
    let cost_per_m_out = 1.10;
    let cost =
        (tokens_in as f64 * cost_per_m_in + tokens_out as f64 * cost_per_m_out) / 1_000_000.0;

    println!(
        "  {}  Tokens: {} in + {} out | Est. cost: ${:.4}",
        "💰".dimmed(),
        tokens_in,
        tokens_out,
        cost
    );
}

#[allow(dead_code)]
pub fn print_error(msg: &str) {
    println!("  {}  {}", "✗".red().bold(), msg.red());
}

#[allow(dead_code)]
pub fn print_success(msg: &str) {
    println!("  {}  {}", "✓".green().bold(), msg.green());
}

#[allow(dead_code)]
pub fn print_info(msg: &str) {
    println!("  {}  {}", "ℹ".blue().bold(), msg.white());
}

pub fn print_skill_activation(skills: &[crate::skills::Skill]) {
    if skills.is_empty() {
        return;
    }
    let names: Vec<String> = skills.iter().map(|s| s.name.clone()).collect();
    println!(
        "  {}  Activated skills: {}",
        "🧠".dimmed(),
        names.join(", ").cyan()
    );
}
