// Haunted Wasteland

use common::read_input_as_string;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, strum::EnumString, PartialEq, Eq)]
enum Direction {
    #[strum(serialize = "L")]
    Left,
    #[strum(serialize = "R")]
    Right,
}

type Directions = Vec<Direction>;
type Node = String;

fn parse(map_text: &str) -> (Directions, HashMap<Node, (Node, Node)>) {
    let lines: Vec<_> = map_text
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    // first non-empty line is directions, remaining lines are nodes
    let directions = lines[0]
        .chars()
        .filter_map(|c| Direction::from_str(&c.to_string()).ok())
        .collect();

    let direction_re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    let mut map = HashMap::default();
    for line in &lines[1..] {
        if let Some(caps) = direction_re.captures(line) {
            map.insert(
                caps.get(1).unwrap().as_str().to_string(),
                (
                    caps.get(2).unwrap().as_str().to_string(),
                    caps.get(3).unwrap().as_str().to_string(),
                ),
            );
        }
    }

    (directions, map)
}

fn count_steps(map_text: &str) -> usize {
    let (directions, map) = parse(map_text);
    let mut steps = 0;
    let mut node = "AAA".to_string();

    for &direction in directions.iter().cycle() {
        if node.as_str() == "ZZZ" {
            break;
        }

        steps += 1;
        let (left, right) = map.get(&node).unwrap();
        node = if direction == Direction::Left {
            left.clone()
        } else {
            right.clone()
        }
    }

    steps
}

fn count_ghost_steps(map_text: &str) -> usize {
    let (directions, map) = parse(map_text);
    let mut steps = 0;

    // TODO: start from all elements ending in A, need to step until all elements end in Z
    //  keep track of steps for each element, checking to see when they start to cycle (ending on **Z)...
    //  hopefully cycle length is equal to integer multiple of length of directions? need to check this
    //  once something is cycling we can remove it from element list - keep track of cycle start idx and number of steps in it
    //  once we have cycles for everything we just need to figure out where they meet!
    todo!();

    steps
}

fn main() {
    let map_str = read_input_as_string(2023, 8).unwrap();
    println!("{}", count_steps(&map_str));
}

#[cfg(test)]
mod tests {
    const SAMPLE1: &str = r#"RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)"#;

    const SAMPLE2: &str = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test() {
        assert_eq!(2, super::count_steps(SAMPLE1));
        assert_eq!(6, super::count_steps(SAMPLE2));
    }
}
