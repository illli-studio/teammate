use crate::cli::args::ConfigArgs;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "~/.teammate/config.yaml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub git_enabled: bool,
    pub default_priority: String,
    pub scan_depth: usize,
    pub ignore_patterns: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            git_enabled: true,
            default_priority: "medium".to_string(),
            scan_depth: 10,
            ignore_patterns: vec![
                "target".to_string(),
                "node_modules".to_string(),
                ".git".to_string(),
                "build".to_string(),
                "dist".to_string(),
            ],
        }
    }
}

pub fn execute(args: &ConfigArgs) -> Result<(), Box<dyn std::error::Error>> {
    let config_file = shellexpand::tilde(CONFIG_FILE).to_string();
    
    // Ensure directory exists
    if let Some(parent) = PathBuf::from(&config_file).parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Read or create config
    let config: Config = if PathBuf::from(&config_file).exists() {
        let content = fs::read_to_string(&config_file)?;
        serde_yaml::from_str(&content)?
    } else {
        Config::default()
    };
    
    // Show config
    if args.show {
        println!("Current configuration:");
        println!("  git_enabled: {}", config.git_enabled);
        println!("  default_priority: {}", config.default_priority);
        println!("  scan_depth: {}", config.scan_depth);
        println!("  ignore_patterns: {:?}", config.ignore_patterns);
        return Ok(());
    }
    
    // Reset config
    if args.reset {
        let default = Config::default();
        let yaml = serde_yaml::to_string(&default)?;
        fs::write(&config_file, yaml)?;
        println!("Config reset to defaults.");
        return Ok(());
    }
    
    // Show help if no action
    println!("Teammate Configuration");
    println!("\nUsage:");
    println!("  teammate config --show     Show current config");
    println!("  teammate config --reset    Reset to defaults");
    println!("  teammate config --set KEY=VALUE  Set config value");
    println!("\nConfig file: {}", config_file);
    
    Ok(())
}
