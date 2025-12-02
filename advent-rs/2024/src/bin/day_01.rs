// Historian Hysteria

use common::read_input_as_lines;
use std::collections::HashMap;

fn main() {
    let lines = read_input_as_lines(2024, 1).expect("Unable to fetch input");

    let list1: Vec<u64> = parse_list(&lines, 0);
    let list2: Vec<u64> = parse_list(&lines, 1);
    println!("{}", total_distance(list1, list2));

    let list1: Vec<u64> = parse_list(&lines, 0);
    let list2: Vec<u64> = parse_list(&lines, 1);
    println!("{}", similarity_scores(list1, list2));
}

fn parse_list(lines: &[String], position: usize) -> Vec<u64> {
    lines
        .iter()
        .filter_map(|line| line.split_whitespace().nth(position))
        .filter_map(|value| value.parse().ok())
        .collect()
}

fn total_distance(mut list1: Vec<u64>, mut list2: Vec<u64>) -> u64 {
    list1.sort_unstable();
    list2.sort_unstable();
    list1
        .into_iter()
        .zip(list2)
        .fold(0, |acc, (a, b)| acc + a.abs_diff(b))
}

fn similarity_scores(list1: Vec<u64>, list2: Vec<u64>) -> u64 {
    let mut frequencies1 = extract_frequencies(list1);
    let frequencies2 = extract_frequencies(list2);

    frequencies1.drain().fold(0, |acc, (element, frequency1)| {
        acc + element * frequency1 * frequencies2.get(&element).unwrap_or(&0)
    })
}

fn extract_frequencies(l: Vec<u64>) -> HashMap<u64, u64> {
    let mut frequencies = HashMap::default();
    for element in l {
        let entry = frequencies.entry(element).or_insert(0);
        *entry += 1;
    }
    frequencies
}
