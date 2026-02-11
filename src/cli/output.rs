use crate::cli::args::OutputFormat;

pub fn format_output<T: std::fmt::Display>(
    _items: &[T], 
    _format: OutputFormat
) -> String {
    // Simplified output - actual implementation needs proper serde derive
    format!("[{} items]", _items.len())
}
