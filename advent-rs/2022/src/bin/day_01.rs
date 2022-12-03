/// day 1: calorie counting
use common::read_input_as_string;

fn sum(vector: impl Iterator<Item = i32>) -> Option<i32> {
    vector.reduce(|a, b| a + b)
}

/// Get the total number of calories carried by each elf.
fn elf_calories(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .flat_map(|elf_lines| {
            sum(elf_lines
                .split('\n')
                .map(|s| i32::from_str_radix(s, 10).unwrap()))
        })
        .collect()
}

/// Get the sum of the top N calorie values.
fn top_n_calories(mut calories: Vec<i32>, n: usize) -> i32 {
    calories.sort_by_key(|c| -c);
    sum(calories.into_iter().take(n)).unwrap_or(0)
}

fn main() {
    let input = read_input_as_string(2022, 1).unwrap();
    println!("{:?}", (top_n_calories(elf_calories(&input), 1)));
    println!("{:?}", (top_n_calories(elf_calories(&input), 3)));
}

#[cfg(test)]
mod test {
    use super::{elf_calories, top_n_calories};
    const SAMPLE: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn test() {
        println!("{:?}", elf_calories(SAMPLE));
        assert_eq!(24000, top_n_calories(elf_calories(SAMPLE), 1));
        assert_eq!(45000, top_n_calories(elf_calories(SAMPLE), 3));
    }
}
