extern crate derive_more;

mod data;
mod io;

use crate::data::Action;

use anyhow::{Context, Result};
// use std::fmt::{Display, Formatter};

const SAMPLE: &str = "sample.dat";
const REAL: &str = "stage_1.dat";
const START: Pos = Pos {
    dist: 0,
    depth: 0,
    aim: 0,
};

#[derive(Clone, Debug)]
struct Pos {
    dist: i32,
    depth: i32,
    aim: i32,
}

fn apply_action_part1(pos: Pos, act: Action) -> Pos {
    match act {
        Action::Up(dist) => Pos {
            dist: pos.dist,
            depth: pos.depth - dist,
            aim: pos.aim,
        },
        Action::Down(dist) => Pos {
            dist: pos.dist,
            depth: pos.depth + dist,
            aim: pos.aim,
        },
        Action::Forward(dist) => Pos {
            dist: pos.dist + dist,
            depth: pos.depth,
            aim: pos.aim,
        },
    }
}

fn apply_action_part2(pos: Pos, act: Action) -> Pos {
    match act {
        Action::Up(dist) => Pos {
            dist: pos.dist,
            depth: pos.depth,
            aim: pos.aim - dist,
        },
        Action::Down(dist) => Pos {
            dist: pos.dist,
            depth: pos.depth,
            aim: pos.aim + dist,
        },
        Action::Forward(dist) => Pos {
            dist: pos.dist + dist,
            depth: pos.depth + pos.aim * dist,
            aim: pos.aim,
        },
    }
}

fn process(file: &str, update_fn: fn(Pos, Action) -> Pos) -> Result<()> {
    println!("PROCESSING {}", file);

    // Read file and convert into actions.
    let actions = io::parse_lines_to_data(file, "action").context("reading file")?;

    // Define starting conditions.
    let mut pos = START.clone();

    // Actually process by applying the update function to the position.
    for act in actions {
        match act {
            Ok(val) => pos = update_fn(pos, val),
            Err(err) => return Err(err),
        }
    }

    println!("final position is {:?}", pos);
    println!("product is {}", pos.dist * pos.depth);

    Ok(())
}

fn main() -> Result<()> {
    process(SAMPLE, apply_action_part1)?;
    process(REAL, apply_action_part1)?;

    process(SAMPLE, apply_action_part2)?;
    process(REAL, apply_action_part2)?;

    Ok(())
}
