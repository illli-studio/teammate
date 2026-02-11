use std::path::Path;
use regex::Regex;
use super::{ParsedTodo, Parser};

pub struct PythonParser;

impl Parser for PythonParser {
    fn extensions(&self) -> Vec<&'static str> {
        vec!["py", "pyx", "pxd"]
    }
    
    fn parse(&self, content: &str, path: &Path) -> Vec<ParsedTodo> {
        let mut todos = Vec::new();
        
        let patterns = vec![
            (Regex::new(r"(?i)#\s*TODO:?\s*(.*)").unwrap(), "#"),
            (Regex::new(r"(?i)#\s*FIXME:?\s*(.*)").unwrap(), "#"),
            (Regex::new(r"(?i)#\s*HACK:?\s*(.*)").unwrap(), "#"),
            (Regex::new(r"(?i)#\s*XXX:?\s*(.*)").unwrap(), "#"),
            (Regex::new(r"(?i)\"\"\".*?TODO:?\s*(.*?)\s*\"\"\"").unwrap(), "\"\"\""),
            (Regex::new(r"(?i)\'\'\'.*?TODO:?\s*(.*?)\s*\'\'\'").unwrap(), "'''"),
        ];
        
        for (line_num, line) in content.lines().enumerate() {
            for (pattern, ptype) in &patterns {
                if let Some(caps) = pattern.captures(line) {
                    if let Some(content_cap) = caps.get(1) {
                        let content = content_cap.as_str().trim().to_string();
                        let tags = extract_python_tags(&content);
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

fn extract_python_tags(content: &str) -> Vec<String> {
    let tag_pattern = Regex::new(r"@(\w+)").unwrap();
    tag_pattern.captures_iter(content)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

fn determine_priority(content: &str) -> Option<String> {
    let lower = content.to_lowercase();
    if lower.contains("urgent") || lower.contains("critical") {
        Some("high".to_string())
    } else if lower.contains("low") || lower.contains("minor") {
        Some("low".to_string())
    } else {
        None
    }
}
