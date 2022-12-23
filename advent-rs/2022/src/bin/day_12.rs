/// day 12: hill climbing algorithm
use common::read_input_as_string;
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    f32::consts::E,
};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Location {
    loc: (u32, u32),
    elevation: u32,
    steps: usize,
}
impl Location {
    fn new(loc: (u32, u32), elevation: u32, steps: usize) -> Self {
        Self {
            loc,
            elevation,
            steps,
        }
    }
}
impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.steps.cmp(&other.steps))
    }
}
impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps)
    }
}

fn parse_elevation(c: char) -> u32 {
    c.to_digit(36).expect("Could not convert char to digit")
}

fn parse_map(map_str: &str) -> Option<(HashMap<(u32, u32), u32>, (u32, u32), (u32, u32))> {
    let mut map = HashMap::default();
    let mut start_location = None;
    let mut end_location = None;
    for (y, line) in map_str.split('\n').enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let location = (x as u32, y as u32);
            let elev_char = match c {
                'S' => {
                    start_location = Some(location);
                    'a'
                }
                'E' => {
                    end_location = Some(location);
                    'z'
                }
                c => c,
            };
            map.insert(location, parse_elevation(elev_char));
        }
    }

    Some((map, start_location?, end_location?))
}

fn nearby_locations(location: &(u32, u32)) -> impl Iterator<Item = (u32, u32)> + '_ {
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    directions.into_iter().filter_map(|(del_x, del_y)| {
        if let (Some(x), Some(y)) = (
            location.0.checked_add_signed(del_x),
            location.1.checked_add_signed(del_y),
        ) {
            Some((x, y))
        } else {
            None
        }
    })
}

fn minimum_steps_bfs<F: Fn(&Location) -> bool>(
    elevations: HashMap<(u32, u32), u32>,
    start_location: (u32, u32),
    can_step: fn(u32, u32) -> bool,
    reached_target: F,
) -> Option<usize> {
    // reverse items to make a min heap
    let mut unvisited = BinaryHeap::default();
    unvisited.push(Reverse(Location::new(
        start_location,
        *elevations.get(&start_location).unwrap(),
        0,
    )));

    let mut visited: HashMap<(u32, u32), usize> = HashMap::default();

    while let Some(location) = unvisited.pop() {
        let location = location.0;
        if visited.contains_key(&location.loc) {
            continue;
        }

        if reached_target(&location) {
            return Some(location.steps);
        }

        visited.insert(location.loc, location.steps);

        for nearby in nearby_locations(&location.loc) {
            if visited.contains_key(&nearby) {
                continue;
            }

            if let Some(elev) = elevations.get(&nearby) {
                if can_step(location.elevation, *elev) {
                    unvisited.push(Reverse(Location::new(nearby, *elev, location.steps + 1)));
                }
            }
        }
    }

    None
}

fn part1(input: &str) -> Option<usize> {
    let (elevations, start_location, end_location) =
        parse_map(input).expect("Could not parse input");

    let can_step = |elev1: u32, elev2: u32| elev1 + 1 >= elev2;
    let reached_target = move |location: &Location| &location.loc == &end_location;
    minimum_steps_bfs(elevations, start_location, can_step, reached_target)
}
fn part2(input: &str) -> Option<usize> {
    let (elevations, _start_location, end_location) =
        parse_map(input).expect("Could not parse input");

    // stepping backward, so reverse can_step args
    let can_step = |elev1: u32, elev2: u32| elev2 + 1 >= elev1;
    let reached_target = move |location: &Location| location.elevation == parse_elevation('a');
    minimum_steps_bfs(elevations, end_location, can_step, reached_target)
}

fn main() {
    let input = read_input_as_string(2022, 12).unwrap();
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi"};

    #[test]
    fn test() {
        assert_eq!(Some(31), part1(SAMPLE));
        assert_eq!(Some(29), part2(SAMPLE));
    }
}
