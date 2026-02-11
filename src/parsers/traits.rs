use std::path::Path;
use super::ParsedTodo;

pub trait Parser: Send + Sync {
    fn extensions(&self) -> Vec<&'static str>;
    fn parse(&self, content: &str, path: &Path) -> Vec<ParsedTodo>;
    
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap_or("unknown")
    }
}
