/// day 14: regolith reservoir
use common::read_input_as_string;
use std::{collections::HashSet, iter::repeat};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Location {
    x: i32,
    y: i32,
}
impl Location {
    fn try_from_str(location_str: &str) -> Option<Location> {
        let mut coords = location_str.split(',');
        let x = coords.next()?.parse().ok()?;
        let y = coords.next()?.parse().ok()?;
        Some(Location { x, y })
    }

    fn from_deltas(&self, x_delta: i32, y_delta: i32) -> Self {
        Location {
            x: self.x + x_delta,
            y: self.y + y_delta,
        }
    }

    fn between_locations(loc1: &Location, loc2: &Location) -> Vec<Location> {
        let mut locations = Vec::default();
        match (loc1.x == loc2.x, loc1.y == loc2.y) {
            (true, true) => {
                locations.push(*loc1);
            }
            (false, false) => panic!("Diagonal formation is not supported"),
            (true, false) => {
                let start_y = loc1.y.min(loc2.y);
                let end_y = loc1.y.max(loc2.y);
                for y in start_y..=end_y {
                    locations.push(Location { x: loc1.x, y });
                }
            }
            (false, true) => {
                let start_x = loc1.x.min(loc2.x);
                let end_x = loc1.x.max(loc2.x);
                for x in start_x..=end_x {
                    locations.push(Location { x, y: loc1.y });
                }
            }
        }
        locations
    }
}

#[derive(Debug, Clone, Default)]
struct RockMap(HashSet<Location>);
impl RockMap {
    fn add_rock_formations(&mut self, input: &str) {
        for line in input.split('\n') {
            let locations: Vec<Location> = line
                .trim()
                .split(" -> ")
                .filter_map(Location::try_from_str)
                .collect();

            for (start, end) in locations.iter().zip(locations.iter().skip(1)) {
                for location in Location::between_locations(start, end) {
                    self.0.insert(location);
                }
            }
        }
    }

    fn lowest_y(&self) -> i32 {
        self.0.iter().map(|l| l.y).max().unwrap_or(0)
    }

    // returns true if sand came to rest
    fn drop_sand_part1(&mut self, lowest_rock_y: i32) -> bool {
        let mut sand_location = Location { x: 500, y: 0 };
        while sand_location.y <= lowest_rock_y {
            let mut possible_locations = vec![
                sand_location.from_deltas(0, 1),
                sand_location.from_deltas(-1, 1),
                sand_location.from_deltas(1, 1),
            ]
            .into_iter()
            .filter(|loc| !self.0.contains(loc));

            if let Some(new_sand_location) = possible_locations.next() {
                sand_location = new_sand_location;
            } else {
                break;
            }
        }

        self.0.insert(sand_location);
        sand_location.y <= lowest_rock_y
    }

    // returns true if we were able to drop the sand
    fn drop_sand_part2(&mut self, lowest_rock_y: i32) -> bool {
        let mut sand_location = Location { x: 500, y: 0 };
        if self.0.contains(&sand_location) {
            return false;
        }

        while sand_location.y < lowest_rock_y + 1 {
            let mut possible_locations = vec![
                sand_location.from_deltas(0, 1),
                sand_location.from_deltas(-1, 1),
                sand_location.from_deltas(1, 1),
            ]
            .into_iter()
            .filter(|loc| !self.0.contains(loc));

            if let Some(new_sand_location) = possible_locations.next() {
                sand_location = new_sand_location;
            } else {
                break;
            }
        }

        self.0.insert(sand_location);
        true
    }

    fn simulate(&mut self, drop_sand: fn(&mut Self, i32) -> bool) -> usize {
        let lowest_rock_y = self.lowest_y();
        repeat(())
            .take_while(|_| drop_sand(self, lowest_rock_y))
            .count()
    }
}

impl From<&str> for RockMap {
    fn from(value: &str) -> Self {
        let mut rock_map = Self::default();
        rock_map.add_rock_formations(value);
        rock_map
    }
}

fn main() {
    let input = read_input_as_string(2022, 14).unwrap();

    println!(
        "{}",
        RockMap::from(input.as_str()).simulate(RockMap::drop_sand_part1)
    );

    println!(
        "{}",
        RockMap::from(input.as_str()).simulate(RockMap::drop_sand_part2)
    );
}

#[cfg(test)]
mod test {
    use crate::RockMap;
    const SAMPLE: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test() {
        assert_eq!(24, RockMap::from(SAMPLE).simulate(RockMap::drop_sand_part1));
        assert_eq!(93, RockMap::from(SAMPLE).simulate(RockMap::drop_sand_part2));
    }
}
