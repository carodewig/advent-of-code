/// day 5: supply stacks
use common::read_input_as_string;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

fn parse_as_crates(input: &str) -> HashMap<usize, VecDeque<char>> {
    // vec<0> == top of stack
    let mut map: HashMap<usize, VecDeque<char>> = HashMap::default();
    for line in input.split('\n') {
        if line.starts_with(" 1") {
            break;
        }

        // not being real careful with parsing here
        // we know that indices 1, 5, 9, etc correspond to stacks 1, 2, 3, etc
        // so just pick out those elements and add them if they're not spaces
        for (index_0, crate_char) in line.chars().enumerate() {
            if (index_0 + 3) % 4 == 0 && crate_char != ' ' {
                let stack = (index_0 + 3) / 4;
                map.entry(stack).or_default().push_back(crate_char);
            }
        }
    }

    map
}

fn move_crates_part1(input: &str, crates: &mut HashMap<usize, VecDeque<char>>) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in input.split('\n') {
        if let Some(caps) = re.captures(line) {
            let num = usize::from_str_radix(&caps[1], 10).unwrap();
            let crate_from = usize::from_str_radix(&caps[2], 10).unwrap();
            let crate_to = usize::from_str_radix(&caps[3], 10).unwrap();

            for _ in 0..num {
                if let Some(crate_char) = crates.get_mut(&crate_from).and_then(|v| v.pop_front()) {
                    crates.entry(crate_to).or_default().push_front(crate_char);
                }
            }
        }
    }
}

fn move_crates_part2(input: &str, crates: &mut HashMap<usize, VecDeque<char>>) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in input.split('\n') {
        if let Some(caps) = re.captures(line) {
            let num = usize::from_str_radix(&caps[1], 10).unwrap();
            let crate_from = usize::from_str_radix(&caps[2], 10).unwrap();
            let crate_to = usize::from_str_radix(&caps[3], 10).unwrap();

            let mut crates_to_move: Vec<char> = (0..num)
                .filter_map(|_| crates.get_mut(&crate_from).and_then(|v| v.pop_front()))
                .collect();
            crates_to_move.reverse();
            for crate_char in crates_to_move.into_iter() {
                crates.entry(crate_to).or_default().push_front(crate_char);
            }
        }
    }
}

fn top_crates(
    input: &str,
    move_crates_fn: fn(&str, &mut HashMap<usize, VecDeque<char>>),
) -> String {
    let mut crates = parse_as_crates(input);
    move_crates_fn(input, &mut crates);

    let mut output = String::default();
    let mut crate_indices: Vec<usize> = crates.keys().cloned().collect();
    crate_indices.sort();
    for crate_index in crate_indices.into_iter() {
        if let Some(crate_char) = crates.get(&crate_index).and_then(|v| v.front()) {
            output.push(*crate_char);
        }
    }

    output
}

fn main() {
    let input = read_input_as_string(2022, 5).unwrap();
    println!("{}", top_crates(&input, move_crates_part1));
    println!("{}", top_crates(&input, move_crates_part2));
}

#[cfg(test)]
mod test {
    use crate::{move_crates_part1, move_crates_part2, top_crates};
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2"};

    #[test]
    fn test() {
        assert_eq!("CMZ", &top_crates(SAMPLE, move_crates_part1));
        assert_eq!("MCD", &top_crates(SAMPLE, move_crates_part2));
    }
}
