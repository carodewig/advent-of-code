// Hoof It

use common::{read_input_as_string, Location};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_input_as_string(2024, 10).unwrap();
    println!("{}", TopographicMap::from(input.as_str()).part1());
    println!("{}", TopographicMap::from(input.as_str()).part2());
}

#[derive(Default, Debug, Clone)]
struct TopographicMap(HashMap<Location, u8>);
impl TopographicMap {
    fn elevation(&self, location: Location) -> Option<u8> {
        self.0.get(&location).copied()
    }
    fn trailheads(&self) -> impl Iterator<Item = Location> + '_ {
        self.0
            .iter()
            .filter(|&(_, elevation)| *elevation == 0)
            .map(|(location, _)| *location)
    }

    fn reachable_max_height_locations(&self, trailhead: Location) -> HashSet<Location> {
        let mut max_height_locations: HashSet<Location> = HashSet::default();
        let mut to_search: HashSet<Location> = HashSet::default();
        to_search.insert(trailhead);

        while let Some(location) = to_search.iter().next().copied() {
            to_search.remove(&location);
            if let Some(elevation) = self.elevation(location) {
                if elevation == 9 {
                    max_height_locations.insert(location);
                    continue;
                }

                for nearby_location in location.neighbors() {
                    if let Some(nearby_elevation) = self.elevation(nearby_location) {
                        if elevation + 1 == nearby_elevation {
                            to_search.insert(nearby_location);
                        }
                    }
                }
            }
        }

        max_height_locations
    }

    fn score(&self, trailhead: Location) -> usize {
        self.reachable_max_height_locations(trailhead).len()
    }

    fn rating(&self, trailhead: Location) -> usize {
        self.reachable_max_height_locations(trailhead)
            .into_iter()
            .map(|max_height_location| self.ways_to_reach(trailhead, max_height_location))
            .sum()
    }

    fn ways_to_reach(&self, from: Location, to: Location) -> usize {
        let elevation_from = self.elevation(from).unwrap();
        let elevation_to = self.elevation(to).unwrap();

        let ways = if elevation_from >= elevation_to {
            0
        } else if elevation_from + 1 == elevation_to {
            usize::from(from.neighbors().iter().contains(&to))
        } else {
            from.neighbors()
                .into_iter()
                .filter(|n| {
                    self.elevation(*n)
                        .map_or(false, |elevation_n| elevation_from + 1 == elevation_n)
                })
                .map(|n| self.ways_to_reach(n, to))
                .sum()
        };

        ways
    }

    fn part1(&self) -> usize {
        self.trailheads()
            .map(|trailhead| self.score(trailhead))
            .sum()
    }

    fn part2(&self) -> usize {
        self.trailheads()
            .map(|trailhead| self.rating(trailhead))
            .sum()
    }
}

impl From<&str> for TopographicMap {
    fn from(input: &str) -> Self {
        let mut topographic_map = HashMap::default();
        for (row, line) in input.trim().split_whitespace().enumerate() {
            for (column, char) in line.chars().enumerate() {
                let location = Location::new(row as isize, column as isize);
                let elevation = char.to_digit(10).unwrap() as u8;
                topographic_map.insert(location, elevation);
            }
        }

        Self(topographic_map)
    }
}

#[cfg(test)]
mod tests {
    use crate::TopographicMap;

    const SAMPLE1: &str = "
        0123
        1234
        8765
        9876";

    const SAMPLE2: &str = "
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732";

    #[test]
    fn test_part1() {
        assert_eq!(TopographicMap::from(SAMPLE1).part1(), 1);
        assert_eq!(TopographicMap::from(SAMPLE2).part1(), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(TopographicMap::from(SAMPLE2).part2(), 81);
    }
}
