use std::path::Path;
use regex::Regex;
use serde::Serialize;

pub mod traits;
pub mod rust;
pub mod python;
pub mod javascript;

#[derive(Debug, Serialize, Clone)]
pub struct ParsedTodo {
    pub content: String,
    pub line: usize,
    pub pattern: String,
    pub tags: Vec<String>,
    pub priority: Option<String>,
}

pub trait Parser: Send + Sync {
    fn extensions(&self) -> Vec<&'static str>;
    fn parse(&self, content: &str, path: &Path) -> Vec<ParsedTodo>;
}

// Common TODO patterns across languages
pub fn get_common_patterns() -> Vec<(Regex, &'static str)> {
    vec![
        (Regex::new(r"(?i)//\s*TODO:?\s*(.*)").unwrap(), "//"),
        (Regex::new(r"(?i)#\s*TODO:?\s*(.*)").unwrap(), "#"),
        (Regex::new(r"(?i)/\/\*\s*TODO:?\s*(.*?)\s*\*/").unwrap(), "/*"),
        (Regex::new(r"(?i)<\!--\s*TODO:?\s*(.*?)\s*-->").unwrap(), "<!--"),
        (Regex::new(r"(?i)\[\s*\]\s*TODO:?\s*(.*)").unwrap(), "[ ]"),
        (Regex::new(r"(?i)TODO:?\s*(.*)").unwrap(), "plain"),
        // FIXME, HACK, XXX patterns
        (Regex::new(r"(?i)//\s*FIXME:?\s*(.*)").unwrap(), "//"),
        (Regex::new(r"(?i)#\s*FIXME:?\s*(.*)").unwrap(), "#"),
        (Regex::new(r"(?i)//\s*HACK:?\s*(.*)").unwrap(), "//"),
        (Regex::new(r"(?i)#\s*HACK:?\s*(.*)").unwrap(), "#"),
        (Regex::new(r"(?i)//\s*XXX:?\s*(.*)").unwrap(), "//"),
        (Regex::new(r"(?i)#\s*XXX:?\s*(.*)").unwrap(), "#"),
    ]
}

pub fn extract_tags(content: &str) -> Vec<String> {
    let tag_pattern = Regex::new(r"@(\w+)").unwrap();
    tag_pattern.captures_iter(content)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

pub fn determine_priority(content: &str) -> Option<String> {
    let lower = content.to_lowercase();
    if lower.contains("urgent") || lower.contains("critical") {
        Some("high".to_string())
    } else if lower.contains("low") || lower.contains("minor") {
        Some("low".to_string())
    } else {
        None
    }
}
