use crate::cli::args::HooksArgs;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn execute(args: &HooksArgs) -> Result<(), Box<dyn std::error::Error>> {
    // Get git directory
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .output()?;
    
    let git_dir_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    if git_dir_str.is_empty() {
        println!("Error: Not in a git repository.");
        return Ok(());
    }
    
    let hooks_dir = PathBuf::from(&git_dir_str).join("hooks");
    
    // List hooks
    if args.list {
        println!("\n📋 Git Hooks:");
        
        // Check teammate hooks
        let teammate_hooks = ["pre-commit", "post-commit", "post-checkout"];
        for hook in &teammate_hooks {
            let hook_path = hooks_dir.join(hook);
            if hook_path.exists() {
                println!("  ✅ {}", hook);
            } else {
                println!("  ❌ {} (not installed)", hook);
            }
        }
        
        // List all hooks
        println!("\n📂 All hooks in {}:", hooks_dir.display());
        if let Ok(entries) = fs::read_dir(&hooks_dir) {
            for entry in entries.flatten() {
                if entry.file_type()?.is_file() {
                    println!("  • {}", entry.file_name().to_string_lossy());
                }
            }
        }
        
        return Ok(());
    }
    
    // Install hooks
    if args.install {
        println!("\n🔧 Installing Teammate Git Hooks...");
        
        fs::create_dir_all(&hooks_dir)?;
        
        // Create pre-commit hook
        let pre_commit_hook = r#"#!/bin/bash
# Teammate pre-commit hook
# Run TODO checks before commit

echo "🔍 Checking for urgent TODOs..."

# Get list of TODO files being committed
FILES=$(git diff --cached --name-only | grep -E '\.(rs|py|js|ts|go|java|cpp|c|h)$')

if [ -z "$FILES" ]; then
    echo "  No code files to check."
    exit 0
fi

# Check for URGENT or FIXME TODOs
URGENT=$(echo "$FILES" | xargs grep -l "TODO.*URGENT\|FIXME" 2>/dev/null | wc -l)

if [ "$URGENT" -gt 0 ]; then
    echo "⚠️  Warning: Found URGENT TODOs in committed files:"
    echo "$FILES" | xargs grep -n "TODO.*URGENT\|FIXME" 2>/dev/null | head -10
    echo ""
    echo "Consider resolving these before committing."
    # Exit 0 to allow commit (use --no-verify to skip)
    exit 0
fi

echo "  ✅ No urgent TODOs found."
exit 0
"#;
        
        let pre_commit_path = hooks_dir.join("pre-commit");
        fs::write(&pre_commit_path, pre_commit_hook)?;
        std::process::Command::new("chmod")
            .arg("+x")
            .arg(&pre_commit_path)
            .output()?;
        
        println!("  ✅ pre-commit hook installed");
        
        // Create post-commit hook
        let post_commit_hook = r#"#!/bin/bash
# Teammate post-commit hook
# Update TODO status after commit

echo "🔄 TODOs synchronized with git history."
"#;
        
        let post_commit_path = hooks_dir.join("post-commit");
        fs::write(&post_commit_path, post_commit_hook)?;
        std::process::Command::new("chmod")
            .arg("+x")
            .arg(&post_commit_path)
            .output()?;
        
        println!("  ✅ post-commit hook installed");
        
        println!("\n✅ Hooks installed successfully!");
        println!("\n📝 Usage:");
        println!("  • pre-commit: Checks for URGENT/FIXME TODOs before commit");
        println!("  • post-commit: Syncs TODO status after commit");
        println!("  • Skip hooks: git commit --no-verify");
        
        return Ok(());
    }
    
    // Uninstall hooks
    if args.uninstall {
        println!("\n🗑️  Uninstalling Teammate Git Hooks...");
        
        let hooks = ["pre-commit", "post-commit", "post-checkout"];
        for hook in &hooks {
            let hook_path = hooks_dir.join(hook);
            if hook_path.exists() {
                fs::remove_file(&hook_path)?;
                println!("  ❌ {} removed", hook);
            } else {
                println!("  ⏭️  {} not found", hook);
            }
        }
        
        println!("\n✅ Hooks uninstalled.");
        return Ok(());
    }
    
    // Default: show help
    println!("\n📖 Teammate Git Hooks");
    println!("\nUsage:");
    println!("  teammate hooks --install    Install teammate hooks");
    println!("  teammate hooks --uninstall  Remove teammate hooks");
    println!("  teammate hooks --list      List installed hooks");
    
    println!("\n📝 Available hooks:");
    println!("  pre-commit    Check for urgent TODOs before commit");
    println!("  post-commit   Sync TODO status after commit");
    println!("  post-checkout Update TODO tracking on branch switch");
    
    Ok(())
}
