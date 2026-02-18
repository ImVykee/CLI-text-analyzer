use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};

#[derive(Parser)]
struct Cli {
    command: Command,
    path: std::path::PathBuf,
    arg1: Option<String>,
    arg2: Option<String>,
}

struct FileStats {
    word_frequency: HashMap<String, i32>,
    total_words: i32,
    longest_word: String,
}

impl FileStats {
    fn print(&self, elem: &str) {
        match elem {
            "all" => {
                self.print_frequent_words();
                println!("For a total of {} words", self.total_words);
                println!("With the longest being {}", self.longest_word);
            }
            "frequent_words" => self.print_frequent_words(),
            "total" => println!("Total amount of words : {}", self.total_words),
            "longest" => println!("Longest word is {}", self.longest_word),
            _ => panic!("Unknown print element"),
        }
    }
    fn print_frequent_words(&self) {
        println!("Frequent words : ");
        let mut sorted_values: Vec<(String, i32)> = self
            .word_frequency
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        sorted_values.sort_by(|a, b| b.1.cmp(&a.1));
        for (word, count) in sorted_values {
            println!("  | {} : {}", word, count);
        }
    }
}

#[derive(clap::ValueEnum, Clone)]
enum Command {
    Search,
    Count,
    Replace,
    Stats,
}

fn main() {
    let statement: Cli = Cli::parse();
    if let Err(err) = run(statement) {
        eprintln!("Error : {}", err);
        std::process::exit(1);
    }
}

fn run(statement: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match statement.command {
        Command::Search => handle_search(&statement.path, statement.arg1.as_deref())?,
        Command::Count => handle_count(&statement.path, statement.arg1.as_deref())?,
        Command::Replace => handle_replace(
            &statement.path,
            statement.arg1.as_deref(),
            statement.arg2.as_deref(),
        )?,
        Command::Stats => stats(&statement.path)?,
    }
    Ok(())
}

fn handle_search(
    path: &std::path::PathBuf,
    pattern: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let pattern = pattern.ok_or("Pattern is required")?;
    let found_at = search(path, pattern)?;
    println!("{}:", pattern);
    for line in found_at {
        println!("   found at line {}", line);
    }
    Ok(())
}

fn handle_count(path: &std::path::PathBuf, pattern: Option<&str>) -> Result<(), String> {
    let pattern = pattern.ok_or("Pattern is required")?;
    let count = count(path, pattern)?;
    println!("{} pattern present {} times", pattern, count);
    Ok(())
}

fn handle_replace(
    path: &std::path::PathBuf,
    pattern: Option<&str>,
    replacement: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let pattern = pattern.ok_or("Pattern is required")?;
    let replacement = replacement.ok_or("Replacement is required")?;
    replace(path, pattern, replacement)?;
    println!("{} pattern replaced with {}", pattern, replacement);
    Ok(())
}

fn count(path: &std::path::PathBuf, pattern: &str) -> Result<i32, String> {
    let mut counter = 0;
    let content = fs::read_to_string(path).expect("File not found");
    for line in content.lines() {
        if line.contains(pattern) {
            counter += 1;
        }
    }
    Ok(counter)
}

fn search(path: &std::path::PathBuf, pattern: &str) -> Result<Vec<i32>, String> {
    let content = fs::read_to_string(path).expect("File not found");
    let mut found_at = Vec::new();
    let mut line_count = 0;
    for line in content.lines() {
        line_count += 1;
        if line.contains(pattern) {
            found_at.push(line_count);
        }
    }
    if found_at.is_empty() {
        return Err(format!("{} not found", pattern));
    }
    Ok(found_at)
}

fn replace(
    path: &std::path::PathBuf,
    pattern: &str,
    replace: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let tempfile = fs::File::create("temp.txt")?;
    let mut writer = BufWriter::new(tempfile);
    for line in reader.lines() {
        let line = line?;
        let replaced = line.replace(pattern, replace);
        writeln!(writer, "{}", replaced)?;
    }
    drop(writer);
    fs::rename("temp.txt", path)?;
    Ok(())
}

fn stats(path: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    match get_filedata(reader) {
        Ok(result) => result.print("all"),
        Err(err) => {
            eprintln!("No words found");
            eprintln!("{}", err);
        }
    };
    Ok(())
}

fn get_filedata(reader: BufReader<fs::File>) -> Result<FileStats, Box<dyn std::error::Error>> {
    let mut frequent_words: HashMap<String, i32> = HashMap::new();
    let mut total = 0;
    let mut longest = String::new();
    for line in reader.lines() {
        let line = line?;
        let words = line.split_whitespace();
        for word in words {
            *frequent_words.entry(word.to_string()).or_insert(0) += 1;
            total += 1;
            if longest.len() < word.len() {
                longest = word.to_string();
            }
        }
    }
    Ok(FileStats {
        word_frequency: frequent_words,
        total_words: total,
        longest_word: longest,
    })
}
