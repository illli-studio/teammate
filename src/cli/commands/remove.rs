use crate::cli::args::RemoveArgs;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

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

pub fn execute(args: &RemoveArgs) -> Result<(), Box<dyn std::error::Error>> {
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
    
    // Find TODO by ID (partial match allowed)
    let target_id = args.id.trim();
    let mut found = None;
    for todo in &todos {
        if todo.id.starts_with(target_id) {
            found = Some(todo.clone());
            break;
        }
    }
    
    let todo = match found {
        Some(t) => t,
        None => {
            println!("TODO not found: {}", target_id);
            return Ok(());
        }
    };
    
    // Confirm deletion
    if !args.force {
        println!("Delete this TODO?");
        println!("ID: {}", todo.id);
        println!("Content: {}", todo.content);
        println!("Priority: {}", todo.priority);
        print!("\nConfirm (y/N): ");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        if input.trim().to_lowercase() != "y" {
            println!("Cancelled.");
            return Ok(());
        }
    }
    
    // Remove TODO
    let before_count = todos.len();
    todos.retain(|t| !t.id.starts_with(target_id));
    let after_count = todos.len();
    
    if before_count == after_count {
        println!("TODO not found: {}", target_id);
        return Ok(());
    }
    
    // Rewrite file
    let new_content: String = todos.iter()
        .map(|t| format!("{}\n", serde_json::to_string(t).unwrap()))
        .collect();
    fs::write(&todo_file, new_content)?;
    
    println!("TODO deleted successfully!");
    
    Ok(())
}
