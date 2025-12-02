// Plutonian Pebbles

use common::read_input_as_string;
use std::collections::HashMap;

fn main() {
    let input_str = read_input_as_string(2024, 11).unwrap();
    println!("{}", run(&input_str, 25));
    println!("{}", run(&input_str, 75));
}

fn run(input: &str, num_blinks: usize) -> usize {
    let mut total = 0;
    let mut stone_cache = StoneCache::default();

    for stone in input.split_whitespace().filter_map(|s| s.parse().ok()) {
        total += stone_cache.recursive_blink(stone, num_blinks);
    }
    total
}

// (stone, num blinks) -> num stones
#[derive(Debug, Default)]
struct StoneCache(HashMap<(usize, usize), usize>);
impl StoneCache {
    fn recursive_blink(&mut self, stone: usize, num_blinks: usize) -> usize {
        if num_blinks == 0 {
            return 1;
        }

        let key = (stone, num_blinks);
        if let Some(num_stones) = self.0.get(&key) {
            return *num_stones;
        }

        let stone_str = stone.to_string();
        let length = stone_str.len();

        let num_stones = match stone {
            0 => self.recursive_blink(1, num_blinks - 1),
            _ if length.is_multiple_of(2) => {
                let (left, right) = split_stone(&stone_str);
                self.recursive_blink(left, num_blinks - 1)
                    + self.recursive_blink(right, num_blinks - 1)
            }
            _ => self.recursive_blink(stone * 2024, num_blinks - 1),
        };

        self.0.insert((stone, num_blinks), num_stones);

        num_stones
    }
}

fn split_stone(stone: &str) -> (usize, usize) {
    let length = stone.len();

    let left = stone.get(..length / 2).unwrap();
    let right = stone.get(length / 2..).unwrap();
    (left.parse().unwrap(), right.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_solution() {
        assert_eq!(run("125 17", 6), 22);
        assert_eq!(run("125 17", 25), 55312);
    }
}
