// Claw Contraption

use common::Location;
use common::{read_input_as_string, Step};
use regex::Regex;
use std::num::ParseIntError;

fn main() {
    let input = read_input_as_string(2024, 13).unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> isize {
    let machines = parse(input);
    machines
        .into_iter()
        .filter_map(|machine| {
            println!("{machine:?}");
            machine.cheapest_way_to_win()
        })
        .sum()
}

fn part2(input: &str) -> isize {
    let machines = parse(input);
    let step_offset = Step {
        x: 10000000000000,
        y: 10000000000000,
    };
    machines
        .into_iter()
        .map(|machine| ClawMachine {
            prize: machine.prize + step_offset,
            ..machine
        })
        .filter_map(|machine| {
            println!("{machine:?}");
            machine.cheapest_way_to_win()
        })
        .sum()
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
struct ClawMachine {
    button_a: Step,
    button_b: Step,
    prize: Location,
}

impl ClawMachine {
    fn max_presses(prize: Location, button: Step) -> isize {
        (prize.x / button.x).min(prize.y / button.y)
    }

    fn cheapest_way_to_win(&self) -> Option<isize> {
        // take advantage of a key property of the input: one button will have x > y and the other
        // will have x < y
        let mut presses_b = Self::max_presses(self.prize, self.button_b);
        while presses_b >= 0 {
            println!("presses_b = {presses_b}");
            let remainder = self.prize - self.button_b * presses_b;
            let presses_a = Self::max_presses(remainder, self.button_a);
            if self.button_a * presses_a + self.button_b * presses_b == self.prize {
                return Some(3 * presses_a + presses_b);
            }

            if remainder.x > remainder.y {
                let presses_needed_a = remainder.x / self.button_a.x;
                let overload_y = remainder.y - self.button_a.y * presses_needed_a;

                if overload_y < 0 {
                    let presses_needed_b = Self::max_presses(
                        self.prize - self.button_a * presses_needed_a,
                        self.button_b,
                    );
                    presses_b = (presses_b - 1).min(presses_needed_b);
                } else {
                    presses_b -= (overload_y / self.button_b.y).max(1);
                }
            } else {
                let presses_needed_a = remainder.y / self.button_a.y;
                let overload_x = remainder.x - self.button_a.x * presses_needed_a;

                if overload_x < 0 {
                    let presses_needed_b = Self::max_presses(
                        self.prize - self.button_a * presses_needed_a,
                        self.button_b,
                    );
                    presses_b = (presses_b - 1).min(presses_needed_b);
                } else {
                    presses_b -= (overload_x / self.button_b.x).max(1);
                }
            }
        }
        None
    }
}

impl TryFrom<[&str; 6]> for ClawMachine {
    type Error = ParseIntError;

    fn try_from(values: [&str; 6]) -> Result<Self, Self::Error> {
        let [step_a_x, step_a_y, step_b_x, step_b_y, prize_x, prize_y] = values;
        Ok(Self {
            button_a: Step {
                x: step_a_x.parse()?,
                y: step_a_y.parse()?,
            },
            button_b: Step {
                x: step_b_x.parse()?,
                y: step_b_y.parse()?,
            },
            prize: Location {
                x: prize_x.parse()?,
                y: prize_y.parse()?,
            },
        })
    }
}

fn parse(input: &str) -> Vec<ClawMachine> {
    let mut machines = Vec::default();
    let regex = Regex::new(r" *Button A: X\+([0-9]+), Y\+([0-9]+)\n *Button B: X\+([0-9]+), Y\+([0-9]+)\n *Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    for (_, matches) in regex.captures_iter(input).map(|c| c.extract()) {
        if let Ok(machine) = ClawMachine::try_from(matches) {
            machines.push(machine)
        }
    }

    machines
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2, ClawMachine};
    use common::{Location, Step};

    const SAMPLE: &str = "
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279";

    #[test]
    fn test_parse() {
        let machines = parse(SAMPLE);
        assert_eq!(
            machines[0],
            ClawMachine {
                button_a: Step { x: 94, y: 34 },
                button_b: Step { x: 22, y: 67 },
                prize: Location { x: 8400, y: 5400 },
            }
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 875318608908);
    }
}
