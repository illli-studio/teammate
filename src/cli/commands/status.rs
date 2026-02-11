use crate::cli::args::StatusArgs;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

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

pub fn execute(args: &StatusArgs) -> Result<(), Box<dyn std::error::Error>> {
    let todo_file = shellexpand::tilde(TODO_FILE).to_string();
    
    if !PathBuf::from(&todo_file).exists() {
        println!("No TODOs found.");
        return Ok(());
    }
    
    // Read all TODOs
    let content = fs::read_to_string(&todo_file)?;
    let mut todos: Vec<Todo> = content.lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    
    // Find TODO by ID
    let target_id = args.id.trim();
    let mut found_index = None;
    for (i, todo) in todos.iter().enumerate() {
        if todo.id.starts_with(target_id) {
            found_index = Some(i);
            break;
        }
    }
    
    let index = match found_index {
        Some(i) => i,
        None => {
            println!("TODO not found: {}", target_id);
            return Ok(());
        }
    };
    
    // Parse new status
    let new_status = match &args.status {
        crate::cli::args::TodoStatus::Open => "open",
        crate::cli::args::TodoStatus::InProgress => "in_progress",
        crate::cli::args::TodoStatus::Resolved => "resolved",
    };
    
    // Update status
    let updated_at = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis() as u64;
    
    todos[index].status = new_status.to_string();
    todos[index].updated_at = updated_at;
    
    // Rewrite file
    let new_content: String = todos.iter()
        .map(|t| format!("{}\n", serde_json::to_string(t).unwrap()))
        .collect();
    fs::write(&todo_file, new_content)?;
    
    println!("Status updated successfully!");
    println!("ID: {}", todos[index].id);
    println!("Content: {}", todos[index].content);
    println!("Status: {} → {}", todos[index].status, new_status);
    
    Ok(())
}
