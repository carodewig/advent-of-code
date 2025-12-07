use common::read_input_as_string;
use std::collections::{HashMap, HashSet};

struct Map {
    splitters: Vec<HashSet<usize>>,
    start: usize,
}

impl<S: AsRef<str>> From<S> for Map {
    fn from(value: S) -> Self {
        let mut splitters = Vec::new();
        let mut start = 0;

        for line in value.as_ref().split('\n') {
            let mut row = HashSet::new();
            for (index, c) in line.trim().chars().enumerate() {
                match c {
                    'S' => start = index,
                    '^' => {
                        row.insert(index);
                    }
                    _ => {}
                }
            }
            splitters.push(row);
        }

        Self { splitters, start }
    }
}

impl Map {
    fn part1(&self) -> usize {
        let mut splits = 0;
        let mut beam_locations = HashSet::from([self.start]);
        for splitter_locations in &self.splitters {
            let mut new_beam_locations = HashSet::default();
            for beam_location in beam_locations {
                if splitter_locations.contains(&beam_location) {
                    splits += 1;
                    new_beam_locations.insert(beam_location - 1);
                    new_beam_locations.insert(beam_location + 1);
                } else {
                    new_beam_locations.insert(beam_location);
                }
            }

            beam_locations = new_beam_locations;
        }

        splits
    }
}

struct QuantumMap {
    splitters: Vec<HashSet<usize>>,
    start: (usize, usize),
    num_timelines: HashMap<(usize, usize), usize>,
}

impl From<Map> for QuantumMap {
    fn from(map: Map) -> Self {
        Self {
            splitters: map.splitters,
            start: (0, map.start),
            num_timelines: HashMap::new(),
        }
    }
}

impl QuantumMap {
    fn num_timelines(&mut self, (row, column): (usize, usize)) -> usize {
        if let Some(timelines) = self.num_timelines.get(&(row, column)) {
            return *timelines;
        }

        let Some(splitters_row) = self.splitters.get(row) else {
            // reached the end of the map
            return 1;
        };

        let value = if splitters_row.contains(&column) {
            // split this beam into two timelines
            self.num_timelines((row, column - 1)) + self.num_timelines((row, column + 1))
        } else {
            self.num_timelines((row + 1, column))
        };

        self.num_timelines.insert((row, column), value);
        value
    }

    fn part2(mut self) -> usize {
        self.num_timelines(self.start)
    }
}

fn main() {
    let map = Map::from(&read_input_as_string(2025, 7).unwrap());
    assert_eq!(map.part1(), 1622);

    let map = QuantumMap::from(map);
    assert_eq!(map.part2(), 10_357_305_916_520);
}

#[cfg(test)]
mod tests {
    use super::{Map, QuantumMap};

    const SAMPLE: &str = "\
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............";

    #[test]
    fn test_part1() {
        let map = Map::from(SAMPLE);
        assert_eq!(map.part1(), 21);
    }

    #[test]
    fn test_part2() {
        let map = QuantumMap::from(Map::from(SAMPLE));
        assert_eq!(map.part2(), 40);
    }
}
