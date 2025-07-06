// Claw Contraption

use common::read_input_as_string;
use regex::Regex;
use std::num::ParseIntError;
use std::ops::Add;

fn main() {
    let input = read_input_as_string(2024, 13).unwrap();
    println!("{}", part1(&input));
    // println!("{}", part2(&input));
}

fn part1(input: &str) -> usize {
    let machines = parse(input);
    machines
        .into_iter()
        .filter_map(|machine| {
            println!("{machine:?}");
            machine.cheapest_way_to_win()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let machines = parse(input);
    machines
        .into_iter()
        .map(|machine| ClawMachine {
            prize: Location {
                x: machine.prize.x + 10000000000000,
                y: machine.prize.y + 10000000000000,
            },
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
    fn cheapest_way_to_win(&self) -> Option<usize> {
        // treat as a system of equations
        // prize.x = a * button_a.x + b * button_b.x
        // prize.y = a * button_a.y + b * button_b.y
        // cost = 3 * a + b
        //
        // constraints
        // 0 <= a <= min(prize.x / button_a.x, prize.y / button_a.y)
        //   '' b ''
        // should become a linear optimization problem, solving for a and b...
        // probably need simplex
        // but for now just hit it with a stick
        let max_a_presses = (self.prize.x / self.button_a.x).min(self.prize.y / self.button_a.y);

        let mut min_cost = usize::MAX;
        for a_presses in 0..(max_a_presses + 1) {
            println!("{a_presses} / {max_a_presses}");
            // solve for b
            if (self.prize.x - a_presses * self.button_a.x) % self.button_b.x == 0 {
                let b_presses = (self.prize.x - a_presses * self.button_a.x) / self.button_b.x;
                if self.prize.y == a_presses * self.button_a.y + b_presses * self.button_b.y {
                    let cost = 3 * a_presses + b_presses;
                    min_cost = min_cost.min(cost);
                }
            }
        }

        if min_cost == usize::MAX {
            None
        } else {
            Some(min_cost)
        }
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

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Step {
    x: usize,
    y: usize,
}

impl Add<Step> for Location {
    type Output = Location;

    fn add(self, rhs: Step) -> Self::Output {
        Location {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{parse, part1, ClawMachine, Location, Step};

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
}
