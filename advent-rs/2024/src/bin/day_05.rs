// Print Queue

use common::read_input_as_string;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", part1(&read_input_as_string(2024, 5).unwrap()));
    println!("{}", part2(&read_input_as_string(2024, 5).unwrap()));
}

fn part1(input: &str) -> u32 {
    let (rules, updates) = parse(input);
    let rule_set = RuleSet::from(rules.iter());

    updates
        .into_iter()
        .filter(|update| rule_set.update_is_valid(update))
        .map(|update| {
            let index = update.len().div_euclid(2);
            update[index]
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let (rules, updates) = parse(input);
    let rule_set = RuleSet::from(rules.iter());

    updates
        .into_iter()
        .filter(|update| !rule_set.update_is_valid(update))
        .map(|update| rule_set.fixed_order(update))
        .map(|update| {
            let index = update.len().div_euclid(2);
            update[index]
        })
        .sum()
}

#[derive(Debug, Clone)]
struct RuleSet(HashMap<u32, HashSet<u32>>);
impl<'a, R: Iterator<Item = &'a Rule>> From<R> for RuleSet {
    fn from(rules: R) -> Self {
        let mut rule_set = HashMap::default();
        for (before, after) in rules {
            let entry: &mut HashSet<u32> = rule_set.entry(*before).or_default();
            entry.insert(*after);
        }

        RuleSet(rule_set)
    }
}

impl RuleSet {
    fn update_is_valid(&self, update: &ManualUpdate) -> bool {
        // iterate through update backwards, making sure that none of the preceding elements
        // are an issue
        for page_index in (0..update.len()).rev() {
            let page = update[page_index];
            let should_be_after = self.0.get(&page).cloned().unwrap_or_default();
            for preceding_page in &update[0..page_index] {
                if should_be_after.contains(preceding_page) {
                    return false;
                }
            }
        }

        true
    }

    fn fixed_order(&self, mut update: ManualUpdate) -> ManualUpdate {
        while !self.update_is_valid(&update) {
            // figure out which page to move (and where to put it)
            let mut page_index = update.len() - 1;
            loop {
                let page = update[page_index];
                let should_be_after = self.0.get(&page).cloned().unwrap_or_default();

                let mut changed = false;

                for preceding_page_index in 0..page_index {
                    if should_be_after.contains(&update[preceding_page_index]) {
                        let value = update.remove(page_index);
                        update.insert(preceding_page_index, value);
                        changed = true;
                        break;
                    }
                }

                if changed {
                    break;
                }

                page_index -= 1;
            }
        }

        update
    }
}

type Rule = (u32, u32);
type ManualUpdate = Vec<u32>;
fn parse(input: &str) -> (Vec<Rule>, Vec<ManualUpdate>) {
    let mut rules = Vec::default();
    let mut updates = Vec::default();

    for line in input.split('\n').filter(|l| !l.is_empty()) {
        if line.contains('|') {
            let mut elements = line.trim().split('|');
            let a = elements.next().unwrap().parse().unwrap();
            let b = elements.next().unwrap().parse().unwrap();
            rules.push((a, b));
        } else {
            let pages: ManualUpdate = line
                .trim()
                .split(',')
                .filter_map(|x| x.parse().ok())
                .collect();
            if !pages.is_empty() {
                updates.push(pages);
            }
        }
    }

    (rules, updates)
}

#[cfg(test)]
mod tests {
    use crate::{RuleSet, parse, part1, part2};

    const SAMPLE: &str = "\
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    ";

    #[test]
    fn test_parsing() {
        let (rules, updates) = parse(SAMPLE);
        assert_eq!(rules[0], (47, 53));
        assert_eq!(updates[0], vec![75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_cases_part1() {
        let (rules, updates) = parse(SAMPLE);
        let rule_set = RuleSet::from(rules.iter());

        assert!(rule_set.update_is_valid(&updates[0]));
        assert!(rule_set.update_is_valid(&updates[1]));
        assert!(rule_set.update_is_valid(&updates[2]));
        assert!(!rule_set.update_is_valid(&updates[3]));
        assert!(!rule_set.update_is_valid(&updates[4]));
        assert!(!rule_set.update_is_valid(&updates[5]));

        assert_eq!(part1(SAMPLE), 143);
    }

    #[test]
    fn test_cases_part2() {
        let (rules, updates) = parse(SAMPLE);
        let rule_set = RuleSet::from(rules.iter());

        assert_eq!(
            rule_set.fixed_order(updates[3].clone()),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(rule_set.fixed_order(updates[4].clone()), vec![61, 29, 13]);
        assert_eq!(
            rule_set.fixed_order(updates[5].clone()),
            vec![97, 75, 47, 29, 13]
        );

        assert_eq!(part2(SAMPLE), 123);
    }
}
