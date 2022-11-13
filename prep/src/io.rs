use crate::data::Action;

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

pub fn parse_lines_to_data(file: &str) -> Result<Vec<Result<Action>>> {
    // Read file and convert into actions.
    Ok(read_lines_from_file(file)
        .context("reading lines")?
        .into_iter()
        .map(|el| {
            el.parse::<Action>()
                .with_context(|| format!("cannot convert '{}' to data", el))
        })
        .collect())
}
