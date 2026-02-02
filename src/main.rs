mod utils;
mod cli;
mod parser;

use clap::Parser;
use crate::cli::Args;
use crate::utils::{count_lines, resolve_ranges, slice_file};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.files.is_empty() {
        // Standard Unix behavior: if no args, maybe show help or read stdin
        // For now, let's just print help usage
        use clap::CommandFactory; // Trait needed to print help
        Args::command().print_help()?;
        return Ok(());
    }

    // Loop through files
    for (i, path) in args.files.iter().enumerate() {
        // LOGIC: Print header "==> filename <==" if:
        // 1. We have more than 1 file
        // 2. AND the user didn't ask for --quiet
        if args.files.len() > 1 && !args.quiet {
            // If it's not the first file, add a blank line before the header (like 'head' does)
            if i > 0 { println!(); }
            println!("==> {} <==", path.display());
        }

        let needs_count = args.tail.is_some() || args.middle.is_some();
        let total_lines = if needs_count {
            Some(count_lines(path)?)
        } else {
            None
        };

        let ranges = resolve_ranges(&args, total_lines)?;

        if !ranges.is_empty() {
            slice_file(path, &ranges, &args)?;
        }
    }

    Ok(())
}