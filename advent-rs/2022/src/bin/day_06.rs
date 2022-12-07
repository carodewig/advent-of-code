/// day 6: tuning trouble
use common::read_input_as_string;
use std::collections::HashSet;

fn start_index(input: &str, num_distinct_chars: usize) -> Option<usize> {
    let mut index = num_distinct_chars;
    while index <= input.len() {
        let packet = &input[index - num_distinct_chars..index];
        let unique_elems: HashSet<char> = packet.chars().collect();
        let num_overlaps = packet.len() - unique_elems.len();
        if num_overlaps == 0 {
            return Some(index);
        } else {
            // minor performance enhancement: we know we need at least `num_overlaps`
            // new characters to have a shot at having all unique elements, so just skip ahead
            index += num_overlaps;
        }
    }

    None
}

fn main() {
    let input = read_input_as_string(2022, 6).unwrap();
    println!("{:?}", start_index(input.trim(), 4));
    println!("{:?}", start_index(input.trim(), 14));
}

#[cfg(test)]
mod test {
    use crate::start_index;

    #[test]
    fn test() {
        assert_eq!(Some(4), start_index("abcd", 4));
        assert_eq!(Some(7), start_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(Some(5), start_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(Some(6), start_index("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(
            Some(10),
            start_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)
        );
        assert_eq!(Some(11), start_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
        assert_eq!(Some(19), start_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
        assert_eq!(Some(23), start_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(Some(23), start_index("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(
            Some(29),
            start_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14)
        );
        assert_eq!(
            Some(26),
            start_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14)
        );
    }
}
