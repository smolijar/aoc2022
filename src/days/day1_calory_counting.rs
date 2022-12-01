use crate::inputs;

pub fn calory_counting(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|calory| calory.parse::<u32>().expect("Non-integer calory"))
                .sum()
        })
        .max()
        .expect("No elves")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(calory_counting(&inputs::test_input(1)), 24000);
    }
}
