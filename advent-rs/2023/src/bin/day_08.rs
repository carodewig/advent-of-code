// Haunted Wasteland

use common::read_input_as_string;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let map_str = read_input_as_string(2023, 8).unwrap();
    println!("{}", count_steps(&map_str));
    println!("{}", count_ghost_steps(&map_str));
}

#[derive(Debug, Copy, Clone, strum::EnumString, PartialEq, Eq)]
enum Direction {
    #[strum(serialize = "L")]
    Left,
    #[strum(serialize = "R")]
    Right,
}

type Directions = Vec<Direction>;
type Node = String;
type Map = HashMap<Node, (Node, Node)>;

fn parse(map_text: &str) -> (Directions, Map) {
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

    let direction_re = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();
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
    let mut node = "AAA".to_string();

    for (step, &direction) in directions.iter().cycle().enumerate() {
        if &node == "ZZZ" {
            return step;
        }

        node = next_node(&map, direction, &node);
    }

    unreachable!()
}

fn next_node(map: &Map, direction: Direction, current_node: &Node) -> Node {
    use Direction::Left;
    let (left, right) = map.get(current_node).unwrap().clone();
    if direction == Left {
        left
    } else {
        right
    }
}

fn find_cycle_length(map: &Map, directions: &Directions, start_node: &String) -> usize {
    let mut possible_cycle_starts: Vec<String> = Vec::default();
    let mut node = start_node.clone();

    for (steps, (direction_index, &direction)) in directions.iter().enumerate().cycle().enumerate()
    {
        // see if we're at the right place
        if direction_index == 0 && node.ends_with('Z') {
            // check what the next node and direction will be
            let next = next_node(map, direction, &node);
            if possible_cycle_starts.contains(&next) {
                return steps;
            }
        }

        if direction_index == directions.len() - 1 {
            possible_cycle_starts.push(node.clone());
        }

        // take a step
        node = next_node(map, direction, &node);
    }

    unreachable!()
}

fn count_ghost_steps(map_text: &str) -> usize {
    let (directions, map) = parse(map_text);
    map.keys()
        .filter(|key| key.ends_with('A'))
        .map(|starting_location| find_cycle_length(&map, &directions, starting_location))
        .fold(1, |acc, cycle_length| lcm(acc, cycle_length))
}

fn lcm(n1: usize, n2: usize) -> usize {
    let (mut x, mut y) = if n1 > n2 { (n1, n2) } else { (n2, n1) };

    let mut rem = x % y;
    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }

    n1 * n2 / y
}

#[cfg(test)]
mod tests {
    const SAMPLE1: &str = "\
        RL
        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

    const SAMPLE2: &str = "\
        LLR
        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

    const SAMPLE3: &str = "\
        LR
        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

    #[test]
    fn test_part1() {
        assert_eq!(2, super::count_steps(SAMPLE1));
        assert_eq!(6, super::count_steps(SAMPLE2));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::count_ghost_steps(SAMPLE3), 6);
    }
}
