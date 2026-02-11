use crate::cli::args::BlameArgs;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::process::Command;

const TODO_FILE: &str = "~/.teammate/todos.jsonl";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: String,
    pub content: String,
    pub file: Option<String>,
    pub line: Option<usize>,
    pub priority: String,
    pub status: String,
    pub tags: Vec<String>,
    pub author: Option<String>,
    pub issue: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

pub fn execute(args: &BlameArgs) -> Result<(), Box<dyn std::error::Error>> {
    let todo_file = shellexpand::tilde(TODO_FILE).to_string();
    
    if !PathBuf::from(&todo_file).exists() {
        println!("No TODOs found.");
        return Ok(());
    }
    
    // Read TODOs
    let content = fs::read_to_string(&todo_file)?;
    let todos: Vec<Todo> = content.lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    
    // Find TODO by ID
    let target_id = args.id.trim();
    let mut found_todo = None;
    for todo in &todos {
        if todo.id.starts_with(target_id) {
            found_todo = Some(todo.clone());
            break;
        }
    }
    
    let todo = match found_todo {
        Some(t) => t,
        None => {
            println!("TODO not found: {}", target_id);
            return Ok(());
        }
    };
    
    // Display TODO info
    println!("\n{}", "=".repeat(70));
    println!("TODO: {}", todo.id);
    println!("Content: {}", todo.content);
    println!("Priority: {} | Status: {}", todo.priority.to_uppercase(), todo.status.to_uppercase());
    println!("Tags: {:?}", todo.tags);
    println!("{}", "=".repeat(70));
    
    // Try git blame if file is available
    if let Some(ref file) = todo.file {
        let path = PathBuf::from(file);
        if path.exists() {
            println!("\n📝 Git Blame for {}:{}", file, todo.line.map(|l| format!(":{}", l)).unwrap_or_default());
            
            let line_arg = match todo.line {
                Some(l) => format!("{},{}", l, l),
                None => String::from("1,1"),
            };
            
            let mut cmd = Command::new("git");
            cmd.arg("blame");
            if args.verbose {
                cmd.arg("-p"); // Porcelain format
            } else {
                cmd.arg("-s"); // Short format
            }
            cmd.arg("-L");
            cmd.arg(&line_arg);
            cmd.arg(file);
            
            let output = cmd.output()?;
            let blame_output = String::from_utf8_lossy(&output.stdout);
            
            if !blame_output.is_empty() {
                println!("\n{}", blame_output);
            } else {
                println!("\nNo git blame information available.");
            }
        } else {
            println!("\n⚠️ File not found: {}", file);
        }
    } else {
        println!("\n📌 Manual TODO (not linked to file)");
    }
    
    // Display metadata
    println!("\n📅 Created: {}", format_timestamp(todo.created_at));
    println!("📅 Updated: {}", format_timestamp(todo.updated_at));
    
    if let Some(ref author) = todo.author {
        println!("👤 Author: {}", author);
    }
    
    Ok(())
}

fn format_timestamp(ts: u64) -> String {
    let default_dt = chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap();
    let dt = chrono::NaiveDateTime::from_timestamp_opt(ts as i64, 0)
        .unwrap_or(default_dt);
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}
