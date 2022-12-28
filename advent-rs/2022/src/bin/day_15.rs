/// day 15: beacon exclusion zone
use common::read_input_as_string;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    Sensor(i32), // contains nearest beacon distance
    Beacon,
    Empty,
}

type Location = (i32, i32);

fn distance((x1, y1): Location, (x2, y2): Location) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn parse_input(input: &str) -> HashMap<Location, State> {
    let mut map = HashMap::default();
    let re = Regex::new(
        "^Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)",
    )
    .unwrap();

    for line in input.split('\n') {
        if let Some(caps) = re.captures(line.trim()) {
            let sensor = (
                caps.get(1).unwrap().as_str().parse().unwrap(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
            );

            let beacon = (
                caps.get(3).unwrap().as_str().parse().unwrap(),
                caps.get(4).unwrap().as_str().parse().unwrap(),
            );

            map.insert(sensor, State::Sensor(distance(sensor, beacon)));
            map.insert(beacon, State::Beacon);
        }
    }

    map
}

fn mark_empty_if_unset(map: &mut HashMap<Location, State>, location: Location) {
    if !map.contains_key(&location) {
        map.insert(location, State::Empty);
    }
}

fn mark_empty_locations_in_row(map: &mut HashMap<Location, State>, row_number: i32) {
    let sensor_locations: Vec<_> = map
        .iter()
        .filter_map(|(location, state)| match state {
            State::Sensor(distance) => Some((location.clone(), distance.clone())),
            _ => None,
        })
        .collect();

    for ((sensor_x, sensor_y), beacon_distance) in sensor_locations.into_iter() {
        let delta_y = (sensor_y - row_number).abs();

        for total_distance in delta_y..=beacon_distance {
            let delta_x = total_distance - delta_y;
            mark_empty_if_unset(map, (sensor_x + delta_x, sensor_y + delta_y));
            mark_empty_if_unset(map, (sensor_x + delta_x, sensor_y - delta_y));
            mark_empty_if_unset(map, (sensor_x - delta_x, sensor_y + delta_y));
            mark_empty_if_unset(map, (sensor_x - delta_x, sensor_y - delta_y));
        }
    }
}

fn count_empty_in_row(map: &HashMap<Location, State>, row_number: i32) -> usize {
    map.iter()
        .filter(|((_, y), state)| y == &row_number && state == &&State::Empty)
        .count()
}

fn part1(input: &str, row_number: i32) -> usize {
    let mut map = parse_input(input);
    mark_empty_locations_in_row(&mut map, row_number);
    count_empty_in_row(&map, row_number)
}

fn find_distress_beacon_location(
    map: &HashMap<Location, State>,
    max_coord: i32,
) -> Option<Location> {
    let sensor_locations: Vec<_> = map
        .iter()
        .filter_map(|(location, state)| match state {
            State::Sensor(distance) => Some((location.clone(), distance.clone())),
            _ => None,
        })
        .collect();

    for y in 0..=max_coord {
        let mut x = 0;
        while x <= max_coord {
            let nearest_sensor = sensor_locations
                .iter()
                .filter_map(|(sensor, max_dist)| {
                    let dist = distance(*sensor, (x, y));
                    if dist <= *max_dist {
                        Some((*max_dist, dist, sensor))
                    } else {
                        None
                    }
                })
                .min_by_key(|s| s.1);

            if let Some((max_dist, dist, (sensor_x, _))) = nearest_sensor {
                // check whether this location is left or right of the closest sensor
                let x_delta = if x < *sensor_x {
                    // left: can move right 2*x_delta (to move to the right side) plus
                    // max_dist-dist (to get to the edge of the sensor's radius)
                    2 * (sensor_x - x) + (max_dist - dist)
                } else {
                    // right: can move to right edge of sensor radius
                    max_dist - dist
                };
                x += x_delta.max(1);
            } else {
                return Some((x, y));
            }
        }
    }

    None
}

fn part2(input: &str, max_coord: i32) -> u64 {
    let map = parse_input(input);
    if let Some((x, y)) = find_distress_beacon_location(&map, max_coord) {
        x as u64 * 4000000 + y as u64
    } else {
        0
    }
}

fn main() {
    let input = read_input_as_string(2022, 15).unwrap();
    let now = std::time::Instant::now();
    println!("{}", part1(&input, 2000000));
    println!("{}", part2(&input, 4000000));
    println!(
        "Computation took {}s",
        (std::time::Instant::now() - now).as_secs()
    );
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use indoc::indoc;
    const SAMPLE: &str = indoc! {"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3"
    };

    #[test]
    fn test() {
        assert_eq!(26, part1(SAMPLE, 10));
        assert_eq!(56000011, part2(SAMPLE, 20));
    }
}
