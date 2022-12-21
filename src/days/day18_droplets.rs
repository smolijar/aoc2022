use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn droplets(input: &str) -> usize {
    let mut xy: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    let mut yz: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    let mut xz: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    let points = input
        .split("\n")
        .map(|line| {
            let point = line
                .split(",")
                .map(|d| d.parse::<u32>().expect("usize coord"))
                .next_tuple::<(_, _, _)>()
                .unwrap();
            match xy.get_mut(&(point.0, point.1)) {
                Some(vec) => {
                    vec.push(point.2);
                }
                _ => {
                    xy.insert((point.0, point.1), vec![point.2]);
                }
            }
            match yz.get_mut(&(point.1, point.2)) {
                Some(vec) => {
                    vec.push(point.0);
                }
                _ => {
                    yz.insert((point.1, point.2), vec![point.0]);
                }
            }
            match xz.get_mut(&(point.0, point.2)) {
                Some(vec) => {
                    vec.push(point.1);
                }
                _ => {
                    xz.insert((point.0, point.2), vec![point.1]);
                }
            }
            point
        })
        .collect_vec();

    let mut shared_faces = 0;
    for dimenion in vec![xy, yz, xz] {
        for points in dimenion.values() {
            let mut prev: Option<u32> = None;
            for value in points.iter().sorted() {
                if let Some(prev_value) = prev {
                    if value.saturating_sub(prev_value) == 1 {
                        shared_faces += 1;
                    }
                }
                prev = Some(*value);
            }
        }
    }
    points.len() * 6 - shared_faces * 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;
    #[test]
    fn test() {
        assert_eq!(droplets(&inputs::demo_input(18)), 64);
        assert_eq!(droplets(&inputs::task_input(18)), 4192);
    }
}
