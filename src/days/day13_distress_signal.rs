use std::{cmp::Ordering, fmt::Error, str::FromStr};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Signal {
    Number(i32),
    List(Vec<Box<Signal>>),
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Signal::Number(x), Signal::Number(y)) => x.cmp(y),
            (Signal::List(xs), Signal::List(ys)) => {
                let non_eq_item_result = xs
                    .iter()
                    .zip(ys)
                    .map(|(a, b)| a.cmp(b))
                    .filter(|ord| ord != &Ordering::Equal)
                    .next();
                match non_eq_item_result {
                    Some(cmp) => cmp,
                    _ => xs.len().cmp(&ys.len()),
                }
            }
            (Signal::Number(x), l @ _) => Signal::List(vec![Box::new(Signal::Number(*x))]).cmp(l),
            (Signal::List(_), Signal::Number(x)) => {
                self.cmp(&Signal::List(vec![Box::new(Signal::Number(*x))]))
            }
        }
    }
}

impl FromStr for Signal {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(number) = s.parse::<i32>() {
            return Ok(Signal::Number(number));
        }
        let mut chars = s.chars();
        let mut camma_idxs = vec![];
        if chars.next().unwrap() == '[' {
            let mut opened_brackets = 1;
            let mut current_bracket_contents = chars
                .enumerate()
                .take_while(|(index, char)| {
                    match *char {
                        ',' if opened_brackets == 1 => camma_idxs.push(index.clone() + 1),
                        '[' => {
                            opened_brackets += 1;
                        }
                        ']' => {
                            opened_brackets -= 1;
                            if opened_brackets == 0 {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    true
                })
                .map(|x| x.1)
                .collect::<String>();
            let mut current_bracket_items = vec![];
            if current_bracket_contents.is_empty() {
                return Ok(Signal::List(vec![]));
            }
            if camma_idxs.is_empty() {
                return Ok(Signal::List(vec![Box::new(Signal::from_str(
                    &current_bracket_contents,
                )?)]));
            }
            for c in camma_idxs.iter().rev() {
                let a = current_bracket_contents.split_off(*c);
                current_bracket_items.push(a);
            }
            current_bracket_items.push(current_bracket_contents.clone());
            let current_bracket_signals = current_bracket_items
                .iter()
                .rev()
                .map(|x| x.trim_end_matches(","))
                .map(|n| Signal::from_str(&n))
                .collect::<Result<Vec<Signal>, _>>()?;

            return Ok(Signal::List(
                current_bracket_signals.into_iter().map(|x| Box::new(x)).collect_vec(),
            ));
        }
        Err(self::Error)
    }
}

pub fn distress_signal(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .map(|x| x.parse::<Signal>().unwrap())
                .collect_vec()
        })
        .enumerate()
        .filter(|(_, g)| g[0] < g[1])
        .map(|(i, _)| i + 1)
        .sum::<usize>() as i32
}
pub fn distress_signal_dividers(input: &str) -> i32 {
    ["[[2]]", "[[6]]", input]
        .join("\n")
        .replace("\n\n", "\n")
        .split("\n")
        .map(|x| x.parse::<Signal>().unwrap())
        .enumerate()
        .sorted_by(|a, b| a.1.cmp(&b.1))
        .enumerate()
        .filter(|(_, (original_index, _))| *original_index == 0 || *original_index == 1)
        .map(|(new_index, (_, _))| new_index + 1)
        .product::<usize>() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;
    #[test]
    fn test() {
        assert_eq!(distress_signal(&inputs::demo_input(13)), 13);
        assert_eq!(distress_signal_dividers(&inputs::demo_input(13)), 140);
        assert_eq!(distress_signal(&inputs::task_input(13)), 5806);
        assert_eq!(distress_signal_dividers(&inputs::task_input(13)), 23600);
    }
}
