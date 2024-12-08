// Bridge Repair

use common::read_input_as_string;
use itertools::{repeat_n, Itertools};

fn main() {
    println!("{}", part1(&read_input_as_string(2024, 7).unwrap()));
    println!("{}", part2(&read_input_as_string(2024, 7).unwrap()));
}

fn sum_valid_equations(input: &str, operators: &[&str]) -> usize {
    let equations = parse(input);
    equations
        .into_iter()
        .filter(|equation| equation.has_solution(operators))
        .map(|equation| equation.total)
        .sum()
}

fn part1(input: &str) -> usize {
    sum_valid_equations(input, &["+", "*"])
}

fn part2(input: &str) -> usize {
    // NB: doing this properly (ie dynamic programming to build up partial solutions)
    //  would be nice, but brute force finishes in <20s so not going to bother ¯\_(ツ)_/¯
    sum_valid_equations(input, &["+", "*", "||"])
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Equation {
    total: usize,
    values: Vec<usize>,
}

impl Equation {
    fn new(total: usize, values: Vec<usize>) -> Self {
        Self { total, values }
    }
    fn evaluate_left_to_write(&self, operators: &[&&str]) -> usize {
        let mut total = self.values[0];
        for (operator, next_value) in operators.iter().zip(self.values.iter().skip(1)) {
            match **operator {
                "+" => total += next_value,
                "*" => total *= next_value,
                "||" => total = format!("{total}{next_value}").parse().unwrap(),
                _ => {}
            }
        }

        total
    }

    fn has_solution(&self, operators: &[&str]) -> bool {
        // permutations with replacement (itertools `permutations` fn does not replace)
        let permutations = repeat_n(operators, self.values.len() - 1).multi_cartesian_product();
        for operator_set in permutations {
            let result = self.evaluate_left_to_write(&operator_set);
            if result == self.total {
                return true;
            }
        }

        false
    }
}

fn parse(input: &str) -> Vec<Equation> {
    let mut equations = Vec::default();
    for line in input.split('\n').filter(|l| !l.trim().is_empty()) {
        let mut split_by_colon = line.trim().split(':');
        let total = split_by_colon.next().unwrap().parse().unwrap();
        let values_str = split_by_colon.next().unwrap();
        let values = values_str
            .split_whitespace()
            .filter_map(|v| v.parse().ok())
            .collect();
        equations.push(Equation { total, values });
    }

    equations
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2, Equation};

    const SAMPLE: &str = "\
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    ";

    #[test]
    fn test_parse() {
        let equations = parse(SAMPLE);
        assert_eq!(
            equations[0],
            Equation {
                total: 190,
                values: vec![10, 19]
            }
        );
    }

    #[test]
    fn test_has_solution() {
        const OPERATORS: [&str; 2] = ["+", "*"];

        let equation = Equation::new(190, vec![10, 19]);
        assert!(equation.has_solution(&OPERATORS));

        let equation = Equation::new(3267, vec![81, 40, 27]);
        assert!(equation.has_solution(&OPERATORS));

        let equation = Equation::new(292, vec![11, 6, 16, 20]);
        assert!(equation.has_solution(&OPERATORS));

        let equation = Equation::new(83, vec![17, 5]);
        assert!(!equation.has_solution(&OPERATORS));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 11387);
    }
}
