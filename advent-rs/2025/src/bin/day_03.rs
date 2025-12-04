use common::read_input_as_string;

type Bank = Vec<u8>;

fn parse_input(input: &str) -> Vec<Bank> {
    input
        .split('\n')
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let chars = line.chars();
            chars
                .into_iter()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn find_maximum_index(elems: &[u8]) -> usize {
    let (index, _value) = elems
        .iter()
        .copied()
        .enumerate()
        .reduce(|(max_index, max_value), (index, value)| {
            if value > max_value {
                (index, value)
            } else {
                (max_index, max_value)
            }
        })
        .unwrap();
    index
}

fn maximum_joltage(bank: &Bank, num_batteries: usize) -> usize {
    let mut value = 0;
    let mut current_index = 0;
    for remaining_batteries in (1..=num_batteries).rev() {
        let remaining_bank = &bank[current_index..=bank.len() - remaining_batteries];
        let index = current_index + find_maximum_index(remaining_bank);

        value *= 10;
        value += bank[index] as usize;

        current_index = index + 1;
    }

    value
}

fn part1(banks: &[Bank]) -> usize {
    banks.iter().map(|b| maximum_joltage(b, 2)).sum()
}

fn part2(banks: &[Bank]) -> usize {
    banks.iter().map(|b| maximum_joltage(b, 12)).sum()
}

fn main() {
    let banks = parse_input(&read_input_as_string(2025, 3).unwrap());
    assert_eq!(part1(&banks), 17_316);
    assert_eq!(part2(&banks), 171_741_365_473_332);
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};

    const SAMPLE: &str = "\
        987654321111111
        811111111111119
        234234234234278
        818181911112111";

    #[test]
    fn test_part1() {
        let banks = parse_input(SAMPLE);
        assert_eq!(part1(&banks), 357);
    }

    #[test]
    fn test_part2() {
        let banks = parse_input(SAMPLE);
        assert_eq!(part2(&banks), 3_121_910_778_619);
    }
}
