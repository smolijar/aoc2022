use std::collections::HashSet;

fn end_of_unique_sequence(signal: &str, len: usize) -> usize {
    let mut windows = 0;
    for segment in signal.chars().collect::<Vec<char>>()[..].windows(len).into_iter() {
        let x = segment.into_iter().collect::<HashSet<&char>>();
        if x.len() == len {
            break
        }
        windows += 1;
    }
    windows + len
}

pub fn end_of_signal_marker(signal: &str) -> usize {
    end_of_unique_sequence(signal, 4)
}

pub fn end_of_message_marker(signal: &str) -> usize {
    end_of_unique_sequence(signal, 14)
}

#[cfg(test)]
mod tests {
    use crate::inputs;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(end_of_signal_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(end_of_signal_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(end_of_signal_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(
            end_of_signal_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            10
        );
        assert_eq!(end_of_signal_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
        assert_eq!(end_of_signal_marker(&inputs::task_input(6)), 1702);


        assert_eq!(end_of_message_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(end_of_message_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(end_of_message_marker("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(
            end_of_message_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            29
        );
        assert_eq!(end_of_message_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
        assert_eq!(end_of_message_marker(&inputs::task_input(6)), 3559);
    }
}
