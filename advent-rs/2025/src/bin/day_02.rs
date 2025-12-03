use common::read_input_as_string;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .split(',')
        .map(|range| {
            let mut range_split = range.trim().split('-');
            let start = range_split.next().unwrap();
            let end = range_split.next().unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect()
}

fn repeats(value_chars: &[char], match_len: usize) -> bool {
    let value_len = value_chars.len();
    if !value_len.is_multiple_of(match_len) {
        return false;
    }

    let match_chars = &value_chars[..match_len];
    for index in (match_len..value_len).step_by(match_len) {
        if match_chars != &value_chars[index..index + match_len] {
            return false;
        }
    }

    true
}

fn is_repeater_part1(value: usize) -> bool {
    let num_digits = value.ilog10() + 1;
    if !num_digits.is_multiple_of(2) {
        // odd number of digits
        return false;
    }

    let offset = 10_usize.pow(num_digits / 2);
    value.div_euclid(offset) == value.rem_euclid(offset)
}

fn is_repeater_part2(value: usize) -> bool {
    let chars: Vec<char> = value.to_string().chars().collect();
    for repeated_len in 1..=chars.len() / 2 {
        if repeats(&chars, repeated_len) {
            return true;
        }
    }

    false
}

fn sum_repeaters(ranges: &[(usize, usize)], repeater_fn: fn(usize) -> bool) -> usize {
    let mut total = 0;
    for (start, end) in ranges {
        for value in *start..=*end {
            if repeater_fn(value) {
                total += value;
            }
        }
    }

    total
}

fn main() {
    let ranges = parse_input(&read_input_as_string(2025, 2).unwrap());
    assert_eq!(sum_repeaters(&ranges, is_repeater_part1), 17_077_011_375);
    assert_eq!(sum_repeaters(&ranges, is_repeater_part2), 36_037_497_037);
}

#[cfg(test)]
mod tests {
    use super::{is_repeater_part1, is_repeater_part2, parse_input, sum_repeaters};
    const SAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        let ranges = parse_input(SAMPLE);
        assert_eq!(sum_repeaters(&ranges, is_repeater_part1), 1_227_775_554);
    }

    #[test]
    fn test_part2() {
        let ranges = parse_input(SAMPLE);
        assert_eq!(sum_repeaters(&ranges, is_repeater_part2), 4_174_379_265);
    }
}
