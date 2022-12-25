/// day 13: distress signal
use common::read_input_as_string;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Data {
    List(Vec<Data>),
    Int(u32),
}

impl Data {
    fn compare(left: &Data, right: &Data) -> Option<bool> {
        match (left, right) {
            (Data::List(left), Data::List(right)) => {
                let mut index = 0;
                loop {
                    match (left.get(index), right.get(index)) {
                        (Some(left_elem), Some(right_elem)) => {
                            if let Some(result) = Self::compare(left_elem, right_elem) {
                                return Some(result);
                            }
                        }
                        (None, None) => return None,
                        (Some(_), None) => return Some(false),
                        (None, Some(_)) => return Some(true),
                    }

                    index += 1;
                }
            }
            (Data::Int(left), Data::Int(right)) => {
                if left == right {
                    None
                } else {
                    Some(left < right)
                }
            }
            (Data::Int(left), Data::List(right)) => Self::compare(
                &Data::List(vec![Data::Int(*left)]),
                &Data::List(right.clone()),
            ),
            (Data::List(left), Data::Int(right)) => Self::compare(
                &Data::List(left.clone()),
                &Data::List(vec![Data::Int(*right)]),
            ),
        }
    }

    fn try_from_str(input: &str) -> Option<Self> {
        #[derive(PartialEq, Eq)]
        enum Element {
            Data(Data),
            OpenParen,
        }

        // assumes all strs will have list at the top level
        let mut stack = Vec::default();
        let mut substr = String::default();
        for character in input.trim().chars() {
            match character {
                ',' => {
                    // push int(substr) if that's what it is
                    if !substr.is_empty() {
                        let value: u32 = substr.parse().unwrap();
                        stack.push(Element::Data(Data::Int(value)));
                        substr = String::default();
                    }
                }
                '[' => {
                    stack.push(Element::OpenParen);
                }
                ']' => {
                    // push int(substr) if that's what it is
                    if !substr.is_empty() {
                        let value: u32 = substr.parse().unwrap();
                        stack.push(Element::Data(Data::Int(value)));
                        substr = String::default();
                    }

                    let mut subvec = Vec::default();
                    while let Some(element) = stack.pop() {
                        match element {
                            Element::OpenParen => break,
                            Element::Data(data) => subvec.push(data),
                        }
                    }
                    subvec.reverse();
                    stack.push(Element::Data(Data::List(subvec)));
                }
                // all others should be numeric
                _ => substr.push(character),
            }
        }

        assert_eq!(stack.len(), 1);

        stack.pop().and_then(|element| match element {
            Element::OpenParen => None,
            Element::Data(data) => Some(data),
        })
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match Data::compare(self, other) {
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
            None => Ordering::Equal,
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> u32 {
    let lines = input.split('\n').filter(|line| !line.trim().is_empty());
    let mut index = 1;
    let mut total = 0;

    for mut line_pair in &lines.chunks(2) {
        let left = Data::try_from_str(line_pair.next().unwrap().trim()).unwrap();
        let right = Data::try_from_str(line_pair.next().unwrap().trim()).unwrap();

        if left < right {
            total += index;
        }

        index += 1;
    }

    total
}

fn part2(input: &str) -> usize {
    let div2 = Data::List(vec![Data::List(vec![Data::Int(2)])]);
    let div6 = Data::List(vec![Data::List(vec![Data::Int(6)])]);
    let mut data: Vec<Data> = input
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .filter_map(Data::try_from_str)
        .collect();
    data.push(div2.clone());
    data.push(div6.clone());
    data.sort();

    let index2 = data.binary_search(&div2).unwrap() + 1;
    let index6 = data.binary_search(&div6).unwrap() + 1;
    index2 * index6
}

fn main() {
    let input = read_input_as_string(2022, 13).unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    const SAMPLE: &str = "\
[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n\
[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n\
[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test() {
        assert_eq!(13, part1(SAMPLE));
        assert_eq!(140, part2(SAMPLE));
    }
}
