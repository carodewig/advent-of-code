/// day 17: pyroclastic flow
use common::read_input_as_string;
use std::collections::HashSet;

type Location = (u64, u64);

#[derive(Clone, Debug)]
struct Rock(HashSet<Location>);
impl Rock {
    fn new(rock_type: char, highest_rock: u64) -> Self {
        match rock_type {
            '-' => Self(HashSet::from_iter([
                (2, highest_rock + 3),
                (3, highest_rock + 3),
                (4, highest_rock + 3),
                (5, highest_rock + 3),
            ])),
            '+' => Self(HashSet::from_iter([
                (3, highest_rock + 3),
                (2, highest_rock + 4),
                (3, highest_rock + 4),
                (4, highest_rock + 4),
                (3, highest_rock + 5),
            ])),
            'j' => Self(HashSet::from_iter([
                (2, highest_rock + 3),
                (3, highest_rock + 3),
                (4, highest_rock + 3),
                (4, highest_rock + 4),
                (4, highest_rock + 5),
            ])),
            'l' => Self(HashSet::from_iter([
                (2, highest_rock + 3),
                (2, highest_rock + 4),
                (2, highest_rock + 5),
                (2, highest_rock + 6),
            ])),
            'o' => Self(HashSet::from_iter([
                (2, highest_rock + 3),
                (2, highest_rock + 4),
                (3, highest_rock + 3),
                (3, highest_rock + 4),
            ])),
            _ => panic!("unexpected rock type"),
        }
    }

    // returns none if it tries to go through the wall
    fn pushed(&self, direction: char) -> Option<Self> {
        let new_locations: HashSet<_> = self
            .0
            .iter()
            .filter_map(|(x, y)| match direction {
                '<' => x.checked_sub(1).map(|new_x| (new_x, *y)),
                '>' => {
                    if x + 1 < 7 {
                        Some((x + 1, *y))
                    } else {
                        None
                    }
                }
                _ => panic!("unexpected direction"),
            })
            .collect();

        if new_locations.len() == self.0.len() {
            Some(Self(new_locations))
        } else {
            None
        }
    }

    // returns none if it tries to go through the bottom
    fn dropped(&self) -> Option<Self> {
        let new_locations: HashSet<_> = self
            .0
            .iter()
            .filter_map(|(x, y)| y.checked_sub(1).map(|new_y| (*x, new_y)))
            .collect();
        if new_locations.len() == self.0.len() {
            Some(Self(new_locations))
        } else {
            None
        }
    }

    fn hit_bottom(&self) -> bool {
        self.0.iter().any(|(_, y)| y == &0)
    }
}

fn highest_rock(rock_locations: &HashSet<Location>) -> u64 {
    rock_locations.iter().map(|(_, y)| y + 1).max().unwrap_or(0)
}

fn drop_rocks_part1(jet_pattern: &str, number_of_rocks: u64) -> u64 {
    let mut jet_iter = jet_pattern.trim().chars().cycle();
    let mut rock_iter = "-+jlo".chars().cycle();
    let mut fallen_rock_locations: HashSet<Location> = HashSet::default();

    for index in 1..=number_of_rocks {
        let highest_rock = highest_rock(&fallen_rock_locations);

        // to prevent `fallen_rock_locations` from growing out of control, drop rocks that are
        // 1000 or more levels deeper than the highest rock
        fallen_rock_locations.retain(|(_, y)| y + 1000 >= highest_rock);

        let mut rock = Rock::new(rock_iter.next().unwrap(), highest_rock);
        println!("Dropping {}th rock: {:?}", index, rock);
        loop {
            if let Some(pushed_rock) = rock.pushed(jet_iter.next().unwrap()) {
                if fallen_rock_locations.is_disjoint(&pushed_rock.0) {
                    rock = pushed_rock;
                }
            }

            if let Some(fallen_rock) = rock.dropped() {
                if fallen_rock_locations.is_disjoint(&fallen_rock.0) {
                    rock = fallen_rock;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        for location in rock.0.drain() {
            fallen_rock_locations.insert(location);
        }
    }

    highest_rock(&fallen_rock_locations)
}

fn main() {
    let jets = read_input_as_string(2022, 17).unwrap();
    println!("{}", drop_rocks_part1(&jets, 2022));
}

#[cfg(test)]
mod tests {
    use crate::drop_rocks_part1;

    const SAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test() {
        assert_eq!(3068, drop_rocks_part1(SAMPLE, 2022));
        assert_eq!(1514285714288, drop_rocks_part1(SAMPLE, 1000000000000));
    }
}
