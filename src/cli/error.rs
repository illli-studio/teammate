use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum CliError {
    NotFound(String),
    InvalidInput(String),
    IoError(String),
    ParseError(String),
    GitError(String),
    DatabaseError(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::NotFound(msg) => write!(f, "Not found: {}", msg),
            CliError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CliError::IoError(msg) => write!(f, "IO error: {}", msg),
            CliError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CliError::GitError(msg) => write!(f, "Git error: {}", msg),
            CliError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl Error for CliError {}

pub type Result<T> = std::result::Result<T, CliError>;
