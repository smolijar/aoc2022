use std::{fmt::Error, str::FromStr};

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "noop" => Instruction::Noop,
            x => Instruction::AddX(
                x.split(' ')
                    .nth(1)
                    .map(|n| n.parse::<i32>().expect("Valid add argument"))
                    .expect("Valid add instruction"),
            ),
        })
    }
}

struct Crt {
    pub output: String,
    cursor: i32,
}
impl Crt {
    pub fn new() -> Crt {
        Crt {
            output: String::new(),
            cursor: 0,
        }
    }
    pub fn write(&mut self, sprite_position: i32) {
        self.cursor += 1;

        self.output += if self.cursor >= sprite_position && self.cursor < sprite_position + 3 {
            "#"
        } else {
            "."
        };
        self.cursor %= 40;
        if self.cursor == 0 {
            self.output += "\n";
        }
    }
}

pub fn process(input: &str) -> (i32, String) {
    let instructions = input
        .split('\n')
        .map(|instr| instr.parse::<Instruction>().unwrap());
    let mut clock = 1;
    let mut registry_x = 1;
    let mut stack = Vec::new();
    let mut signal_strenghts = Vec::new();
    let mut crt = Crt::new();
    for i in instructions {
        // Unload to stack
        match i {
            i @ Instruction::AddX(_) => {
                stack.push(i);
                stack.push(Instruction::Noop);
            }
            i => stack.push(i),
        }
        // Reduce stack
        while !stack.is_empty() {
            crt.write(registry_x);
            if let Some(Instruction::AddX(x)) = stack.pop() {
                registry_x += x;
            }
            clock += 1;
            if clock == 20 || (clock - 60) % 40 == 0 {
                signal_strenghts.push((clock, registry_x));
            }
        }
    }
    println!("{}", &crt.output);
    let accumulated_strength = signal_strenghts.iter().map(|(c, r)| c * r).sum();
    (accumulated_strength, crt.output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;
    #[test]
    fn test() {
        assert_eq!(process(&inputs::demo_input(10)).0, 13140);
        assert_eq!(process(&inputs::task_input(10)).0, 13520);
    }
    #[test]
    fn test_crt_demo() {
        let crt = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
        "
        .trim();
        assert_eq!(process(&inputs::demo_input(10)).1.trim(), crt);
    }
    #[test]
    fn test_crt_task() {
        let crt = "
###...##..###..#..#.###..####..##..###..
#..#.#..#.#..#.#..#.#..#.#....#..#.#..#.
#..#.#....#..#.####.###..###..#..#.###..
###..#.##.###..#..#.#..#.#....####.#..#.
#....#..#.#....#..#.#..#.#....#..#.#..#.
#.....###.#....#..#.###..####.#..#.###..
        "
        .trim();
        assert_eq!(process(&inputs::task_input(10)).1.trim(), crt);
    }
}
