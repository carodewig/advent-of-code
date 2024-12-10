// Mirage Maintenance

use common::read_input_as_string;

fn main() {
    println!("{}", part1(&read_input_as_string(2023, 9).unwrap()));
    println!("{}", part2(&read_input_as_string(2023, 9).unwrap()));
}

fn part1(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|sequence| extrapolate_next_value(&sequence))
        .sum()
}

fn part2(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|sequence| extrapolate_previous_value(&sequence))
        .sum()
}

fn sequence_difference(sequence: &[i64]) -> Vec<i64> {
    let mut difference = Vec::with_capacity(sequence.len() - 1);
    for (a, b) in sequence.iter().zip(sequence.iter().skip(1)) {
        difference.push(b - a);
    }

    difference
}

fn extrapolate_next_value(sequence: &[i64]) -> i64 {
    if sequence.iter().all(|x| *x == 0) {
        0
    } else {
        sequence[sequence.len() - 1] + extrapolate_next_value(&sequence_difference(sequence))
    }
}

fn extrapolate_previous_value(sequence: &[i64]) -> i64 {
    if sequence.iter().all(|x| *x == 0) {
        0
    } else {
        sequence[0] - extrapolate_previous_value(&sequence_difference(sequence))
    }
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE: &str = "\
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 2);
    }
}
