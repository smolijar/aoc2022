use itertools::Itertools;

pub fn grove_coordinates(input: &str, encryption_key: i64, iterations: usize) -> i64 {
    let mut ns = input
        .split("\n")
        .map(|x| encryption_key * x.parse::<i64>().unwrap())
        .enumerate()
        .collect_vec();
    let len = ns.len() as i64;

    for _ in 0..iterations {
        for i in 0..len {
            let (current_index, item) = ns
                .iter()
                .find_position(|(init_ord, _)| *init_ord == i as usize)
                .unwrap();
            let item = item.clone();
            let new_idx = (current_index as i64 + item.1).rem_euclid(len - 1);
            ns.remove(current_index);
            ns.insert(new_idx as usize, item);
        }
    }
    let (zero_idx, _) = ns.iter().find_position(|(_, val)| *val == 0).unwrap();

    vec![1000, 2000, 3000]
        .iter()
        .map(|i| {
            ns.get((zero_idx + *i as usize).rem_euclid(len as usize))
                .unwrap()
                .1
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;
    #[test]
    fn test() {
        assert_eq!(grove_coordinates(&inputs::demo_input(20), 1, 1), 3);
        // That's not the right answer. If you're stuck, make sure you're using the full input data; there are also some general tips on the about page, or you can ask for hints on the subreddit. Please wait one minute before trying again. (You guessed -4921.)
        assert_ne!(grove_coordinates(&inputs::task_input(20), 1, 1), -4921);
        // That's not the right answer. If you're stuck, make sure you're using the full input data; there are also some general tips on the about page, or you can ask for hints on the subreddit. Please wait one minute before trying again. (You guessed -8811.)
        assert_ne!(grove_coordinates(&inputs::task_input(20), 1, 1), -8811);
        assert_eq!(grove_coordinates(&inputs::task_input(20), 1, 1), 10707);

        assert_eq!(
            grove_coordinates(&inputs::demo_input(20), 811589153, 10),
            1623178306
        );
        assert_eq!(grove_coordinates(&inputs::task_input(20), 811589153, 10), 2488332343098);
    }
}
