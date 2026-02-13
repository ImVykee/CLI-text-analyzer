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
    match statement.command {
        Command::Search => println!("search command called"),
        Command::Count => println!("count command called"),
        _ => println!("invalid command")
    }
}
