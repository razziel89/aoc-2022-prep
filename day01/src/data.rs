use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Baggage {
    Calories(u64),
    EndOfElf,
}

impl FromStr for Baggage {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "" => Ok(Baggage::EndOfElf),
            val => Ok(Baggage::Calories(val.parse::<u64>()?)),
        }
    }
}
