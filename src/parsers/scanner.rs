use std::fs;
use std::path::PathBuf;
use rayon::prelude::*;
use crate::parsers::{ParserRegistry, ParsedTodo};

pub struct TodoScanner {
    registry: ParserRegistry,
    ignore_patterns: Vec<String>,
}

impl TodoScanner {
    pub fn new() -> Self {
        TodoScanner {
            registry: ParserRegistry::new(),
            ignore_patterns: vec![
                "target".to_string(),
                "node_modules".to_string(),
                ".git".to_string(),
                "build".to_string(),
                "dist".to_string(),
                ".DS_Store".to_string(),
            ],
        }
    }
    
    pub fn scan(&self, paths: &[PathBuf]) -> ScanResult {
        let mut all_todos = Vec::new();
        let mut files_scanned = 0;
        let mut errors = Vec::new();
        
        // Collect all files
        let files: Vec<PathBuf> = paths.iter()
            .flat_map(|p| self.collect_files(p))
            .collect();
        
        // Process files in parallel
        let results: Vec<(PathBuf, Vec<ParsedTodo>)> = files
            .par_iter()
            .filter_map(|file| {
                if self.should_ignore(file) {
                    return None;
                }
                
                if let Ok(content) = fs::read_to_string(file) {
                    let todos = self.registry.parse_file(&content, file);
                    files_scanned += 1;
                    Some((file.clone(), todos))
                } else {
                    Some((file.clone(), Vec::new()))
                }
            })
            .collect();
        
        // Collect results
        for (file, todos) in results {
            for todo in todos {
                all_todos.push(FileTodo {
                    file: file.to_string_lossy().to_string(),
                    line: todo.line,
                    content: todo.content,
                    priority: todo.priority,
                    pattern: todo.pattern,
                });
            }
        }
        
        ScanResult {
            todos: all_todos,
            files_scanned,
            duration_ms: 0, // Could add timing
        }
    }
    
    fn collect_files(&self, path: &PathBuf) -> Vec<PathBuf> {
        let mut files = Vec::new();
        
        if path.is_file() {
            files.push(path.clone());
            return files;
        }
        
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    if !self.should_ignore(&entry_path) {
                        files.extend(self.collect_files(&entry_path));
                    }
                } else if entry_path.is_file() {
                    files.push(entry_path);
                }
            }
        }
        
        files
    }
    
    fn should_ignore(&self, path: &PathBuf) -> bool {
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if self.ignore_patterns.iter().any(|p| p == file_name) {
                return true;
            }
        }
        
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let binary_exts = ["exe", "bin", "zip", "tar", "gz", "png", "jpg", "jpeg", 
                              "gif", "pdf", "doc", "docx", "ico", "woff", "woff2"];
            if binary_exts.contains(&ext) {
                return true;
            }
        }
        
        false
    }
}

#[derive(Debug)]
pub struct ScanResult {
    pub todos: Vec<FileTodo>,
    pub files_scanned: usize,
    pub duration_ms: u64,
}

#[derive(Debug)]
pub struct FileTodo {
    pub file: String,
    pub line: usize,
    pub content: String,
    pub priority: String,
    pub pattern: String,
}
