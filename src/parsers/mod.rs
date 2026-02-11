use std::path::Path;
use regex::Regex;
use serde::Serialize;

pub mod traits;
pub mod rust;
pub mod python;
pub mod javascript;
pub mod scanner;

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

pub struct ParserRegistry {
    parsers: Vec<Box<dyn Parser>>,
}

impl ParserRegistry {
    pub fn new() -> Self {
        let mut registry = ParserRegistry {
            parsers: Vec::new(),
        };
        
        // Register language-specific parsers
        registry.parsers.push(Box::new(rust::RustParser));
        registry.parsers.push(Box::new(python::PythonParser));
        registry.parsers.push(Box::new(javascript::JavaScriptParser));
        
        registry
    }
    
    pub fn parse_file(&self, content: &str, path: &Path) -> Vec<ParsedTodo> {
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();
        
        // Find parser for extension
        for parser in &self.parsers {
            if parser.extensions().contains(&ext) {
                return parser.parse(content, path);
            }
        }
        
        // Fall back to common patterns
        self.parse_common(content)
    }
    
    fn parse_common(&self, content: &str) -> Vec<ParsedTodo> {
        let mut todos = Vec::new();
        let patterns = get_common_patterns();
        
        for (line_num, line) in content.lines().enumerate() {
            for (pattern, ptype) in &patterns {
                if let Some(caps) = pattern.captures(line) {
                    if let Some(content_cap) = caps.get(1) {
                        let content = content_cap.as_str().trim().to_string();
                        let tags = extract_tags(&content);
                        let priority = determine_priority(&content);
                        
                        todos.push(ParsedTodo {
                            content,
                            line: line_num + 1,
                            pattern: ptype.to_string(),
                            tags,
                            priority,
                        });
                    }
                }
            }
        }
        
        todos
    }
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
