use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::ops::Range;
use std::path::{Path};
use colored::*;
use crate::cli::{Args, ColorMode};

const BUFFER_SIZE: usize = 16 * 1024;
pub fn count_lines(path: &Path) -> io::Result<usize>{
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);
    let mut buffer = [0; BUFFER_SIZE];
    let mut count = 0;

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        count += buffer[..n].iter().filter(|&&b| b == b'\n').count();
    }
    Ok(count)
}

pub fn resolve_ranges(args: &Args, total_lines: Option<usize>) -> anyhow::Result<Vec<Range<usize>>> {
    let mut ranges = Vec::new();

    if let Some(n) = args.head {
        ranges.push(0..n);
    }

    if let Some(ref s) = args.lines {
        ranges.push(s.clone());
    }

    if let Some(total) = total_lines {
        if let Some(n) = args.tail {
            let start = total.saturating_sub(n);
            ranges.push(start..total);
        }
        if let Some(n) = args.middle {
            let mid = total / 2;
            let half_width = n / 2;
            let start = mid.saturating_sub(half_width);
            let end = (mid + half_width).min(total);
            ranges.push(start..end);
        }
    }

    Ok(ranges)
}

fn get_context_sizes(args: &Args) -> (usize, usize) {
    let before = args.before_context.or(args.context).unwrap_or(0);
    let after = args.after_context.or(args.context).unwrap_or(0);
    (before, after)
}

// Update the function signature to accept 'args'
pub(crate) fn slice_file(path: &Path, ranges: &[Range<usize>], args: &Args) -> anyhow::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file); // Make reader mutable
    let mut writer = BufWriter::new(io::stdout().lock());

    let (before_size, after_size) = get_context_sizes(args);
    let mut before_buffer: VecDeque<(usize, String)> = VecDeque::with_capacity(before_size);
    let mut print_after_count = 0;

    // Optimization: Stop early
    let max_line = ranges.iter().map(|r| r.end).max().unwrap_or(usize::MAX) + after_size;

    // --- NEW: REUSABLE BUFFER STRATEGY ---
    let mut line_buf = String::new(); // Allocate ONCE
    let mut index = 0;

    // We use a while loop with read_line
    while reader.read_line(&mut line_buf)? > 0 {
        if index >= max_line { break; }

        let line_num = index + 1;

        // Trim the newline for printing logic (read_line includes \n)
        let line_content = line_buf.trim_end();

        let is_match = ranges.iter().any(|r| r.contains(&index));

        if is_match {
            // A. HIT
            while let Some((hist_num, hist_line)) = before_buffer.pop_front() {
                print_line(&mut writer, &hist_line, hist_num, args, false)?;
            }
            print_line(&mut writer, line_content, line_num, args, true)?;
            print_after_count = after_size;
        } else {
            // B. NO HIT
            if print_after_count > 0 {
                print_line(&mut writer, line_content, line_num, args, false)?;
                print_after_count -= 1;
            } else if before_size > 0 {
                // Only clone string if we absolutely must save it for context
                before_buffer.push_back((line_num, line_content.to_string()));
                if before_buffer.len() > before_size {
                    before_buffer.pop_front();
                }
            }
        }

        // IMPORTANT: Clear the buffer to reuse the memory for the next line
        line_buf.clear();
        index += 1;
    }

    writer.flush()?;
    Ok(())
}

// Helper function to handle the -N flag cleanly
fn print_line(
    writer: &mut impl Write,
    line: &str,
    line_num: usize,
    args: &Args,
    is_match: bool,
) -> anyhow::Result<()> {
    // 1. Check Color Mode
    // For MVP, let's just assume true if "Never" isn't explicitly set.
    // (We can add is_terminal detection later to be perfect)
    let use_colors = !matches!(args.color, ColorMode::Never);

    // 2. Prepare the components
    let num_str = line_num.to_string();

    // Create the "ColoredString" object based on logic
    let formatted_num = if use_colors {
        if is_match {
            num_str.green().bold() // Green & Bold for matches
        } else {
            num_str.blue().dimmed() // Dim Blue for context
        }
    } else {
        num_str.normal() // "normal" means no ANSI codes
    };

    let separator = if is_match { ":" } else { "-" };

    // 3. Print
    if args.show_line_numbers {
        // formatted_num knows how to print itself (with or without colors)
        writeln!(writer, "{}{}\t{}", formatted_num, separator, line)?;
    } else {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}