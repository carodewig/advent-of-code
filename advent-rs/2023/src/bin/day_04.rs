// Scratchcards

use common::read_input_as_string;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref REGEX_NUMBER: Regex = Regex::new(r"[0-9]+").unwrap();
}

fn extract_numbers(line: &str) -> Vec<u32> {
    REGEX_NUMBER
        .find_iter(line)
        .filter_map(|m| m.as_str().parse().ok())
        .collect()
}

fn parse_card(line: &str) -> Option<(u32, HashSet<u32>, Vec<u32>)> {
    let parts: Vec<_> = line.split(|c| c == ':' || c == '|').collect();
    if parts.len() != 3 {
        return None;
    }

    let card_number = extract_numbers(parts[0])[0];
    let winning_numbers = extract_numbers(parts[1]).into_iter().collect();
    let have_numbers = extract_numbers(parts[2]);

    Some((card_number, winning_numbers, have_numbers))
}

fn count_winners(winning_numbers: HashSet<u32>, have_numbers: Vec<u32>) -> usize {
    have_numbers
        .into_iter()
        .filter(|n| winning_numbers.contains(n))
        .count()
}

fn count_card_pile(card_pile: &str) -> u32 {
    let mut total_points = 0;
    for line in card_pile.split('\n') {
        if let Some((_card_number, winning_numbers, have_numbers)) = parse_card(line) {
            let number_of_winners = count_winners(winning_numbers, have_numbers);
            if number_of_winners > 0 {
                total_points += 2_u32.pow((number_of_winners - 1) as u32);
            }
        }
    }

    total_points
}

fn count_total_scratchcards(card_pile: &str) -> usize {
    let mut total_scratchcards = 0;
    let mut card_multipliers: HashMap<u32, usize> = HashMap::default();

    for line in card_pile.split('\n') {
        if let Some((card_number, winning_numbers, have_numbers)) = parse_card(line) {
            {
                let this_card_entry = card_multipliers.entry(card_number).or_insert(0);
                *this_card_entry += 1;
            }

            let copies_of_this_card = *card_multipliers.get(&card_number).unwrap();
            total_scratchcards += copies_of_this_card;

            let number_of_winners = count_winners(winning_numbers, have_numbers);
            if number_of_winners > 0 {
                for offset in 1..=number_of_winners {
                    let entry = card_multipliers
                        .entry(card_number + (offset as u32))
                        .or_insert(0);
                    *entry += copies_of_this_card;
                }
            }
        }
    }

    total_scratchcards
}

fn main() {
    let card_pile = read_input_as_string(2023, 4).unwrap();
    println!("{}", count_card_pile(&card_pile));
    println!("{}", count_total_scratchcards(&card_pile));
}

#[cfg(test)]
mod tests {
    use super::{count_card_pile, count_total_scratchcards};

    const SAMPLE: &str = r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#;

    #[test]
    fn test() {
        assert_eq!(13, count_card_pile(SAMPLE));
        assert_eq!(30, count_total_scratchcards(SAMPLE));
    }
}
