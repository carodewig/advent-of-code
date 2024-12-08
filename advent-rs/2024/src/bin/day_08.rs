// Resonant Collinearity

use common::read_input_as_string;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Mul, Sub};

fn main() {
    println!("{}", part1(&read_input_as_string(2024, 8).unwrap()));
    println!("{}", part2(&read_input_as_string(2024, 8).unwrap()));
}

fn part1(input: &str) -> usize {
    Map::from(input).count_unique_antinodes(true)
}

fn part2(input: &str) -> usize {
    Map::from(input).count_unique_antinodes(false)
}

// NB: `isize`s make arithmetic easier (ie you can go off the map into the negatives)
//  IRL it would be better to handle `usize` math properly
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Location((isize, isize));
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Step((isize, isize));

impl Add<Step> for Location {
    type Output = Location;
    fn add(self, rhs: Step) -> Self::Output {
        Self((self.0 .0 + rhs.0 .0, self.0 .1 + rhs.0 .1))
    }
}

impl Sub<Location> for Location {
    type Output = Step;
    fn sub(self, rhs: Location) -> Self::Output {
        Step((self.0 .0 - rhs.0 .0, self.0 .1 - rhs.0 .1))
    }
}

impl Mul<isize> for Step {
    type Output = Step;
    fn mul(self, rhs: isize) -> Self::Output {
        Self((self.0 .0 * rhs, self.0 .1 * rhs))
    }
}

struct Map {
    rows: isize,
    columns: isize,
    antennae: HashMap<char, Vec<Location>>,
}

impl Map {
    fn within_map_bounds(&self, location: Location) -> bool {
        let (row, column) = location.0;
        (0..self.rows).contains(&row) && (0..self.columns).contains(&column)
    }

    fn extract_antinodes_at_frequency(
        &self,
        frequency: char,
        check_distance: bool,
    ) -> HashSet<Location> {
        let Some(antennae) = self.antennae.get(&frequency) else {
            return HashSet::default();
        };

        let mut antinodes = HashSet::default();

        // need to grab all possible pairs of antennae
        for antenna_pair in antennae.iter().combinations(2) {
            let [&antenna1, &antenna2] = antenna_pair[..2] else {
                return HashSet::default();
            };

            let step = antenna1 - antenna2;

            for factor in [1, -1] {
                let mut antinode_location = antenna1;
                while self.within_map_bounds(antinode_location) {
                    let dist1 = antenna1 - antinode_location;
                    let dist2 = antenna2 - antinode_location;
                    if !check_distance || dist1 == dist2 * 2 || dist1 * 2 == dist2 {
                        antinodes.insert(antinode_location);
                    }

                    antinode_location = antinode_location + step * factor;
                }
            }
        }

        antinodes
    }

    fn count_unique_antinodes(&self, check_distance: bool) -> usize {
        let frequencies = self.antennae.keys().copied();
        frequencies
            .map(|frequency| self.extract_antinodes_at_frequency(frequency, check_distance))
            .reduce(|acc, antinodes| acc.union(&antinodes).copied().collect())
            .unwrap_or_default()
            .len()
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let lines: Vec<&str> = input.split_whitespace().filter(|l| !l.is_empty()).collect();

        let rows = lines.len() as isize;
        let columns = lines[0].len() as isize;
        let mut antennae = HashMap::default();
        for (row_index, line) in lines.into_iter().enumerate() {
            for (column_index, character) in line.chars().enumerate() {
                if character != '.' {
                    let entry: &mut Vec<Location> = antennae.entry(character).or_default();
                    entry.push(Location((row_index as isize, column_index as isize)));
                }
            }
        }

        Self {
            rows,
            columns,
            antennae,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const SAMPLE: &str = "
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............";

    #[test]
    fn test_solution() {
        assert_eq!(part1(SAMPLE), 14);
        assert_eq!(part2(SAMPLE), 34);
    }
}
