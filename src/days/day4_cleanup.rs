use crate::inputs;

use itertools::Itertools;

fn cleanup_generic<T>(input: &str, overlap_fn: T) -> usize
where
    T: Fn((u32, u32, u32, u32)) -> bool,
{
    input
        .split("\n")
        .filter_map(|line| {
            let clean_instructions = line
                .split(',')
                .flat_map(|range| {
                    range
                        .split('-')
                        .map(|x| x.parse::<u32>().expect("Invalid number"))
                })
                .next_tuple::<(u32, u32, u32, u32)>()
                .expect("Missing 4 instructions");
            if overlap_fn(clean_instructions) {
                return Some(());
            }
            return None;
        })
        .count()
}

pub fn cleanup_full(input: &str) -> usize {
    cleanup_generic(input, |(a1, a2, b1, b2)| {
        if a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2 {
            true
        } else {
            false
        }
    })
}

pub fn cleanup_partial(input: &str) -> usize {
    cleanup_generic(input, |(a1, a2, b1, b2)| {
        if a1 <= b1 && a2 >= b1 || b1 <= a1 && b2 >= a1 {
            true
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(cleanup_full(&inputs::demo_input(4)), 2);
        assert_eq!(cleanup_full(&inputs::task_input(4)), 464);
        assert_eq!(cleanup_partial(&inputs::demo_input(4)), 4);
        assert_eq!(cleanup_partial(&inputs::task_input(4)), 770);
    }
}
