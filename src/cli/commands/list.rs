use crate::cli::args::ListArgs;
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
    pub created_at: u64,
    pub updated_at: u64,
}

pub fn execute(args: &ListArgs) -> Result<(), Box<dyn std::error::Error>> {
    let todo_file = shellexpand::tilde(TODO_FILE).to_string();
    
    // Read TODOs from file
    let todos: Vec<Todo> = if PathBuf::from(&todo_file).exists() {
        let content = fs::read_to_string(&todo_file)?;
        content.lines()
            .filter(|l| !l.trim().is_empty())
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect()
    } else {
        Vec::new()
    };
    
    // Apply filters
    let mut filtered: Vec<Todo> = todos.into_iter().filter(|todo| {
        // Status filter
        if args.open && todo.status != "open" {
            return false;
        }
        
        // Tag filter
        if !args.tag.is_empty() {
            if !args.tag.iter().any(|t| todo.tags.contains(t)) {
                return false;
            }
        }
        
        // Author filter
        if let Some(author) = &args.author {
            if todo.author.as_ref().map(|a| a != author).unwrap_or(true) {
                return false;
            }
        }
        
        // File filter
        if let Some(file) = &args.file {
            if todo.file.as_ref().map(|f| f != file.to_string_lossy().as_ref()).unwrap_or(false) {
                return false;
            }
        }
        
        true
    }).collect();
    
    // Sort
    if let Some(sort) = &args.sort {
        match sort {
            crate::cli::args::SortField::Priority => {
                filtered.sort_by(|a, b| {
                    let priority_order = |p: &str| match p.to_lowercase().as_str() {
                        "high" => 0,
                        "medium" => 1,
                        "low" => 2,
                        _ => 3,
                    };
                    priority_order(&a.priority).cmp(&priority_order(&b.priority))
                });
                if args.ascending {
                    filtered.reverse();
                }
            }
            crate::cli::args::SortField::Created => {
                filtered.sort_by(|a, b| a.created_at.cmp(&b.created_at));
                if args.ascending {
                    filtered.reverse();
                }
            }
            crate::cli::args::SortField::File => {
                filtered.sort_by(|a, b| {
                    let file_a = a.file.as_deref().unwrap_or("");
                    let file_b = b.file.as_deref().unwrap_or("");
                    file_a.cmp(file_b)
                });
            }
            crate::cli::args::SortField::Updated => {
                filtered.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));
                if args.ascending {
                    filtered.reverse();
                }
            }
        }
    }
    
    // Limit
    if let Some(limit) = args.limit {
        filtered.truncate(limit);
    }
    
    // Display
    println!("\n{}", "=".repeat(100));
    println!("{:^5} │ {:^8} │ {:^10} │ {:^15} │ {}", 
        "ID", "Priority", "Status", "Tags", "Content");
    println!("{}", "=".repeat(100));
    
    for todo in &filtered {
        let tags_str = if todo.tags.is_empty() {
            "-".to_string()
        } else {
            todo.tags.join(", ")
        };
        let file_str = todo.file.as_deref().unwrap_or("-");
        
        println!("{:^5} │ {:^8} │ {:^10} │ {:^15} │ [{}:{}] {}",
            &todo.id[..8.min(todo.id.len())],
            &todo.priority.to_uppercase(),
            &todo.status.to_uppercase(),
            tags_str,
            file_str,
            todo.line.map(|l| l.to_string()).unwrap_or_else(|| "-".to_string()),
            &todo.content[..std::cmp::min(80, todo.content.len())]
        );
    }
    
    println!("{}", "=".repeat(100));
    println!("Total: {} TODOs", filtered.len());
    
    Ok(())
}
