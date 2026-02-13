use std::{collections::HashMap, error::Error};

pub fn replace_multiple_parallel(
    content: String,
    replacements: &HashMap<usize, String>,
) -> Result<String, Box<dyn Error>> {
    let mut lines: Vec<&str> = content.lines().collect();
    for (line_number, new_line) in replacements {
        if *line_number < lines.len() {
            lines[*line_number - 1] = new_line.as_str();
        } else {
            return Err(format!("Line number {} out of bounds", line_number).into());
        }
    }
    Ok(lines.join("\n"))
}

pub fn replace_single_parallel(
    content: String,
    line_number: usize,
    new_line: String,
) -> Result<String, Box<dyn Error>> {
    let mut lines: Vec<&str> = content.lines().collect();
    if line_number < lines.len() {
        lines[line_number - 1] = new_line.as_str();
        Ok(lines.join("\n"))
    } else {
        Err(format!("Line number {} out of bounds", line_number).into())
    }
}

/// Mode 0: insert before the line number.
///
/// Mode 1: insert after the line number.
pub fn insert_line_at(
    content: String,
    line_number: usize,
    mode: u32,
    new_line: String,
) -> Result<String, Box<dyn Error>> {
    let mut lines: Vec<&str> = content.lines().collect();
    if line_number <= lines.len() {
        if mode == 1 {
            lines.insert(line_number, new_line.as_str());
        } else if mode == 0 {
            lines.insert(line_number - 1, new_line.as_str());
        }
        Ok(lines.join("\n"))
    } else {
        Err(format!("Line number {} out of bounds", line_number).into())
    }
}
