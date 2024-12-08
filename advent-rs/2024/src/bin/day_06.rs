use common::read_input_as_string;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", part1(&read_input_as_string(2024, 6).unwrap()));
    println!("{}", part2(&read_input_as_string(2024, 6).unwrap()));
}

fn part1(input: &str) -> usize {
    let (map, mut guard) = parse(input);
    let mut visited: HashSet<Location> = HashSet::default();
    loop {
        // if the guard is outside the map, stop
        if !map.0.contains_key(&guard.location) {
            break;
        }

        visited.insert(guard.location);

        let mut next_location = guard.next_location();
        while map.0.get(&next_location).copied().unwrap_or(false) {
            guard.turn_right();
            next_location = guard.next_location();
        }

        guard.step();
    }

    visited.len()
}

fn part2(input: &str) -> usize {
    let (map, guard) = parse(input);
    let mut loops = 0;

    let total = map.0.iter().filter(|(_, v)| !*v).count();

    for (num, (location, _)) in map.0.iter().filter(|(_, v)| !*v).enumerate() {
        println!("{num} / {total}");
        if location == &guard.location {
            continue;
        }

        let mut modified_map = map.clone();
        modified_map.0.insert(*location, true);
        if stuck_in_a_loop(modified_map, guard) {
            loops += 1;
        }
    }

    loops
}

fn stuck_in_a_loop(map: Map, mut guard: Guard) -> bool {
    // need guard to end up in the same location and direction to ensure a loop
    let mut visited: HashSet<(Location, Direction)> = HashSet::default();

    loop {
        // if the guard is outside the map, stop
        if !map.0.contains_key(&guard.location) {
            return false;
        }

        let key = (guard.location, guard.direction);

        // if this location and direction has been seen before, stop
        if visited.contains(&key) {
            return true;
        }
        visited.insert(key);

        let mut next_location = guard.next_location();
        while map.0.get(&next_location).copied().unwrap_or(false) {
            guard.turn_right();
            next_location = guard.next_location();
        }

        guard.step();
    }
}

// NB: (row, column)
type Location = (i32, i32);
type Direction = (i32, i32);

// true == obstacle
#[derive(Debug, Clone)]
struct Map(HashMap<Location, bool>);

#[derive(Debug, Copy, Clone)]
struct Guard {
    location: Location,
    direction: Direction,
}

impl Guard {
    fn turn_right(&mut self) {
        let new_direction = match self.direction {
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            _ => unreachable!(),
        };
        self.direction = new_direction;
    }

    fn next_location(&self) -> Location {
        (
            self.location.0 + self.direction.0,
            self.location.1 + self.direction.1,
        )
    }

    fn step(&mut self) {
        self.location = self.next_location();
    }
}

fn parse(input: &str) -> (Map, Guard) {
    let mut locations = HashMap::default();
    let mut guard = None;

    for (row, line) in input
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .enumerate()
    {
        for (column, char) in line.trim().chars().enumerate() {
            let location = (row as i32, column as i32);

            // check for guard
            if char == '^' {
                guard = Some(Guard {
                    location,
                    direction: (-1, 0),
                });
            }

            let has_obstacle = char == '#';
            locations.insert(location, has_obstacle);
        }
    }
    (Map(locations), guard.unwrap())
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    const SAMPLE: &str = "\
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...";

    #[test]
    fn test_parse() {
        let (map, guard) = parse(SAMPLE);
        assert_eq!(guard.location, (6, 4));
        assert_eq!(guard.direction, (-1, 0));

        assert!(map.0.get(&(0, 4)).unwrap());
        assert!(map.0.get(&(3, 2)).unwrap());
        assert!(!map.0.get(&(0, 5)).unwrap());
        assert!(!map.0.get(&(6, 4)).unwrap());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(".#..\n..#.\n.^.."), 2);
        assert_eq!(part1(SAMPLE), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 6);
    }
}

// .#..
// ..#.
// .^..
