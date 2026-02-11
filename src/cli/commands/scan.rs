use crate::cli::args::ScanArgs;
use std::fs;
use std::path::PathBuf;
use regex::Regex;
use std::collections::HashMap;

pub fn execute(args: &ScanArgs) -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    
    let mut stats = ScanStats {
        files_scanned: 0,
        todos_found: 0,
        by_language: HashMap::new(),
    };
    
    // Parse extensions filter
    let extensions: Option<Vec<String>> = args.language.as_ref().map(|e| {
        e.split(',').map(|s| s.trim().to_string()).collect()
    });
    
    // Collect paths
    let paths: Vec<PathBuf> = if args.path.as_os_str() == "." {
        vec![std::env::current_dir()?]
    } else {
        vec![args.path.clone()]
    };
    
    // Scan each path
    for path in paths {
        let recursive = !args.no_recursive;
        scan_directory(&path, &extensions, &mut stats, recursive)?;
    }
    
    // Output results
    if !args.stats_only {
        println!("\nScan Statistics:");
        println!("Files scanned: {}", stats.files_scanned);
        println!("TODOs found: {}", stats.todos_found);
        println!("\nBy language:");
        for (lang, count) in &stats.by_language {
            println!("  {}: {}", lang, count);
        }
    }
    
    let elapsed = start.elapsed();
    println!("\nScan completed in {:.2}s", elapsed.as_secs_f64());
    
    Ok(())
}

#[derive(Debug)]
struct ScanStats {
    files_scanned: usize,
    todos_found: usize,
    by_language: HashMap<String, usize>,
}

fn scan_directory(
    path: &PathBuf, 
    extensions: &Option<Vec<String>>,
    stats: &mut ScanStats,
    recursive: bool
) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Ok(());
    }
    
    if path.is_file() {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            scan_file(path, ext, extensions, stats)?;
        }
        return Ok(());
    }
    
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() && recursive {
                if should_ignore(&path, &["target", "node_modules", ".git", "build", "dist"]) {
                    continue;
                }
                scan_directory(&path, extensions, stats, recursive)?;
            } else if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    scan_file(&path, ext, extensions, stats)?;
                }
            }
        }
    }
    
    Ok(())
}

fn should_ignore(path: &PathBuf, patterns: &[&str]) -> bool {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        patterns.iter().any(|p| *p == name)
    } else {
        false
    }
}

fn scan_file(
    path: &PathBuf,
    ext: &str,
    extensions: &Option<Vec<String>>,
    stats: &mut ScanStats
) -> Result<(), Box<dyn std::error::Error>> {
    // Check extension filter
    if let Some(exts) = extensions {
        if !exts.iter().any(|e| e == ext) {
            return Ok(());
        }
    }
    
    // Skip binary files by extension
    let binary_exts = ["exe", "bin", "zip", "tar", "gz", "png", "jpg", "jpeg", "gif", "pdf", "doc", "docx"];
    if binary_exts.contains(&ext) {
        return Ok(());
    }
    
    stats.files_scanned += 1;
    
    let content = fs::read_to_string(path)?;
    
    // Define TODO patterns
    let patterns = vec![
        Regex::new(r"(?i)//\s*TODO:?\s*(.*)")?,
        Regex::new(r"(?i)#\s*TODO:?\s*(.*)")?,
        Regex::new(r"(?i)/\/\*\s*TODO:?\s*(.*?)\s*\*/")?,
        Regex::new(r"(?i)<\!--\s*TODO:?\s*(.*?)\s*-->")?,
        Regex::new(r"(?i)\[\s*\]\s*TODO:?\s*(.*)")?,
        Regex::new(r"(?i)TODO:?\s*(.*)")?,
    ];
    
    // Map extension to language name
    let lang = match ext {
        "rs" => "Rust",
        "py" => "Python",
        "js" | "ts" => "JavaScript",
        "java" => "Java",
        "c" | "cpp" | "h" | "hpp" => "C/C++",
        "go" => "Go",
        "rb" => "Ruby",
        "php" => "PHP",
        "sh" | "bash" | "zsh" => "Shell",
        "md" => "Markdown",
        "html" | "htm" => "HTML",
        "css" | "scss" | "sass" => "CSS",
        "json" => "JSON",
        "yaml" | "yml" => "YAML",
        "xml" => "XML",
        _ => ext,
    };
    
    let mut file_todos = 0;
    let mut line_num = 0;
    
    for line in content.lines() {
        line_num += 1;
        for pattern in &patterns {
            if pattern.is_match(line) {
                if let Some(cap) = pattern.captures(line) {
                    if let Some(todo) = cap.get(1) {
                        println!("{}:{} - [TODO] {}", 
                            path.file_name().unwrap_or_default().to_string_lossy(),
                            line_num,
                            todo.as_str().trim()
                        );
                        file_todos += 1;
                    }
                }
            }
        }
    }
    
    if file_todos > 0 {
        *stats.by_language.entry(lang.to_string()).or_insert(0) += file_todos;
        stats.todos_found += file_todos;
    }
    
    Ok(())
}
