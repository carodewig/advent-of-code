//! Secret Entrance

use common::read_input_as_string;
use regex::Regex;

enum Rotation {
    Left(i64),
    Right(i64),
}

impl Rotation {
    fn from_regex(re: &Regex, line: &str) -> Option<Self> {
        let caps = re.captures(line)?;
        let steps = caps.get(2)?.as_str().parse().ok()?;
        match caps.get(1)?.as_str() {
            "L" => Some(Self::Left(steps)),
            "R" => Some(Self::Right(steps)),
            _ => unreachable!(),
        }
    }
}

fn parse_rotations(lines: &str) -> Vec<Rotation> {
    let re = Regex::new(r"([LR])([0-9]+)").unwrap();
    lines
        .split('\n')
        .filter_map(|line| Rotation::from_regex(&re, line))
        .collect()
}

fn part1(rotations: &[Rotation]) -> i64 {
    let mut value = 50;
    let mut count = 0;
    for rotation in rotations {
        match rotation {
            Rotation::Left(steps) => {
                value -= steps % 100;
            }
            Rotation::Right(steps) => {
                value += steps % 100;
            }
        }

        value = value.rem_euclid(100);
        if value == 0 {
            count += 1;
        }
    }

    count
}

fn part2(rotations: &[Rotation]) -> i64 {
    let mut value = 50;
    let mut count = 0;
    for rotation in rotations {
        let prev_zero = value == 0;

        match rotation {
            Rotation::Left(steps) => {
                count += steps / 100;
                value -= steps % 100;
            }
            Rotation::Right(steps) => {
                count += steps / 100;
                value += steps % 100;
            }
        }

        if !prev_zero && !(0..=100).contains(&value) {
            // Passed through zero on the way to this value - increment count
            count += 1;
        }

        value = value.rem_euclid(100);
        if value == 0 {
            count += 1;
        }
    }

    count
}

fn main() {
    let rotations = parse_rotations(&read_input_as_string(2025, 1).unwrap());
    assert_eq!(part1(&rotations), 1150);
    assert_eq!(part2(&rotations), 6738);
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::{parse_rotations, part2};

    const SAMPLE: &str = "\
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_rotations(SAMPLE)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_rotations(SAMPLE)), 6);
    }
}
