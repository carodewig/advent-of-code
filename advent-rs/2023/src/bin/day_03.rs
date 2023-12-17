// Gear Ratios

use common::read_input_as_string;
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Location = (usize, usize);

fn scan_for_symbol_locations(schematic: &str) -> HashMap<Location, char> {
    let mut locations = HashMap::default();
    for (y, line) in schematic.split('\n').enumerate() {
        for (x, character) in line.char_indices() {
            if character.is_ascii_punctuation() && character != '.' {
                locations.insert((x, y), character);
            }
        }
    }

    locations
}

// returns map<location, part_id>, map<part_id, part_number> since part numbers are not unique
fn scan_for_part_locations(schematic: &str) -> (HashMap<Location, u32>, HashMap<u32, u32>) {
    let mut locations = HashMap::default();
    let mut part_ids = HashMap::default();
    let mut part_id = 0;

    let part_number_regex = Regex::new(r"[0-9]{1,3}").unwrap();

    for (y, line) in schematic.split('\n').enumerate() {
        for re_match in part_number_regex.find_iter(line) {
            let part_number: u32 = re_match.as_str().parse().unwrap();
            for x in re_match.start()..re_match.end() {
                locations.insert((x, y), part_id);
            }

            part_ids.insert(part_id, part_number);
            part_id += 1;
        }
    }

    (locations, part_ids)
}

fn has_nearby_symbol(
    locations: &HashMap<Location, char>,
    y: usize,
    start_x: usize,
    end_x: usize,
) -> bool {
    let mut check_locations = Vec::default();

    // end_x already represents the index beyond the end of the word - make start_x represent the
    // index before the start of the word
    let start_x = start_x.saturating_sub(1);

    let start_y = y.saturating_sub(1);
    let end_y = y + 1;

    // we can check the same indexes as the known part number - won't hurt anything
    for y in start_y..=end_y {
        for x in start_x..=end_x {
            check_locations.push((x, y));
        }
    }

    check_locations
        .into_iter()
        .any(|loc| locations.contains_key(&loc))
}

fn sum_part_numbers(schematic: &str) -> u32 {
    let symbols = scan_for_symbol_locations(schematic);
    let part_number_regex = Regex::new(r"[0-9]+").unwrap();

    let mut total = 0;
    for (y, line) in schematic.split('\n').enumerate() {
        for re_match in part_number_regex.find_iter(line) {
            let part_number: u32 = re_match.as_str().parse().unwrap();
            // see if there's a symbol around this part number
            // we can use match.start() and end() since these are single byte characters!
            if has_nearby_symbol(&symbols, y, re_match.start(), re_match.end()) {
                total += part_number;
            }
        }
    }

    total
}

fn gear_ratio(
    gear_location: Location,
    part_locations: &HashMap<Location, u32>,
    part_ids: &HashMap<u32, u32>,
) -> Option<u32> {
    let mut part_id_set: HashSet<u32> = HashSet::default();

    let start_x = gear_location.0.saturating_sub(1);
    let start_y = gear_location.1.saturating_sub(1);
    let end_x = gear_location.0 + 1;
    let end_y = gear_location.1 + 1;

    for x in start_x..=end_x {
        for y in start_y..=end_y {
            if let Some(part_id) = part_locations.get(&(x, y)) {
                part_id_set.insert(*part_id);
            }
        }
    }

    if part_id_set.len() == 2 {
        return part_id_set
            .iter()
            .filter_map(|id| part_ids.get(id))
            .copied()
            .reduce(|acc, e| acc * e);
    }

    None
}

fn sum_gear_ratios(schematic: &str) -> u32 {
    let symbols = scan_for_symbol_locations(schematic);
    let (part_locations, part_ids) = scan_for_part_locations(schematic);

    let mut total = 0;
    for (possible_gear_location, _) in symbols
        .into_iter()
        .filter(|(_, character)| character == &'*')
    {
        if let Some(ratio) = gear_ratio(possible_gear_location, &part_locations, &part_ids) {
            total += ratio;
        }
    }

    total
}

fn main() {
    let schematic = read_input_as_string(2023, 3).unwrap();
    println!("{}", sum_part_numbers(&schematic));
    println!("{}", sum_gear_ratios(&schematic));
}

#[cfg(test)]
mod tests {
    use crate::{sum_gear_ratios, sum_part_numbers};

    const SAMPLE: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

    #[test]
    fn test() {
        assert_eq!(4361, sum_part_numbers(SAMPLE));
        assert_eq!(467835, sum_gear_ratios(SAMPLE));
        assert_eq!(1170, sum_gear_ratios(".2.\n.*.\n585"))
    }
}
