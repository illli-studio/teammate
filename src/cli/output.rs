use crate::cli::args::OutputFormat;

pub fn format_output<T: std::fmt::Display>(
    items: &[T], 
    format: OutputFormat
) -> String {
    match format {
        OutputFormat::Json => format_json(items),
        OutputFormat::Compact => format_compact(items),
        OutputFormat::Csv => format_csv(items),
        OutputFormat::Table | OutputFormat::Table => format_table(items),
    }
}

fn format_json<T: std::fmt::Display>(items: &[T]) -> String {
    serde_json::to_string_pretty(items).unwrap_or_else(|_| "[]".to_string())
}

fn format_compact<T: std::fmt::Display>(items: &[T]) -> String {
    items.iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_csv<T: std::fmt::Display>(items: &[T]) -> String {
    items.iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn format_table<T: std::fmt::Display>(items: &[T]) -> String {
    // Simple table formatting
    let mut output = String::new();
    for item in items {
        output.push_str(&format!("{}\n", item));
    }
    output
}
