/// day 3: rucksack reorganization
use common::read_input_as_string;
use std::collections::HashSet;

fn item_priority(item: char) -> u8 {
    if item.is_uppercase() {
        item as u8 - 'A' as u8 + 27
    } else {
        item as u8 - 'a' as u8 + 1
    }
}

fn shared_item(rucksack: &str) -> char {
    let compartment1_items: HashSet<char> = (&rucksack[..rucksack.len() / 2]).chars().collect();
    let compartment2_items: HashSet<char> = (&rucksack[rucksack.len() / 2..]).chars().collect();

    compartment1_items
        .intersection(&compartment2_items)
        .next()
        .copied()
        .unwrap()
}

fn shared_item_between_elves(elves: &[String]) -> char {
    let mut items: HashSet<char> = elves[0].chars().collect();
    for elf in &elves[1..] {
        let elf_items: HashSet<char> = elf.chars().collect();
        let intersecting_items = items.intersection(&elf_items).map(|c| *c).collect();
        items = intersecting_items;
    }

    items.into_iter().next().unwrap()
}

fn score_rucksacks_part1(rucksacks: &str) -> u32 {
    rucksacks
        .trim()
        .split('\n')
        .map(|rucksack| shared_item(rucksack))
        .map(|item| item_priority(item) as u32)
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}

fn score_rucksacks_part2(rucksacks: &str) -> u32 {
    let rucksacks: Vec<String> = rucksacks
        .trim()
        .split('\n')
        .map(|s| String::from(s))
        .collect();

    rucksacks
        .as_slice()
        .chunks(3)
        .map(|elves| shared_item_between_elves(elves))
        .map(|item| item_priority(item) as u32)
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}

fn main() {
    let input = read_input_as_string(2022, 3).unwrap();
    println!("{}", score_rucksacks_part1(&input));
    println!("{}", score_rucksacks_part2(&input));
}

#[cfg(test)]
mod test {
    use super::{item_priority, score_rucksacks_part1, score_rucksacks_part2, shared_item};
    const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_item_priority() {
        assert_eq!(item_priority('a'), 1);
        assert_eq!(item_priority('z'), 26);
        assert_eq!(item_priority('A'), 27);
        assert_eq!(item_priority('B'), 28);
    }

    #[test]
    fn test_shared_item() {
        assert_eq!(shared_item("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
        assert_eq!(shared_item("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 'L');
        assert_eq!(shared_item("PmmdzqPrVvPwwTWBwg"), 'P');
        assert_eq!(shared_item("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 'v');
        assert_eq!(shared_item("ttgJtRGJQctTZtZT"), 't');
        assert_eq!(shared_item("CrZsJsPPZsGzwwsLwLmpwMDw"), 's');
    }

    #[test]
    fn test() {
        assert_eq!(score_rucksacks_part1(SAMPLE), 157);
        assert_eq!(score_rucksacks_part2(SAMPLE), 70);
    }
}
