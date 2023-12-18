// Camel Cards

use std::collections::HashMap;

use common::read_input_as_string;

// NB: PartialOrd will be derived by the order of elements in the enum. hacky but works :shrug:
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<HashMap<char, u8>> for HandType {
    fn from(card_counts: HashMap<char, u8>) -> Self {
        match card_counts.values().max().copied().unwrap_or(0) {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if card_counts.values().any(|&v| v == 2) {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if card_counts.values().filter(|&&v| v == 2).count() == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    hand_type: HandType,
    cards: Vec<char>,
    bid: u32,
}

mod part1 {
    use super::HandType;
    use std::collections::HashMap;

    pub const CARD_ORDER: [char; 13] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    // assumes this provides 5 chars...
    pub fn hand_type_from_chars(cards: &[char]) -> HandType {
        let mut card_counts: HashMap<char, u8> = HashMap::default();
        for card in cards {
            let entry = card_counts.entry(*card).or_insert(0);
            *entry += 1;
        }

        // check for different hands
        HandType::from(card_counts)
    }
}

mod part2 {
    use super::HandType;
    use std::collections::HashMap;

    pub const CARD_ORDER: [char; 13] = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];

    // assumes this provides 5 chars...
    pub fn hand_type_from_chars(cards: &[char]) -> HandType {
        let mut card_counts: HashMap<char, u8> = HashMap::default();
        for card in cards {
            let entry = card_counts.entry(*card).or_insert(0);
            *entry += 1;
        }

        let jokers = card_counts.remove(&'J').unwrap_or(0);
        if jokers == 5 {
            return HandType::FiveOfAKind;
        }

        // add jokers to most common element
        let max_entry_card = *card_counts
            .iter()
            .max_by(|a, b| (*a).1.cmp((*b).1))
            .unwrap()
            .0;
        card_counts
            .entry(max_entry_card)
            .and_modify(|x| *x += jokers);

        HandType::from(card_counts)
    }
}

fn hand_from_str(parse_hand_type_fn: fn(&[char]) -> HandType, hand_str: &str) -> Option<Hand> {
    let elems: Vec<&str> = hand_str.split_whitespace().collect();
    let cards: Vec<char> = elems.get(0)?.to_string().chars().take(5).collect();
    let bid = elems.get(1)?.parse().ok()?;
    Some(Hand {
        hand_type: parse_hand_type_fn(&cards),
        cards,
        bid,
    })
}

fn partial_cmp_hand(card_order: &[char], h1: &Hand, h2: &Hand) -> std::cmp::Ordering {
    if h1.hand_type != h2.hand_type {
        return h1.hand_type.cmp(&h2.hand_type);
    }

    for (&c1, &c2) in h1.cards.iter().zip(h2.cards.iter()) {
        if c1 == c2 {
            continue;
        }

        // unwrapping bc we assume we're given valid cards
        let c1_idx = card_order.iter().position(|&c| c == c1).unwrap();
        let c2_idx = card_order.iter().position(|&c| c == c2).unwrap();
        return c1_idx.cmp(&c2_idx);
    }

    std::cmp::Ordering::Equal
}

fn total_winnings(
    input: &str,
    parse_hand_type_fn: fn(&[char]) -> HandType,
    card_order: &[char],
) -> u32 {
    let mut hands: Vec<Hand> = input
        .split('\n')
        .filter_map(|s| hand_from_str(parse_hand_type_fn, s))
        .collect();
    hands.sort_unstable_by(|h1, h2| partial_cmp_hand(card_order, h1, h2));

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

fn main() {
    let hands_str = read_input_as_string(2023, 7).unwrap();

    let winnings = total_winnings(&hands_str, part1::hand_type_from_chars, &part1::CARD_ORDER);
    println!("{winnings}");

    let winnings = total_winnings(&hands_str, part2::hand_type_from_chars, &part2::CARD_ORDER);
    println!("{winnings}");
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, total_winnings};
    const SAMPLE: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    #[test]
    fn test_part1() {
        let winnings = total_winnings(SAMPLE, part1::hand_type_from_chars, &part1::CARD_ORDER);
        assert_eq!(6440, winnings);
    }

    #[test]
    fn test_part2() {
        let winnings = total_winnings(SAMPLE, part2::hand_type_from_chars, &part2::CARD_ORDER);
        assert_eq!(5905, winnings);
    }
}
