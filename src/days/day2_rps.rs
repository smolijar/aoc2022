use crate::inputs;
use itertools::Itertools;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(PartialEq, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

pub fn rps_generic<T>(input: &str, mapper: T) -> u32
where
    T: Fn((&str, &str)) -> (Hand, Hand),
{
    input
        .split("\n")
        .map(|round| {
            let (opponent, me) = round
                .split_whitespace()
                .next_tuple()
                .expect("Missing two hands");
            let hands = mapper((opponent, me));
            let hand_score = match hands.1 {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3,
            };
            let win_score = match hands {
                (a, b) if a == b => 3,
                (Hand::Scissors, Hand::Rock)
                | (Hand::Rock, Hand::Paper)
                | (Hand::Paper, Hand::Scissors) => 6,
                _ => 0,
            };
            hand_score + win_score
        })
        .sum()
}

fn opponent_to_hand(hand: &str) -> Hand {
    match hand {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        _ => unreachable!("Unknown hand"),
    }
}

pub fn rps(input: &str) -> u32 {
    rps_generic(input, |hands| {
        let map_hand = |hand| match hand {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            hand => opponent_to_hand(hand),
        };
        (map_hand(hands.0), map_hand(hands.1))
    })
}

pub fn rps_full_plan(input: &str) -> u32 {
    rps_generic(input, |hands| match (opponent_to_hand(hands.0), hands.1) {
        (opponent, "X") => (
            opponent,
            Hand::try_from_primitive((opponent as u8 + 2) % 3).expect("..."),
        ),
        (opponent, "Y") => (opponent, opponent),
        (opponent, "Z") => (
            opponent,
            Hand::try_from_primitive((opponent as u8 + 1) % 3).expect("..."),
        ),
        _ => unreachable!("Unknown hand"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(rps(&inputs::demo_input(2)), 15);
        assert_eq!(rps_full_plan(&inputs::demo_input(2)), 12);
        assert_eq!(rps(&inputs::task_input(2)), 13005);
        assert_eq!(rps_full_plan(&inputs::task_input(2)), 11373);
    }
}
