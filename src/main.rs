use std::fs;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: Command,
    path: std::path::PathBuf,
    arg1: Option<String>,
}

#[derive(clap::ValueEnum, Clone)]
enum Command {
    Search,
    Count,
}

fn main() {
    let statement: Cli = Cli::parse();
    let content = fs::read_to_string(statement.path).unwrap();
    match statement.command {
        Command::Search => search(content, statement.arg1),
        Command::Count => count(content, statement.arg1),
    }
}

fn count(content: String, pattern: Option<String>) -> Result<i32, String> {
    let mut counter = 0;
    let pattern = pattern.ok_or("Pattern is required")?;
    for line in content.lines() {
        if line.contains(&pattern){
            counter += 1;
        }
    }
    Ok(counter)
}

fn search(content: String, pattern: Option<String>) -> Result<String, String> {
    match pattern
}
