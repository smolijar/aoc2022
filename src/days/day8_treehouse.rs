use std::{
    collections::{HashMap, HashSet},
    fmt::Error,
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug)]
struct Forrest {
    tree_map: HashMap<Coord, u32>,
}

type Coord = (usize, usize);

impl Forrest {
    fn rows_and_cols(limit: &(usize, usize)) -> Vec<Vec<Coord>> {
        let mut res = vec![];
        for x in 0..=limit.0 {
            let up = (0..=limit.1).map(|y| (x, y)).collect_vec();
            res.push(up.clone().into_iter().rev().collect_vec());
            res.push(up);
        }
        for y in 0..=limit.1 {
            let right = (0..=limit.1).map(|x| (x, y)).collect_vec();
            res.push(right.clone().into_iter().rev().collect_vec());
            res.push(right);
        }
        res
    }
    fn visible_trees(&self) -> usize {
        let mut results = HashSet::new();
        let max = self.tree_map.iter().map(|x| x.0).max().expect("Has trees");
        for file in Forrest::rows_and_cols(max) {
            for x in self.visible_coords(&file) {
                results.insert(x);
            }
        }
        results.len()
    }
    fn max_scenic_score(&self) -> u32 {
        let mut res = HashMap::new();
        let limit = self.tree_map.iter().map(|x| x.0).max().expect("Has trees");
        for lane in Forrest::rows_and_cols(limit) {
            for i in 0..lane.len() {
                let tree_coord = lane.get(i).unwrap().clone();
                let tree_height = self.tree_map.get(&tree_coord).unwrap();
                let unblocked = lane
                    .iter()
                    .take(i)
                    .rev()
                    .map(|t| (t, self.tree_map.get(t).unwrap()))
                    .take_while(|t| t.1 < tree_height)
                    .count();
                let one_tree_comp = if (i - unblocked) > 0 { 1 } else { 0 };
                let lane_score = unblocked + one_tree_comp;
                let current_score = res.get(&tree_coord).unwrap_or(&1);
                res.insert(tree_coord, current_score * lane_score);
            }
        }
        *res.values().max().unwrap() as u32
    }
    fn visible_coords(&self, vals: &Vec<Coord>) -> Vec<Coord> {
        let mut cur_max = None;
        vals.iter()
            .filter(|x| {
                let cur_heigth = self.tree_map.get(x).expect("Tree on coord");
                let see = match cur_max {
                    Some(cm) => cur_heigth > &cm,
                    _ => true,
                };
                cur_max = Some(match cur_max {
                    Some(ref cm) => *cur_heigth.max(cm),
                    _ => *cur_heigth,
                });
                see
            })
            .map(|x| *x)
            .collect_vec()
    }
}

impl FromStr for Forrest {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tree_map = HashMap::new();
        for (x, line) in s.split('\n').enumerate() {
            for (y, tree) in line
                .chars()
                .map(|t| t.to_digit(10).expect("Tree is integer"))
                .enumerate()
            {
                tree_map.insert((x, y), tree);
            }
        }
        Ok(Forrest { tree_map })
    }
}

pub fn count_visible_trees(input: &str) -> usize {
    let forrest = Forrest::from_str(input).unwrap();
    forrest.visible_trees()
}

pub fn max_scenic_score(input: &str) -> u32 {
    let forrest = Forrest::from_str(input).unwrap();
    forrest.max_scenic_score()
}

#[cfg(test)]
mod tests {
    use crate::inputs;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(count_visible_trees(&inputs::demo_input(8)), 21);
        assert_eq!(count_visible_trees(&inputs::task_input(8)), 1538);

        // That's not the right answer; your answer is too low. If you're stuck, make sure you're using the full input data; there are also some general tips on the about page, or you can ask for hints on the subreddit. Please wait one minute before trying again. (You guessed 10560.) [Return to Day 8]
        assert_eq!(max_scenic_score(&inputs::task_input(8)) > 10560, true);
    }
    #[test]
    fn test2() {
        assert_eq!(max_scenic_score(&inputs::demo_input(8)), 8);
        assert_eq!(max_scenic_score(&inputs::task_input(8)), 496125);
    }
}
