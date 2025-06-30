//! Utilities for vectors.
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[cfg(test)]
mod tests;

/// Read a file into a newline-separated `Vec` of `String`s. Skips empty lines.
///
/// Parameters
/// - `path` - The file to read from.
///
/// Errors
/// The file cannot be read.
pub fn read_file_to_vector(path: &impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(path)?)
        .lines()
        .filter(|line| if let Ok(line) = line { !line.is_empty() } else { false })
        .collect()
}
