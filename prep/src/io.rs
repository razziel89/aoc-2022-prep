use anyhow::{Context, Error, Result};
use std::str::FromStr;

fn read_lines_from_file(path: &str) -> Result<Vec<String>> {
    Ok(std::fs::read_to_string(path)
        .context("reading from disk")?
        .split('\n')
        .filter(|el| el.len() > 0)
        .map(|el| String::from(el))
        .collect())
}

pub fn parse_lines_to_data<T>(file: &str, type_name: &str) -> Result<Vec<Result<T>>>
where
    T: FromStr<Err = Error>,
{
    // Read file and convert into actions.
    Ok(read_lines_from_file(file)
        .context("reading lines")?
        .into_iter()
        .enumerate()
        .map(|(idx, el)| {
            el.parse::<T>()
                .with_context(|| format!("cannot parse line {} as {}: {}", idx, type_name, el))
        })
        .collect())
}
