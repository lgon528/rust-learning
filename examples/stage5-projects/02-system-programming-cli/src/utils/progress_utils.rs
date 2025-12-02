use indicatif::{ProgressBar, ProgressStyle, ProgressDrawTarget};
use console::{style, Term};
use std::time::Duration;
use crate::error::Result;

pub struct ProgressManager;

impl ProgressManager {
    pub fn new_bar(size: u64, message: &str) -> ProgressBar {
        let bar = ProgressBar::new(size);

        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-")
        );

        bar.set_message(message.to_string());
        bar.set_draw_target(ProgressDrawTarget::stderr());

        bar
    }

    pub fn new_spinner(message: &str) -> ProgressBar {
        let spinner = ProgressBar::new_spinner();

        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
        );

        spinner.set_message(message.to_string());
        spinner.enable_steady_tick(Duration::from_millis(100));

        spinner
    }

    pub fn print_success(message: &str) {
        println!("{} {}", style("✓").green(), message);
    }

    pub fn print_warning(message: &str) {
        println!("{} {}", style("⚠").yellow(), message);
    }

    pub fn print_error(message: &str) {
        eprintln!("{} {}", style("✗").red(), message);
    }

    pub fn print_info(message: &str) {
        println!("{} {}", style("ℹ").blue(), message);
    }
}

pub struct TablePrinter;

impl TablePrinter {
    pub fn print_table<T>(headers: &[&str], data: &[T])
    where
        T: TableRow,
    {
        if data.is_empty() {
            println!("No data to display");
            return;
        }

        // Calculate column widths
        let mut column_widths: Vec<usize> = headers.iter()
            .map(|h| h.len())
            .collect();

        for row in data {
            for (i, cell) in row.cells().iter().enumerate() {
                if i < column_widths.len() {
                    column_widths[i] = column_widths[i].max(cell.len());
                }
            }
        }

        // Print header
        Self::print_row(headers, &column_widths);
        Self::print_separator(&column_widths);

        // Print data rows
        for row in data {
            Self::print_row(&row.cells(), &column_widths);
        }
    }

    fn print_row(row: &[&str], widths: &[usize]) {
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() {
                print!("| {:<width$} ", cell, width = widths[i]);
            }
        }
        println!("|");
    }

    fn print_separator(widths: &[usize]) {
        for &width in widths {
            print!("+{:-<width$}-", "", width = width + 2);
        }
        println!("+");
    }
}

pub trait TableRow {
    fn cells(&self) -> Vec<&str>;
}

pub struct ColorPrinter;

impl ColorPrinter {
    pub fn green(text: &str) -> String {
        style(text).green().to_string()
    }

    pub fn red(text: &str) -> String {
        style(text).red().to_string()
    }

    pub fn yellow(text: &str) -> String {
        style(text).yellow().to_string()
    }

    pub fn blue(text: &str) -> String {
        style(text).blue().to_string()
    }

    pub fn cyan(text: &str) -> String {
        style(text).cyan().to_string()
    }

    pub fn magenta(text: &str) -> String {
        style(text).magenta().to_string()
    }

    pub fn bold(text: &str) -> String {
        style(text).bold().to_string()
    }

    pub fn dim(text: &str) -> String {
        style(text).dim().to_string()
    }

    pub fn underline(text: &str) -> String {
        style(text).underline().to_string()
    }

    pub fn print_colored_size(size: u64) -> String {
        let size_str = Self::format_bytes(size);

        if size > 1024 * 1024 * 100 { // > 100MB
            Self::red(&size_str)
        } else if size > 1024 * 1024 * 10 { // > 10MB
            Self::yellow(&size_str)
        } else {
            Self::green(&size_str)
        }
    }

    pub fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        }
    }

    pub fn print_file_status(status: &str) -> String {
        match status {
            "New" => Self::green(status),
            "Modified" => Self::yellow(status),
            "Deleted" => Self::red(status),
            _ => Self::dim(status),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(ColorPrinter::format_bytes(512), "512 B");
        assert!(ColorPrinter::format_bytes(1024).contains("KB"));
        assert!(ColorPrinter::format_bytes(1024 * 1024).contains("MB"));
    }

    #[test]
    fn test_color_printer() {
        assert!(!ColorPrinter::green("test").is_empty());
        assert!(!ColorPrinter::red("test").is_empty());
        assert!(!ColorPrinter::yellow("test").is_empty());
    }

    struct TestRow {
        values: Vec<String>,
    }

    impl TableRow for TestRow {
        fn cells(&self) -> Vec<&str> {
            self.values.iter().map(|v| v.as_str()).collect()
        }
    }

    #[test]
    fn test_table_printer() {
        let headers = ["Name", "Age", "City"];
        let data = vec![
            TestRow {
                values: vec!["Alice".to_string(), "30".to_string(), "New York".to_string()],
            },
            TestRow {
                values: vec!["Bob".to_string(), "25".to_string(), "London".to_string()],
            },
        ];

        // This would print a table, but we just test that it doesn't panic
        TablePrinter::print_table(&headers, &data);
    }
}
