use std::{fmt::Error, str::FromStr};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    interest: (String, String, String), // "old"/number, +/*, "old"/number */
    divisible: i64,
    pass_monkey: usize,
    fail_monkey: usize,
}

impl FromStr for Monkey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_MONKEY: Regex = Regex::new(
                r"Starting items: ([\d, ]+).*Operation: new = (.*).*Test: divisible by (\d+).*If true: throw to monkey (\d+).*If false: throw to monkey (\d+)"
            )
            .unwrap();
        }
        let str = s.replace("\n", " ");
        let first_capture = RE_MONKEY.captures_iter(&str).nth(0).unwrap();
        // println!("{:?}", first_capture[2].to_string().trim().split(" ").collect_vec());

        let items = first_capture[1]
            .split(", ")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect_vec();
        let interest: (_, _, _) = first_capture[2]
            .trim()
            .split(" ")
            .map(|x| x.to_string())
            .next_tuple()
            .unwrap();
        let divisible = first_capture[3].trim().parse::<i64>().unwrap();
        let pass_monkey = first_capture[4].trim().parse::<usize>().unwrap();
        let fail_monkey = first_capture[5].trim().parse::<usize>().unwrap();
        // println!("{divisible:?}");
        Ok(Monkey {
            items,
            divisible,
            fail_monkey,
            interest,
            pass_monkey,
        })
    }
}

pub fn monkeys(input: &str, relief: bool, rounds: usize) -> i64 {
    let mut monkeys = input
        .split("\n\n")
        .map(|monkey| monkey.parse::<Monkey>().unwrap())
        .collect_vec();

    let common = monkeys.iter().map(|m| m.divisible).product::<i64>();
    let mut monkeys_inspections = (0..monkeys.len()).map(|_| 0).collect_vec();
    for round in 1..=rounds {
        for monkey_index in 0..monkeys.len() {
            let mut throws = vec![];
            let monkey = monkeys.get_mut(monkey_index).unwrap();
            for interest in &monkey.items {
                let a = match monkey.interest.0.as_str() {
                    "old" => *interest,
                    x => x.parse::<i64>().unwrap(),
                };
                let b = match monkey.interest.2.as_str() {
                    "old" => *interest,
                    x => x.parse::<i64>().unwrap(),
                };
                let mut worry = match monkey.interest.1.as_str() {
                    "*" => a * b,
                    "+" => a + b,
                    _ => unreachable!(),
                } % common;
                let x= monkeys_inspections.get_mut(monkey_index).unwrap();
                *x = *x + 1;
                //relief
                if relief {
                    worry = worry / 3;
                }
                if worry % monkey.divisible == 0 {
                    throws.push((monkey.pass_monkey, worry));
                } else {
                    throws.push((monkey.fail_monkey, worry));
                }
            }
            monkey.items.clear();
            for t in throws {
                monkeys.get_mut(t.0).unwrap().items.push(t.1);
            }
        }
        println!("ROUND {round}");
        for monkey in &monkeys {
            println!("{:?}", monkey.items)
        }
    }
    monkeys_inspections.iter().sorted().rev().take(2).product::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;
    #[test]
    fn test() {
        assert_eq!(monkeys(&inputs::demo_input(11), true, 20), 10605);
        assert_eq!(monkeys(&inputs::task_input(11), true, 20), 56350);
        assert_eq!(monkeys(&inputs::demo_input(11), false, 10000), 2713310158);
        assert_eq!(monkeys(&inputs::task_input(11), false, 10000), 13954061248);

    }
}
