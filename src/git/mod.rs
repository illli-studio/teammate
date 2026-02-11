use std::process::Command;

pub fn get_current_branch() -> Option<String> {
    let output = Command::new("git")
        .args(&["branch", "--show-current"])
        .output()
        .ok()?;
    
    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch.is_empty() {
        None
    } else {
        Some(branch)
    }
}

pub fn get_last_commit_info() -> Option<(String, String, String)> {
    // Get commit hash
    let hash_output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .ok()?;
    let hash = String::from_utf8_lossy(&hash_output.stdout).trim().to_string();
    
    // Get commit message
    let msg_output = Command::new("git")
        .args(&["log", "-1", "--format=%s"])
        .output()
        .ok()?;
    let message = String::from_utf8_lossy(&msg_output.stdout).trim().to_string();
    
    // Get author
    let author_output = Command::new("git")
        .args(&["log", "-1", "--format=%an"])
        .output()
        .ok()?;
    let author = String::from_utf8_lossy(&author_output.stdout).trim().to_string();
    
    Some((hash, message, author))
}

pub fn get_file_blame(file: &str, line: usize) -> Option<BlameInfo> {
    // Git blame with commit info
    let output = Command::new("git")
        .args(&["blame", "-L", &format!("{},{}", line, line), file])
        .output()
        .ok()?;
    
    let blame_line = String::from_utf8_lossy(&output.stdout);
    
    // Parse blame output
    // Format: "hash author time (line) content"
    // Example: "abc1234 (John 2024-01-15 10:30:00 +0800 10) TODO: fix this"
    
    let parts: Vec<&str> = blame_line.splitn(4).collect();
    if parts.len() >= 4 {
        let hash = parts[0].trim();
        let author_line = parts[1].trim();
        let timestamp_pos = author_line.rfind(')')?;
        let author = &author_line[..timestamp_pos].trim();
        
        // Try to parse timestamp
        let ts_str = &author_line[timestamp_pos..];
        let timestamp = if ts_str.starts_with(')') {
            // Extract timestamp if present
            let ts_inner = &ts_str[1..].trim();
            // Simple parsing - just use author
        }
        
        let content = parts.get(3).map(|s| s.trim().to_string()).unwrap_or_default();
        
        Some(BlameInfo {
            commit_hash: hash.to_string(),
            author: author.to_string(),
            content,
            line: line.to_string(),
        })
    } else {
        None
    }
}

#[derive(Debug)]
pub struct BlameInfo {
    pub commit_hash: String,
    pub author: String,
    pub content: String,
    pub line: String,
}

pub fn get_commit_message(hash: &str) -> Option<String> {
    let output = Command::new("git")
        .args(&["log", "-1", "--format=%s", hash])
        .output()
        .ok()?;
    
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn is_git_repo() -> bool {
    Command::new("git")
        .args(&["rev-parse", "--git-dir"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
