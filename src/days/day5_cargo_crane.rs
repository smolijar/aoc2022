use std::{fmt::Error, str::FromStr};

use crate::inputs;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}
#[derive(Debug)]
struct Cargo {
    piles: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Cargo {
    fn top_crates(&self) -> String {
        self.piles
            .iter()
            .map(|p| p.iter().last().expect("Empty top"))
            .collect::<String>()
    }
    fn perform_crane_9000(&mut self) {
        for instruction in &self.instructions {
            for _ in 1..=instruction.count {
                let create = self
                    .piles
                    .get_mut(instruction.from)
                    .expect("Invalid from coordinate")
                    .pop()
                    .expect("Popping empty");
                self.piles
                    .get_mut(instruction.to)
                    .expect("Invalid to coordinate")
                    .push(create)
            }
        }
    }
    fn perform_crane_9001(&mut self) {
        for instruction in &self.instructions {
            let form_len = self
                .piles
                .get_mut(instruction.from)
                .expect("Invalid from coordinate")
                .len();
            let mut top_n = self
                .piles
                .get_mut(instruction.from)
                .expect("Invalid from coordinate")
                .split_off(form_len - instruction.count);
            self.piles
                .get_mut(instruction.to)
                .expect("Invalid to coordinate")
                .append(&mut top_n)
        }
    }

}

impl FromStr for Cargo {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (crates, instructions) = s
            .split("\n\n")
            .next_tuple()
            .expect("Missing instructions delimiter");

        let rev_lines = crates.split('\n').rev().collect_vec();
        let max_height = rev_lines.len() - 1;
        let max_width = rev_lines
            .iter()
            .map(|l| l.replace(" ", "").len())
            .max()
            .map(|max_length| max_length / 3)
            .expect("Empty lanes");

        let mut vectors = vec![];
        for _ in 0..max_width {
            vectors.push(vec![])
        }
        for current_level in 1..=max_height {
            for current_pile in 0..max_width {
                let cargo = rev_lines
                    .get(current_level)
                    .and_then(|line| {
                        line.chars().nth(
                            if current_pile > 0 {
                                current_pile * 4
                            } else {
                                0
                            } + 1,
                        )
                    })
                    .and_then(|char| match char {
                        ' ' => None,
                        x => Some(x),
                    });
                vectors
                    .get_mut(current_pile)
                    .expect("Missing vector pile")
                    .push(cargo);
            }
        }
        lazy_static! {
            static ref RE_INSTRUCTION: Regex =
                Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let cargo = Cargo {
            piles: vectors
                .into_iter()
                .map(|v| v.into_iter().filter_map(|c| c).collect_vec())
                .collect(),
            instructions: instructions
                .split('\n')
                .map(|i| {
                    let capture = RE_INSTRUCTION
                        .captures_iter(i)
                        .nth(0)
                        .expect("Invalid instruction");
                    return Instruction {
                        count: capture[1].parse::<usize>().expect("Instruction parse"),
                        from: capture[2].parse::<usize>().expect("Instruction parse") - 1,
                        to: capture[3].parse::<usize>().expect("Instruction parse") - 1,
                    };
                })
                .collect_vec(),
        };
        Ok(cargo)
    }
}

pub fn cargo_crane_9000(input: &str) -> String {
    let mut cargo = input.parse::<Cargo>().expect("Error parsing input");
    cargo.perform_crane_9000();
    cargo.top_crates()
}

pub fn cargo_crane_9001(input: &str) -> String {
    let mut cargo = input.parse::<Cargo>().expect("Error parsing input");
    cargo.perform_crane_9001();
    cargo.top_crates()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(cargo_crane_9000(&inputs::demo_input(5)), "CMZ");
        assert_eq!(cargo_crane_9001(&inputs::demo_input(5)), "MCD");
        assert_eq!(cargo_crane_9000(&inputs::task_input(5)), "TDCHVHJTG");
        assert_eq!(cargo_crane_9001(&inputs::task_input(5)), "NGCMPJLHV");
    }
}
