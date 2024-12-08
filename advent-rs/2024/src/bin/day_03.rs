// Mull It Over

use common::read_input_as_string;
use regex::Regex;
use std::ops::Range;

const MUL_PATTERN: &str = r"mul\(([0-9]{1,3}),([0-9]{1,3})\)";
const DO_PATTERN: &str = r"do\(\)";
const DONT_PATTERN: &str = r"don't\(\)";

fn main() {
    println!("{}", part1(&read_input_as_string(2024, 3).unwrap()));
    println!("{}", part2(&read_input_as_string(2024, 3).unwrap()));
}

fn part1(memory: &str) -> usize {
    let mul_regex = Regex::new(MUL_PATTERN).unwrap();
    let mut total = 0;

    for (_, [x, y]) in mul_regex.captures_iter(&memory).map(|c| c.extract()) {
        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();
        total += x * y;
    }

    total
}

fn part2(memory: &str) -> usize {
    let mul_regex = Regex::new(MUL_PATTERN).unwrap();

    let mut total = 0;

    let enabled_ranges = determine_enabled_offsets(memory);

    let mul_offsets: Vec<usize> = mul_regex.find_iter(&memory).map(|m| m.start()).collect();
    for (capture, offset) in mul_regex
        .captures_iter(&memory)
        .zip(mul_offsets.into_iter())
    {
        // see if this is a useful offset
        if !enabled_ranges.iter().any(|r| r.contains(&offset)) {
            continue;
        }

        let (_, [x, y]) = capture.extract();

        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();
        total += x * y;
    }

    total
}

fn determine_enabled_offsets(memory: &str) -> Vec<Range<usize>> {
    let do_regex = Regex::new(DO_PATTERN).unwrap();
    let dont_regex = Regex::new(DONT_PATTERN).unwrap();

    let mut do_offsets: Vec<usize> = do_regex.find_iter(&memory).map(|m| m.start()).collect();
    let dont_offsets: Vec<usize> = dont_regex.find_iter(&memory).map(|m| m.start()).collect();
    do_offsets.insert(0, 0);

    let mut enabled_ranges: Vec<Range<usize>> = Vec::default();
    let (mut i, mut j) = (0, 0);

    while i < do_offsets.len() && j < dont_offsets.len() {
        let start = do_offsets[i];
        let mut end = dont_offsets[j];
        while end < start && j < dont_offsets.len() - 1 {
            j += 1;
            end = dont_offsets[j];
        }

        if start < end {
            enabled_ranges.push(start..end);
            i += 1;
        } else {
            enabled_ranges.push(start..usize::MAX);
            break;
        }
    }

    enabled_ranges
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        const SAMPLE: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(&SAMPLE), 161);
    }

    #[test]
    fn test_part2() {
        const SAMPLE: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(&SAMPLE), 48);
    }
}
