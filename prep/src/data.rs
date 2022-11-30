use anyhow::{Error, Result};
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

#[derive(Debug, PartialEq)]
pub enum Spot {
    X,
    O,
}

impl FromStr for Spot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 1 {
            match s.chars().nth(0).unwrap_or('\0') {
                'x' => Ok(Spot::X),
                'o' => Ok(Spot::O),
                _ => Err(Error::msg(format!("unexpected input {:?}", s))),
            }
        } else {
            Err(Error::msg(format!("unexpected empty input")))
        }
    }
}
