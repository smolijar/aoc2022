use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::{Display, Error},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug)]
enum Debris {
    Rock,
    Sand,
}
impl Display for Debris {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Debris::Rock => "#",
                Debris::Sand => "o",
            }
        )
    }
}

#[derive(Debug)]
struct Cave {
    debris: HashMap<(i32, i32), Debris>,
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let max = self.debris.keys().max().unwrap();
        // let min = self.debris.keys().min().unwrap();
        for row in (0).min(0)..=(13).max(0) {
            for col in (480).min(500)..=(520).max(500) {
                // write!(f, "{:?}", (col, row))?;
                write!(
                    f,
                    "{}",
                    match self.debris.get(&(col, row)) {
                        Some(d) => format!("{}", d),
                        None => format!("."),
                    }
                )?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

impl Cave {
    pub fn drop_sand(&mut self) -> bool {
        let mut start = (500, 0);
        let lowest_rock = self.debris.iter().filter_map(|((_, y), d)| {
            match d {
                Debris::Rock => { Some(y) },
                _ => None,
            }
        }).max().unwrap().clone() + 1;
        while start.1 < lowest_rock {

            if !self.debris.contains_key(&(start.0, start.1 + 1)) {
                start.1 += 1;
            } else if !self.debris.contains_key(&(start.0 - 1, start.1 + 1)) {
                start.1 += 1;
                start.0 -= 1;
            } else if !self.debris.contains_key(&(start.0 + 1, start.1 + 1)) {
                start.1 += 1;
                start.0 += 1;
            } else {
                break;
            }
        }
        self.debris.insert(start, Debris::Sand);
        start.1 < lowest_rock
    }
    pub fn hole_blocked(&self) -> bool {
        self.debris.contains_key(&(500, 0))
    }
}

impl FromStr for Cave {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut debris = HashMap::new();
        for line in s.split("\n") {
            let mut last: Option<(i32, i32)> = None;
            for point in line.split(" -> ").filter(|d| !d.is_empty()) {
                let (x, y) = point
                    .split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .next_tuple()
                    .unwrap();
                if let Some(last) = last {
                    if x == last.0 {
                        for y in last.1.min(y)..=last.1.max(y) {
                            debris.insert((x, y), Debris::Rock);
                        }
                    } else if y == last.1 {
                        for x in last.0.min(x)..=last.0.max(x) {
                            debris.insert((x, y), Debris::Rock);
                        }
                    } else {
                        panic!()
                    }
                }
                last = Some((x, y));
            }
        }
        Ok(Cave { debris })
    }
}

pub fn count_sand(input: &str) -> i32 {
    let mut cave = input.parse::<Cave>().unwrap();
    let mut drops = 0;
    while cave.drop_sand() {
        drops += 1;
    }
    drops
}

pub fn count_filled(input: &str) -> i32 {
    let mut cave = input.parse::<Cave>().unwrap();
    let mut drops = 0;
    while !cave.hole_blocked() {
        cave.drop_sand();
        drops += 1;
    }
    drops
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;
    #[test]
    fn test() {
        assert_eq!(count_sand(&inputs::demo_input(14)), 24);
        assert_eq!(count_sand(&inputs::task_input(14)), 961);
        assert_eq!(count_filled(&inputs::demo_input(14)), 93);
        assert_eq!(count_filled(&inputs::task_input(14)), 26375);
    }
}
