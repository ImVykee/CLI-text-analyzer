use clap::Parser;
use std::fs;

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
    let content = fs::read_to_string(statement.path).expect("File not found");
    let result = match statement.command {
        Command::Search => search(&content, statement.arg1.as_deref()).unwrap(),
        Command::Count => count(&content, statement.arg1.as_deref())
            .unwrap()
            .to_string(),
    };
    println!("{}", result);
}

fn count(content: &str, pattern: Option<&str>) -> Result<i32, String> {
    let mut counter = 0;
    let pattern = pattern.ok_or("Pattern is required")?;
    for line in content.lines() {
        if line.contains(pattern) {
            counter += 1;
        }
    }
    Ok(counter)
}

fn search(content: &str, pattern: Option<&str>) -> Result<String, String> {
    let mut line_count = 0;
    let pattern = pattern.ok_or("Pattern is required")?;
    for line in content.lines() {
        line_count += 1;
        if line.contains(pattern) {
            break;
        }
    }
    Ok(format!("{} found at line {}", pattern, line_count))
}
