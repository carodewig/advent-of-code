use common::{read_input_as_string, Location};
use std::collections::HashSet;

#[derive(Clone)]
struct Map {
    paper_locations: HashSet<Location>,
}

impl<S: AsRef<str>> From<S> for Map {
    fn from(value: S) -> Self {
        let mut map = Map {
            paper_locations: HashSet::new(),
        };
        for (line, y) in value
            .as_ref()
            .split('\n')
            .filter(|l| !l.is_empty())
            .zip(0..)
        {
            for (char, x) in line.trim().chars().zip(0..) {
                if char == '@' {
                    map.paper_locations.insert(Location::new(x, y));
                }
            }
        }
        map
    }
}

impl Map {
    fn adjacent_rolls_of_paper(&self, location: &Location) -> usize {
        location
            .principal_neighbors()
            .into_iter()
            .filter(|nearby| self.paper_locations.contains(nearby))
            .count()
    }

    fn can_remove(&self, location: &Location) -> bool {
        self.adjacent_rolls_of_paper(location) < 4
    }
}

fn part1(map: &Map) -> usize {
    map.paper_locations
        .iter()
        .filter(|location| map.can_remove(location))
        .count()
}

fn part2(mut map: Map) -> usize {
    let mut removed_rolls = 0;
    loop {
        let to_remove: Vec<_> = map
            .paper_locations
            .iter()
            .filter(|location| map.can_remove(location))
            .copied()
            .collect();

        if to_remove.is_empty() {
            return removed_rolls;
        }

        for location in to_remove {
            removed_rolls += 1;
            map.paper_locations.remove(&location);
        }
    }
}

fn main() {
    let map = Map::from(&read_input_as_string(2025, 4).unwrap());
    assert_eq!(part1(&map), 1460);
    assert_eq!(part2(map), 9243);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, Map};
    const SAMPLE: &str = "\
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.";

    #[test]
    fn test_part1() {
        let map = Map::from(SAMPLE);
        assert_eq!(part1(&map), 13);
    }

    #[test]
    fn test_part2() {
        let map = Map::from(SAMPLE);
        assert_eq!(part2(map), 43);
    }
}
