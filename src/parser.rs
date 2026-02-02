use std::ops::Range;

pub fn parse_range(s: &str) -> Result<Range<usize>, String> {
    if s.is_empty() {
        return Err("Range cannot be empty".to_string());
    }

    let (start_str, end_str) = s
        .split_once(|c| c == '-' || c == ':')
        .ok_or_else(|| format!("Invalid format '{}'. Expected START-END or START:END", s))?;

    let mut start = start_str
        .parse::<usize>()
        .map_err(|_| format!("Invalid start number: '{}'", start_str))?;

    let end = end_str
        .parse::<usize>()
        .map_err(|_| format!("Invalid end number: '{}'", end_str))?;

    if start > end {
        return Err(format!(
            "Start ({}) cannot be greater than End ({})",
            start, end
        ));
    }

    if start == 0 { start = 1; } // Prevent underflow if user types 0

    // User types 50 (Line 50), we want Index 49.
    Ok((start - 1)..(end))
}
