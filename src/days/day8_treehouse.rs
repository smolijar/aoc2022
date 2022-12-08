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
    fn scenic_score(&self, (x, y): &Coord) -> u32 {
        let (max_x, max_y) = self.tree_map.iter().map(|x| x.0).max().expect("Has trees");
        let mut up = (0..=*max_x).map(|x| (x, *y)).collect_vec();
        let mut left = (0..=*max_y).map(|y| (*x, y)).collect_vec();
        let mut down = up.split_off(*x);
        let mut right = left.split_off(*y);
        down.remove(0);
        right.remove(0);
        left.reverse();
        up.reverse();
        // println!("---------------------------");
        // println!("---------------------------");

        // println!("up={up:?}");
        // println!("down={down:?}");
        // println!("left={left:?}");
        // println!("right={right:?}");
        // println!("---------------------------");

        let treehouse_height = self.tree_map.get(&(*x, *y)).map(|x| *x).unwrap();

        let up = up;
        let down = down;
        let left = left;
        let right = right;

        // println!("up={up:?}");
        // println!("down={down:?}");
        // println!("left={left:?}");
        // println!("right={right:?}");

        let mut res = vec![];
        for lane in [up, down, left, right] {
            let mut mid_res = 0;
            for coord in lane {
                let cur_heigth = self.tree_map.get(&coord).expect("Tree on coord");
                mid_res+=1;
                if cur_heigth >= &treehouse_height {
                    break;
                }
            }
            res.push(mid_res)
        }
        return res.iter().product();

        let score = [up, down, left, right]
            .iter()
            .map(|dir| dir.len())
            .product::<usize>() as u32;
        // println!("({x}, {y}) = {score}");

        score
        // let curr_height = self.tree_map.get(&(*x, *y)).map(|x| *x);
        // let up = self.visible_coords(
        //     &(0..=x.saturating_sub(1))
        //         .map(|x| (x, *y))
        //         .rev()
        //         .collect_vec(),
        //     curr_height,
        // );
        // let down = self.visible_coords(
        //     &((x + 1)..=*max_row).map(|x| (x, *y)).collect_vec(),
        //     curr_height,
        // );
        // let left = self.visible_coords(
        //     &(0..=y.saturating_sub(1))
        //         .map(|y| (*x, y))
        //         .rev()
        //         .collect_vec(),
        //     curr_height,
        // );
        // let right = self.visible_coords(
        //     &((y + 1)..=*max_col).map(|y| (*x, y)).collect_vec(),
        //     curr_height,
        // );

        // return [up, down, left, right]
        //     .iter()
        //     .map(|dir| if dir.is_empty() { 1 } else { dir.len() })
        //     .product::<usize>() as u32;

        // // if *x == 3 && *y == 2 {
        // if *x == 2 && *y == 3 {
        //     println!("=={} {} {} {}", up.len().min(1), down.len().min(1), left.len().min(1), right.len().min(1))
        // }

        // (up.len().max(1) * down.len().max(1) * left.len().max(1) * right.len().max(1)) as u32
    }
    fn max_scenic_score(&self) -> u32 {
        println!(
            "{:?}",
            &self
                .tree_map
                .iter()
                .map(|x| (x.0, self.scenic_score(&x.0)))
                .collect_vec()
        );
        self.tree_map
            .iter()
            .map(|x| self.scenic_score(&x.0))
            .max()
            .expect("Has trees")
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
    fn visible_coords_or_eq(&self, vals: &Vec<Coord>, max_height: u32) -> Vec<Coord> {
        let mut blocked = true;
        vals.iter()
            .take_while(|x| {
                let cur_heigth = self.tree_map.get(x).expect("Tree on coord");
                cur_heigth <= &max_height
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
        let demo_forest = Forrest::from_str(&inputs::demo_input(8)).expect("");
        assert_eq!(demo_forest.scenic_score(&(1, 2)), 4);
        assert_eq!(demo_forest.scenic_score(&(3, 2)), 8);

        // That's not the right answer; your answer is too low. If you're stuck, make sure you're using the full input data; there are also some general tips on the about page, or you can ask for hints on the subreddit. Please wait one minute before trying again. (You guessed 10560.) [Return to Day 8]
        // TODO: slow
        // assert_eq!(max_scenic_score(&inputs::task_input(8)) > 10560, true);
    }
    #[test]
    fn test2() {
        assert_eq!(max_scenic_score(&inputs::demo_input(8)), 8);
        // TODO: slow
        // assert_eq!(max_scenic_score(&inputs::task_input(8)), 496125);
    }
}
