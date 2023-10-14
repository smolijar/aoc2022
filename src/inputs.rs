use std::fs;

use itertools::Itertools;

fn read_file(path: &str) -> String {
    fs::read_to_string(&path).expect(&format!("Failed to read file {}", &path))
}

pub fn demo_input(day: u32) -> String {
    read_file(&format!("inputs/demo/{}.txt", day))
}
pub fn task_input(day: u32) -> String {
    read_file(&format!("inputs/task/{}.txt", day))
}

mod tests {
    use super::*;

    #[test]
    fn test_inputs() {
        assert_eq!(demo_input(1).len(), 54);
        assert_eq!(task_input(1).len(), 10459);
    }
}

fn order_weight(s: &str) -> String {
    s.split(" ")
        .into_iter()
        .sorted()
        .sorted_by_key(|x| x.chars().map(|d| d.to_digit(10).unwrap()).sum::<u32>())
        .join(" ")
}
