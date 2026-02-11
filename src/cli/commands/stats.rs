use crate::cli::args::StatsArgs;
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

pub fn execute(_args: &StatsArgs) -> Result<(), Box<dyn std::error::Error>> {
    let todo_file = shellexpand::tilde(TODO_FILE).to_string();
    
    // Read TODOs
    let todos: Vec<Todo> = if PathBuf::from(&todo_file).exists() {
        let content = fs::read_to_string(&todo_file)?;
        content.lines()
            .filter(|l| !l.trim().is_empty())
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect()
    } else {
        Vec::new()
    };
    
    let total = todos.len();
    if total == 0 {
        println!("No TODOs found.");
        return Ok(());
    }
    
    // Calculate statistics
    let mut by_priority: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut by_status: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut by_tag: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    
    for todo in &todos {
        *by_priority.entry(todo.priority.clone()).or_insert(0) += 1;
        *by_status.entry(todo.status.clone()).or_insert(0) += 1;
        for tag in &todo.tags {
            *by_tag.entry(tag.clone()).or_insert(0) += 1;
        }
    }
    
    let open_count = by_status.get("open").unwrap_or(&0);
    let resolved_count = by_status.get("resolved").unwrap_or(&0);
    let completion_rate = if total > 0 {
        (*resolved_count * 100) / total
    } else {
        0
    };
    
    // Display statistics
    println!("\n{}", "=".repeat(60));
    println!("Teammate Statistics");
    println!("{}", "=".repeat(60));
    println!("\n📊 Overview:");
    println!("  Total TODOs: {}", total);
    println!("  Completion rate: {}% ({} resolved)", completion_rate, resolved_count);
    println!("  Open TODOs: {}", open_count);
    
    println!("\n📈 By Priority:");
    for (priority, count) in &by_priority {
        let pct = (*count * 100) / total;
        let bar = "█".repeat(pct / 5);
        println!("  {:8}: {:3} ({:2}%) {}", 
            priority.to_uppercase(), count, pct, bar);
    }
    
    println!("\n📋 By Status:");
    for (status, count) in &by_status {
        let pct = (*count * 100) / total;
        let bar = "█".repeat(pct / 5);
        println!("  {:8}: {:3} ({:2}%) {}", 
            status.to_uppercase(), count, pct, bar);
    }
    
    println!("\n🏷️ By Tag (top 10):");
    let mut tags: Vec<_> = by_tag.iter().collect();
    tags.sort_by(|a, b| b.1.cmp(a.1));
    for (tag, count) in tags.iter().take(10) {
        let pct = (*count * 100) / total;
        println!("  {:15}: {:3} ({:2}%)", tag, count, pct);
    }
    
    println!("\n{}", "=".repeat(60));
    
    Ok(())
}
