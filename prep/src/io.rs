use anyhow::{Context, Result};
use std::str::FromStr;

fn read_lines_from_file(path: &str) -> Result<Vec<String>> {
    Ok(std::fs::read_to_string(path)
        .context("reading from disk")?
        .split('\n')
        .filter(|el| el.len() > 0)
        .map(|el| String::from(el))
        .collect())
}

pub fn parse_lines_to_data<T>(file: &str) -> Result<Vec<Result<T, <T as FromStr>::Err>>>
where
    T: FromStr,
{
    // Read file and convert into actions.
    Ok(read_lines_from_file(file)
        .context("reading lines")?
        .into_iter()
        .map(|el| el.parse::<T>())
        .collect())
}
