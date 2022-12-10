use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // fn diff(&self, other: &Point) -> Point {
    //     Point {
    //         x: self.x - other.x,
    //         y: self.x - other.x,
    //     }
    // }
}

fn move_tail(tail: &Point, head: &Point) -> Option<Point> {
    let diff_x = head.x - tail.x;
    let diff_y = head.y - tail.y;

    if diff_x.abs() < 2 && diff_y.abs() < 2 {
        return None;
    }

    Some(Point {
        x: tail.x + diff_x.signum(),
        y: tail.y + diff_y.signum(),
    })
}

struct Rope<'a> {
    knots: &'a mut Vec<Point>,
}

impl<'a> Rope<'a> {
    fn move_head(&mut self, to: &Point) {
        let head = self.knots.first_mut().expect("Rope has head");
        head.x += to.x;
        head.y += to.y;
        for i in 0..self.knots.len() {
            let mut new_pos = None;
            match (self.knots.get(i), self.knots.get(i + 1)) {
                (Some(head), Some(tail)) => {
                    new_pos = move_tail(tail, head);
                }
                _ => {}
            }
            if let Some(x) = new_pos {
                let movee = self.knots.get_mut(i + 1).unwrap();
                movee.x = x.x;
                movee.y = x.y;
            }
        }
    }
    fn tail(&self) -> &Point {
        self.knots.last().expect("Has tail")
    }
}

fn tail_locations(input: &str, knot_count: usize) -> usize {
    let mut tracks = HashSet::new();
    let mut knots = vec![];
    for _ in 0..knot_count {
        knots.push(Point { x: 0, y: 0 })
    }
    let mut rope = Rope {
        knots: &mut knots,
    };
    // tracks.insert(tail.clone());
    for instruction in input.split('\n') {
        let (dir, steps) = instruction.split_at(1);
        let vec = match dir {
            "R" => (0, 1),
            "L" => (0, -1),
            "U" => (1, 0),
            "D" => (-1, 0),
            _ => unreachable!(),
        };
        for _ in 0..steps.trim().parse::<u32>().expect("Step count to be uint") {
            rope.move_head(&Point { x: vec.0, y: vec.1 });
            tracks.insert(rope.tail().clone());
        }
    }
    tracks.len()
}

#[cfg(test)]
mod tests {
    use crate::inputs;

    use super::*;

    #[test]
    fn test() {
        let zero = Point { x: 0, y: 0 };
        // assert_eq!(move_tail(&zero, &Point { x: 1, y: -1 }), None);
        assert_eq!(
            move_tail(&zero, &Point { x: 0, y: 2 }),
            Some(Point { x: 0, y: 1 })
        );
        assert_eq!(
            move_tail(&zero, &Point { x: 1, y: 2 }),
            Some(Point { x: 1, y: 1 })
        );
        let alt_demo = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(tail_locations(&inputs::demo_input(9), 2), 13);
        assert_eq!(tail_locations(&inputs::task_input(9), 2), 6314);
        assert_eq!(tail_locations(alt_demo, 10), 36);
        assert_eq!(tail_locations(&inputs::task_input(9), 10), 2504);


    }
}
