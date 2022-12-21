use std::collections::HashMap;

use itertools::Itertools;

fn evaluate(
    mid: &str,
    results: &mut HashMap<&str, i64>,
    inputs: &HashMap<&str, (&str, &str, &str)>,
) -> (i64, bool) {
    let node_hooman = mid == "humn";

    match results.get(mid) {
        Some(res) => (*res, node_hooman),
        None => {
            let (a, op, b) = inputs.get(mid).unwrap();
            let (a, a_h) = evaluate(a, results, inputs);
            let (b, b_h) = evaluate(b, results, inputs);
            let contains_hooman = a_h || b_h || node_hooman;
            match *op {
                "+" => (a + b, contains_hooman),
                "-" => (a - b, contains_hooman),
                "*" => (a * b, contains_hooman),
                "/" => (a / b, contains_hooman),
                _ => unreachable!(),
            }
        }
    }
}

fn parse(input: &str) -> (HashMap<&str, (&str, &str, &str)>, HashMap<&str, i64>) {
    let mut inputs = HashMap::new();
    let mut results = HashMap::new();
    for line in input.split("\n") {
        let (mid, expr) = line.split(": ").next_tuple().unwrap();
        match expr.parse::<i64>() {
            Ok(res) => {
                results.insert(mid, res);
            }
            _ => {
                let (a, op, b) = expr.split(" ").next_tuple().unwrap();
                inputs.insert(mid, (a, op, b));
            }
        }
    }
    (inputs, results)
}

pub fn monkeys1(input: &str) -> i64 {
    let (inputs, mut results) = parse(input);

    evaluate("root", &mut results, &inputs).0
}

pub fn monkeys2(input: &str) -> i64 {
    let (inputs, mut results) = parse(input);

    let left = inputs.get("root").unwrap().0;
    let right = inputs.get("root").unwrap().2;
    let left_val = evaluate(left, &mut results, &inputs);
    let right_val = evaluate(right, &mut results, &inputs);
    let mut result: i64 = if left_val.1 == true {
        right_val.0
    } else {
        left_val.0
    };
    let mut input = inputs
        .get(if left_val.1 == true { left } else { right })
        .unwrap();
    loop {
        let (left, op, right) = input;
        let (left_val, left_hooman) = evaluate(left, &mut results, &inputs);
        let (right_val, _) = evaluate(right, &mut results, &inputs);
        let other_val = if left_hooman != true {
            left_val
        } else {
            right_val
        };
        result = match *op {
            "+" => result - other_val,
            "*" => result / other_val,
            "-" if left_hooman == true => result + other_val,
            "-" => other_val - result,
            "/" if left_hooman == true => result * other_val,
            "/" => other_val / result,
            _ => unreachable!(),
        };
        if left == &"humn" || right == &"humn" {
            break;
        }
        input = inputs
            .get(if left_hooman == true { left } else { right })
            .unwrap();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;
    #[test]
    fn test() {
        assert_eq!(monkeys1(&inputs::demo_input(21)), 152);
        assert_eq!(monkeys1(&inputs::task_input(21)), 63119856257960);
        assert_eq!(monkeys2(&inputs::demo_input(21)), 301);
        assert_eq!(monkeys2(&inputs::task_input(21)), 3006709232464);
    }
}
