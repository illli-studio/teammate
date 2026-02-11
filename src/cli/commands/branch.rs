use crate::cli::args::BranchArgs;
use std::process::Command;

pub fn execute(_args: &BranchArgs) -> Result<(), Box<dyn std::error::Error>> {
    // Get current branch
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()?;
    
    let branch_binding = String::from_utf8_lossy(&output.stdout);
    let current_branch = branch_binding.trim().to_string();
    
    if current_branch.is_empty() {
        println!("Not in a git repository or no branch found.");
        return Ok(());
    }
    
    println!("\n{}", "=".repeat(60));
    println!("Current Branch: {}", current_branch);
    println!("{}", "=".repeat(60));
    
    // List all branches
    println!("\n📚 All branches:");
    let output = Command::new("git")
        .arg("branch")
        .arg("-a")
        .output()?;
    
    let branches = String::from_utf8_lossy(&output.stdout);
    for branch in branches.lines() {
        let clean_branch = branch.trim().trim_start_matches('*').trim();
        let marker = if clean_branch == current_branch { " *" } else { "  " };
        println!("{}{}", marker, clean_branch);
    }
    
    // Branch TODO summary
    println!("\n📊 Branch TODO Summary:");
    println!("  Note: Branch-specific TODO tracking requires database integration.");
    println!("  Use 'teammate scan' to find TODOs in this branch.");
    
    // List recent commits with TODO-related changes
    println!("\n📝 Recent commits with TODO keywords:");
    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .arg("-10")
        .arg("--grep=TODO")
        .output()?;
    
    let commits = String::from_utf8_lossy(&output.stdout);
    if commits.is_empty() {
        println!("  No recent TODO-related commits found.");
    } else {
        for commit in commits.lines().take(5) {
            println!("  {}", commit);
        }
    }
    
    println!("\n{}", "=".repeat(60));
    
    Ok(())
}
