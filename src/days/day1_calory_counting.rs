use itertools::Itertools;

use crate::inputs;

fn top_n_elves_sum(input: &str, count: usize) -> u32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|calory| calory.parse::<u32>().expect("Non-integer calory"))
                .sum::<u32>()
        })
        .sorted()
        .rev()
        .take(count)
        .sum()
}

pub fn calory_counting(input: &str) -> u32 {
    top_n_elves_sum(input, 1)
}

pub fn calory_counting_top3(input: &str) -> u32 {
    top_n_elves_sum(input, 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(calory_counting(&inputs::demo_input(1)), 24000);
        assert_eq!(calory_counting_top3(&inputs::demo_input(1)), 45000);
        assert_eq!(calory_counting(&inputs::task_input(1)), 72718);
        assert_eq!(calory_counting_top3(&inputs::task_input(1)), 213089);
    }
}
