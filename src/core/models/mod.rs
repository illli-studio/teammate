use serde::{Deserialize, Serialize};
use std::time::SystemTime;

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

impl Todo {
    pub fn new(
        id: String,
        content: String,
        file: Option<String>,
        line: Option<usize>,
        priority: String,
        tags: Vec<String>,
        author: Option<String>,
        issue: Option<String>,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        
        Todo {
            id,
            content,
            file,
            line,
            priority: priority.to_lowercase(),
            status: "open".to_string(),
            tags,
            author,
            issue,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: String,
    pub color: Option<String>,
    pub created_at: u64,
}

impl Tag {
    pub fn new(name: String, color: Option<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        
        Tag {
            id: None,
            name,
            color,
            created_at: now,
        }
    }
}

#[derive(Debug)]
pub struct Project {
    pub id: Option<i64>,
    pub path: String,
    pub name: Option<String>,
    pub last_scanned_at: Option<u64>,
}

#[derive(Debug)]
pub struct ScanSession {
    pub id: Option<i64>,
    pub path: String,
    pub started_at: u64,
    pub completed_at: Option<u64>,
    pub files_scanned: usize,
    pub todos_found: usize,
}
