use std::fs;
use std::path::PathBuf;

const GIT_HOOKS_DIR: &str = ".git/hooks";

pub struct HooksManager;

impl HooksManager {
    pub fn new() -> Self {
        HooksManager
    }
    
    pub fn install(&self, hook_type: HookType) -> Result<(), String> {
        let hooks_dir = PathBuf::from(GIT_HOOKS_DIR);
        
        if !hooks_dir.exists() {
            return Err("Not a git repository".to_string());
        }
        
        // Create hook content
        let hook_content = match hook_type {
            HookType::PreCommit => self.pre_commit_hook(),
            HookType::PostCommit => self.post_commit_hook(),
            HookType::PostCheckout => self.post_checkout_hook(),
        };
        
        // Write hook file
        let hook_path = hooks_dir.join(hook_type.filename());
        fs::write(&hook_path, hook_content)
            .map_err(|e| format!("Failed to write hook: {}", e))?;
        
        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&hook_path)
                .map_err(|e| format!("Failed to get permissions: {}", e))?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook_path, perms)
                .map_err(|e| format!("Failed to set permissions: {}", e))?;
        }
        
        Ok(format!("{} hook installed", hook_type.name()))
    }
    
    pub fn uninstall(&self, hook_type: HookType) -> Result<(), String> {
        let hook_path = PathBuf::from(GIT_HOOKS_DIR).join(hook_type.filename());
        
        if hook_path.exists() {
            fs::remove_file(&hook_path)
                .map_err(|e| format!("Failed to remove hook: {}", e))?;
        }
        
        Ok(format!("{} hook uninstalled", hook_type.name()))
    }
    
    pub fn list(&self) -> Vec<String> {
        let hooks_dir = PathBuf::from(GIT_HOOKS_DIR);
        if !hooks_dir.exists() {
            return Vec::new();
        }
        
        let mut hooks = Vec::new();
        if let Ok(entries) = fs::read_dir(&hooks_dir) {
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                    hooks.push(entry.file_name().to_string_lossy().to_string());
                }
            }
        }
        hooks
    }
    
    fn pre_commit_hook(&self) -> String {
        r#"#!/bin/bash
# Teammate pre-commit hook
# Checks for URGENT TODOs before commit

echo "🔍 Teammate: Checking for urgent TODOs..."

# Find TODO/FIXME/URGENT in staged files
urgency_count=$(git diff --cached --grep="URGENT\|FIXME" 2>/dev/null | grep -c "URGENT\|FIXME" || echo "0")

if [ "$urgency_count" -gt 0 ]; then
    echo "⚠️  Found urgent TODOs in commit message!"
    echo "Consider resolving them before committing."
fi

# Scan staged source files
echo "📝 Scanning staged files for TODOs..."
todo_count=$(git diff --cached --name-only 2>/dev/null | grep -E "\.(rs|py|js|ts|go|java|cpp|c|h)$" | xargs grep -l "TODO\|FIXME\|HACK" 2>/dev/null | wc -l || echo "0")

if [ "$todo_count" -gt 0 ]; then
    echo "📋 Found TODOs in $todo_count staged files:"
    git diff --cached --name-only 2>/dev/null | grep -E "\.(rs|py|js|ts|go|java|cpp|c|h)$" | xargs grep -n "TODO\|FIXME\|HACK" 2>/dev/null | head -10
    echo "..."
fi

echo "✅ Pre-commit check complete"
exit 0
"#.to_string()
    }
    
    fn post_commit_hook(&self) -> String {
        r#"#!/bin/bash
# Teammate post-commit hook
# Updates TODO status tracking

echo "🔄 Teammate: Commit recorded"

# Get last commit hash
commit_hash=$(git rev-parse HEAD)

echo "📝 Commit $commit_hash recorded"

exit 0
"#.to_string()
    }
    
    fn post_checkout_hook(&self) -> String {
        r#"#!/bin/bash
# Teammate post-checkout hook
# Updates TODO tracking for new branch

echo "🔄 Teammate: Branch switched"

# Get current branch
branch=$(git branch --show-current 2>/dev/null || echo "detached")

if [ -n "$branch" ]; then
    echo "📍 Now on branch: $branch"
fi

exit 0
"#.to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HookType {
    PreCommit,
    PostCommit,
    PostCheckout,
}

impl HookType {
    pub fn name(&self) -> &'static str {
        match self {
            HookType::PreCommit => "Pre-commit",
            HookType::PostCommit => "Post-commit",
            HookType::PostCheckout => "Post-checkout",
        }
    }
    
    pub fn filename(&self) -> &'static str {
        match self {
            HookType::PreCommit => "pre-commit",
            HookType::PostCommit => "post-commit",
            HookType::PostCheckout => "post-checkout",
        }
    }
}
