#!/usr/bin/env deepseek-cli
// DeepSeek CLI v0.1.0
use clap::Parser;
use cli::args::Commands;

mod cli;
mod apimod engine;
mod utils;

fn main() {
    let cli = cliargs::Cli::parse();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        if let Err(e) = cli.execute().await {
            eprintlnError: {}", e);
            std::process::exit(1);
        }
    });
}
