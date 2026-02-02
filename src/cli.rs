use crate::parser::parse_range;
use clap::{ArgGroup, Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "slice", version, about = "Slice text by lines/bytes/middle")]
// Byte Mode (Exclusive)
// If you pick --bytes, you cannot use line-based logic or line-context.
#[command(group(
    ArgGroup::new("byte_mode")
        .args(&["bytes"])
        .conflicts_with_all(&["head", "tail", "middle", "lines", "context", "after_context", "before_context", "follow", "after_match"])
))]
// Streaming vs Static
// You cannot calculate the "Middle" of a stream that never ends (--follow).
#[command(group(
    ArgGroup::new("infinite_stream")
        .args(&["follow"])
        .conflicts_with_all(&["middle"])
))]
pub struct Args {
    // --- INPUT ---
    /// File(s) to process. Defaults to Stdin if empty.
    #[arg(value_name = "FILE")]
    pub files: Vec<PathBuf>,

    // --- LINE SELECTORS (Composable) ---
    /// First N lines
    #[arg(long)]
    pub head: Option<usize>,

    /// Last N lines
    #[arg(short = 't', long)]
    pub tail: Option<usize>,

    /// Middle N lines (calculated from total line count)
    #[arg(short = 'm', long)]
    pub middle: Option<usize>,

    /// Specific range (e.g., "500-600", "500:600", "500-")
    #[arg(short = 'n', long, value_parser = parse_range)]
    pub lines: Option<std::ops::Range<usize>>,

    /// Start after the first line matching this REGEX
    #[arg(long, value_name = "PATTERN")]
    pub after_match: Option<String>,

    // --- BYTE SELECTOR (Exclusive) ---
    /// Byte range (e.g., "0-1024")
    #[arg(short = 'c', long)]
    pub bytes: Option<String>,

    // --- CONTEXT ---
    /// Add N lines before and after matches
    #[arg(short = 'C', long, conflicts_with_all = ["before_context", "after_context"])]
    pub context: Option<usize>,

    /// Add N lines after matches
    #[arg(short = 'A', long)]
    pub after_context: Option<usize>,

    /// Add N lines before matches
    #[arg(short = 'B', long)]
    pub before_context: Option<usize>,

    // --- DISPLAY & BEHAVIOR ---
    /// Show line numbers in output
    #[arg(short = 'N', long)]
    pub show_line_numbers: bool,

    /// Colorize output
    #[arg(long, value_enum, default_value_t = ColorMode::Auto)]
    pub color: ColorMode,

    /// Suppress file name headers when slicing multiple files
    #[arg(short, long)]
    pub quiet: bool,

    /// Output appended data as the file grows
    #[arg(short, long)]
    pub follow: bool,
}

// Color Handling
#[derive(Clone, Debug, ValueEnum)]
pub enum ColorMode {
    /// Use colors if output is a TTY
    Auto,
    /// Always use colors (good for piping to `less -R`)
    Always,
    /// Never use colors
    Never,
}
