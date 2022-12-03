use std::collections::HashSet;

use itertools::Itertools;

use crate::inputs;

fn evaluate_item(item: &char) -> u32 {
    match item {
        char @ 'a'..='z' => *char as u32 - 'a' as u32 + 1,
        char @ 'A'..='Z' => *char as u32 - 'A' as u32 + 26 + 1,
        _ => unreachable!(),
    }
}

pub fn rucksack(input: &str) -> u32 {
    input
        .split("\n")
        .map(|items| {
            let compartments = items.split_at(items.len() / 2);
            let a = compartments.0.chars().collect::<HashSet<char>>();
            let b = compartments.1.chars().collect::<HashSet<char>>();
            a.intersection(&b).map(evaluate_item).sum::<u32>()
        })
        .sum()
}

pub fn rucksack_badges(input: &str) -> u32 {
    input
        .split("\n")
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(|rucksack| rucksack.chars().collect::<HashSet<char>>())
                .reduce(|acc, current| acc.intersection(&current).map(|x| *x).collect())
                .expect("Empty rucksack triplet")
                .iter()
                .map(evaluate_item)
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(rucksack(&inputs::demo_input(3)), 157);
        assert_eq!(rucksack_badges(&inputs::demo_input(3)), 70);
        assert_eq!(rucksack(&inputs::task_input(3)), 7850);
        assert_eq!(rucksack_badges(&inputs::task_input(3)), 2581);
    }
}
