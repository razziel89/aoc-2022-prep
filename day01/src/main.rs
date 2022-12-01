extern crate derive_more;

mod data;
mod io;

use crate::data::Baggage;

use anyhow::{Context, Error, Result};

const SAMPLE: &str = "sample.dat";
const REAL: &str = "stage_1.dat";
const NUM_ELVES: usize = 3;

#[derive(Debug)]
struct Elf {
    idx: usize,
    baggage: Vec<Baggage>,
}

impl Elf {
    fn total_calories(&self) -> u64 {
        self.baggage
            .iter()
            .map(|el| match el {
                Baggage::Calories(val) => val,
                Baggage::EndOfElf => &0,
            })
            .sum()
    }
}

fn baggages_to_elves(baggage: Vec<Baggage>) -> Vec<Elf> {
    let mut elves = vec![];
    let mut elf = Elf {
        idx: 1,
        baggage: vec![],
    };

    for el in baggage {
        match el {
            Baggage::Calories(_) => {
                elf.baggage.push(el);
            }
            Baggage::EndOfElf => {
                let next_elf = Elf {
                    idx: elf.idx + 1,
                    baggage: vec![],
                };
                elves.push(elf);
                elf = next_elf;
            }
        }
    }

    elves
}

fn process(file: &str) -> Result<()> {
    eprintln!("PROCESSING {}", file);

    // Read file and convert into data.
    let baggage = io::parse_lines_to_data::<Baggage>(file, "baggage").context("reading file")?;

    let mut elves = baggages_to_elves(baggage);

    // Elf carrying the most calories will be first in line. It is inefficient to calculate total
    // calories for every comparison, but it's not really important for this exercise.
    elves.sort_by(|el1, el2| el2.total_calories().cmp(&el1.total_calories()));

    if elves.len() == 0 {
        Err(Error::msg("somehow, we found no elves :("))
    } else {
        println!(
            "elf carrying most is {} who carries {} calories",
            elves[0].idx,
            elves[0].total_calories()
        );

        if elves.len() >= NUM_ELVES {
            let total_calories: u64 = elves
                .iter()
                .take(NUM_ELVES)
                .map(|el| el.total_calories())
                .sum();
            println!(
                "the {} elves carrying the most carry {} in total",
                NUM_ELVES, total_calories
            );
            Ok(())
        } else {
            Err(Error::msg("somehow, we found too few elves :("))
        }
    }
}

fn main() -> Result<()> {
    process(SAMPLE)?;
    process(REAL)?;

    Ok(())
}
