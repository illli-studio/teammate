use crate::cli::args::AddArgs;
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

pub fn execute(args: &AddArgs) -> Result<(), Box<dyn std::error::Error>> {
    let todo_file = shellexpand::tilde(TODO_FILE).to_string();
    
    // Ensure directory exists
    if let Some(parent) = PathBuf::from(&todo_file).parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Generate unique ID using timestamp + random
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis() as u64;
    let random: u32 = rand::RngCore::next_u32(&mut rand::thread_rng());
    let id = format!("{:x}{:x}", timestamp, random);
    
    // Parse priority
    let priority = match args.priority {
        Some(ref p) => {
            match p {
                crate::cli::args::Priority::Low => "low".to_string(),
                crate::cli::args::Priority::Medium => "medium".to_string(),
                crate::cli::args::Priority::High => "high".to_string(),
            }
        }
        None => "medium".to_string(),
    };
    
    // Create TODO
    let todo = Todo {
        id: id[..12].to_string(),
        content: args.content.clone(),
        file: args.file.as_ref().map(|p| p.to_string_lossy().to_string()),
        line: args.line,
        priority,
        status: "open".to_string(),
        tags: args.tag.clone(),
        author: args.author.clone(),
        issue: args.issue.clone(),
        created_at: timestamp,
        updated_at: timestamp,
    };
    
    // Append to file
    let json = serde_json::to_string(&todo)?;
    fs::write(&todo_file, format!("{}\n", json))?;
    
    println!("TODO added successfully!");
    println!("ID: {}", todo.id);
    println!("Content: {}", todo.content);
    println!("Priority: {}", todo.priority);
    println!("Tags: {:?}", todo.tags);
    
    Ok(())
}
