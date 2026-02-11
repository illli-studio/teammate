use serde::{Deserialize, Serialize};

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
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        
        Todo {
            id,
            content,
            file,
            line,
            priority,
            status: "open".to_string(),
            tags,
            author,
            issue,
            created_at: now,
            updated_at: now,
        }
    }
}
