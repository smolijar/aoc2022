use std::collections::HashSet;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]

struct ManhattonArea {
    point: Point,
    distance: u32,
}

impl ManhattonArea {
    pub fn new(origin: &Point, edge: &Point) -> ManhattonArea {
        ManhattonArea {
            point: origin.clone(),
            distance: (origin.x - edge.x).abs() as u32 + (origin.y - edge.y).abs() as u32,
        }
    }
    pub fn coverage_on_y(&self, y: i32) -> Vec<Point> {
        let dist_left = self
            .distance
            .saturating_sub((self.point.y - y).abs() as u32);
        let points_n = dist_left * 2 + 1;
        let start = self.point.x - 1 - dist_left as i32;
        (1..=points_n)
            .map(|i| Point {
                x: start + i as i32,
                y,
            })
            .collect_vec()
    }
}

pub fn beacons(input: &str, y: i32) -> usize {
    lazy_static! {
        static ref RE_BEACON: Regex =
            Regex::new(r"Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)").unwrap();
    }
    let mut beacons = HashSet::new();
    println!(
        "{:?}",
        input.split("\n").map(|line| {
            let cap = RE_BEACON.captures_iter(line).nth(0).unwrap();
            let sensor = Point {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
            };
            let beacon = Point {
                x: cap[3].parse().unwrap(),
                y: cap[4].parse().unwrap(),
            };
            beacons.insert(beacon.clone());
            ManhattonArea::new(&sensor, &beacon)
        }).collect_vec()
    );
    let points_on_y = input
        .split("\n")
        .map(|line| {
            let cap = RE_BEACON.captures_iter(line).nth(0).unwrap();
            let sensor = Point {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
            };
            let beacon = Point {
                x: cap[3].parse().unwrap(),
                y: cap[4].parse().unwrap(),
            };
            beacons.insert(beacon.clone());
            ManhattonArea::new(&sensor, &beacon)
        })
        .flat_map(|area| area.coverage_on_y(y))
        .collect::<HashSet<Point>>();
    points_on_y.difference(&beacons).count()
    // println!("{:#?}", areas.get(6));
    // println!("{:#?}", areas.get(6).unwrap().coverage_on_y(10));
    // 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;
    #[test]
    fn test() {
        assert_eq!(beacons(&inputs::demo_input(15), 10), 26);
        // TODO:(slow)  assert_eq!(beacons(&inputs::task_input(15), 2000000), 5108096);
    }
}
