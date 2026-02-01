//! Utilities for vectors.
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[cfg(test)]
mod tests;

/// Read a file into a newline-separated `HashSet` of Strings. Skips empty lines.
///
/// ## Errors
/// - The file cannot be read.
//# TESTED
pub fn read_file_to_hashset(path: &impl AsRef<Path>) -> io::Result<HashSet<String>> {
    let mut lines = BufReader::new(File::open(path)?).lines().collect::<io::Result<HashSet<String>>>()?;
    lines.remove("");
    Ok(lines)
}
