use anyhow::{Error, Result};
// use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Action {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let elems: Vec<&str> = s.split_whitespace().collect();
        match elems[..] {
            ["forward", num] => Ok(Action::Forward(num.parse::<i32>()?)),
            ["down", num] => Ok(Action::Down(num.parse::<i32>()?)),
            ["up", num] => Ok(Action::Up(num.parse::<i32>()?)),
            _ => Err(Error::msg(format!("unexpected input {:?}", elems))),
        }
    }
}
