use crate::cli::args::InitArgs;
use std::fs;
use std::path::PathBuf;

const CONFIG_DIR: &str = "~/.teammate";
const CONFIG_FILE: &str = "~/.teammate/config.yaml";
const TODOS_FILE: &str = "~/.teammate/todos.jsonl";

pub fn execute(args: &InitArgs) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = shellexpand::tilde(CONFIG_DIR).to_string();
    let config_file = shellexpand::tilde(CONFIG_FILE).to_string();
    let todos_file = shellexpand::tilde(TODOS_FILE).to_string();
    
    let config_path = PathBuf::from(&config_file);
    let config_dir_path = PathBuf::from(&config_dir);
    
    // Check if already initialized
    if config_path.exists() && !args.force {
        println!("Teammate is already initialized.");
        println!("Config file: {}", config_file);
        println!("TODOs file: {}", todos_file);
        return Ok(());
    }
    
    // Create directory
    println!("Initializing teammate...");
    fs::create_dir_all(&config_dir_path)?;
    
    // Create default config
    let default_config = r#"# Teammate Configuration
git_enabled: true
default_priority: medium
scan_depth: 10
ignore_patterns:
  - target
  - node_modules
  - .git
  - build
  - dist
"#;
    fs::write(&config_file, default_config)?;
    
    // Create empty todos file
    fs::write(&todos_file, "")?;
    
    println!("Teammate initialized successfully!");
    println!("\nFiles created:");
    println!("  Config: {}", config_file);
    println!("  TODOs:  {}", todos_file);
    println!("\nNext steps:");
    println!("  teammate scan          # Scan for TODOs");
    println!("  teammate add \"...\"    # Add new TODO");
    println!("  teammate list          # List TODOs");
    
    Ok(())
}
