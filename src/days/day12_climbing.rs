use std::{
    collections::{HashMap, HashSet},
    fmt::Error,
    ops::Sub,
    str::FromStr,
};

use itertools::Itertools;

struct HeightMap {
    height_map: HashMap<(i64, i64), i32>,
}

impl HeightMap {
    fn bfs2<'s, 'i>(&'s self, start: &'i (i64, i64)) {
        println!(
            "{:?}",
            self.height_map
                .iter()
                .permutations(5)
                .filter(|c| c.first().unwrap().0 == start)
                // .filter(|c| c.iter().tuple_windows::<(_, _)>().into_iter().all(|(((prev_x, prev_y), prev_height), ((curr_x, curr_y), curr_val))| {
                //     prev_x.sub(curr_x).abs() + prev_y.sub(curr_y).abs() <= 1
                // }))
                .take(1)
                .collect_vec()
        )
    }
    fn bfs(&self) -> i32 {
        let max = self.height_map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

        let min = self.height_map.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();

        let mut dist_map: HashMap<(i64, i64), i32> = HashMap::new();
        dist_map.insert(*max.0, 0);
        let mut current_points = vec![*max.0];
        let mut dist = 0;
        loop {
            dist += 1;
            // if dist >= 10 {
            //     break;
            // }
            let mut extension = vec![];
            for point in &current_points {
                let current_height = self.height_map.get(point).unwrap();
                let mut reachable_points = vec![
                    (point.0 + 1, point.1),
                    (point.0 - 1, point.1),
                    (point.0, point.1 + 1),
                    (point.0, point.1 - 1),
                ]
                .into_iter()
                .flat_map(|d| match self.height_map.get(&d) {
                    Some(x) => Some((d, x)),
                    None => None,
                })
                .filter(|(a, b)| (current_height - *b) <= 1)
                .filter(|(a, b)| match dist_map.get(a) {
                    Some(d) => d > &dist,
                    None => true,
                })
                .collect_vec();
                for reachable in reachable_points {
                    dist_map.insert(reachable.0, dist);
                    extension.push(reachable.0.clone());
                }
            }

            current_points.clear();
            for e in extension {
                current_points.push(e);
            }

            if let Some(res) = dist_map.get(&min.0) {
                break;
            }
        }
        println!("{:?}", dist_map);
        for r in 0..41 {
            for c in 0..161 {
                let dir = if dist_map.get(&(r, c)).unwrap_or(&-1) - dist_map.get(&(r + 1, c)).unwrap_or(&-1) == 1 {
                    "^"
                } else if dist_map.get(&(r, c)).unwrap_or(&-1) - dist_map.get(&(r - 1, c)).unwrap_or(&-1) == 1 {
                    "v"
                } else if dist_map.get(&(r, c)).unwrap_or(&-1) - dist_map.get(&(r, c + 1)).unwrap_or(&-1) == 1 {
                    ">"
                } else if dist_map.get(&(r, c)).unwrap_or(&-1) - dist_map.get(&(r, c - 1)).unwrap_or(&-1) == 1 {
                    "<"
                } else { " "};
                print!("{:4}", dist_map.get(&(r, c)).unwrap_or(&-1));
                // print!("{:8}", format!("{}/{}{}", dist_map.get(&(r, c)).unwrap_or(&-1), self.height_map.get(&(r, c)).unwrap(), dir))
            }
            println!("");
        }
        *dist_map.get(&min.0).unwrap()
    }
}

impl FromStr for HeightMap {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height_map = s
            .split('\n')
            .enumerate()
            .flat_map(|(x, row)| {
                row.chars()
                    .map(|c| match c {
                        'S' => 0,
                        'E' => 27,
                        c => (c as i32) - ('a' as i32) + 1,
                    })
                    .enumerate()
                    .map(move |(y, level)| ((x as i64, y as i64), level))
            })
            .collect::<HashMap<(i64, i64), i32>>();
        Ok(HeightMap { height_map })
    }
}

pub fn climbing(input: &str) -> i32 {
    let map = input.parse::<HeightMap>().unwrap();
    map.bfs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;
    #[test]
    fn test() {
        // assert_eq!(climbing(&inputs::demo_input(12)), 31);
    }
    #[test]
    fn test2() {
        // That's not the right answer; your answer is too high. If you're stuck, make sure you're using the full input data; there are also some general tips on the about page, or you can ask for hints on the subreddit. Please wait one minute before trying again. (You guessed 444.) [Return to Day 12]
        // assert_eq!(climbing(&inputs::task_input(12)) < 444, true);
        assert_eq!(climbing(&inputs::task_input(12)), 0);
        // assert_eq!(climbing(&inputs::task_input(12)), 31);
    }
}
