// day 1: no time for a taxicab

use advent_rs_2016::read_input::read_file;
use std::collections::HashSet;

#[derive(Eq, Hash, Clone, Debug, PartialEq)]
struct Location(i32, i32);
impl Location {
    fn step(&self, direction: i32, num_steps: i32) -> Location {
        let direction_rad = (direction as f32).to_radians();
        let (x_step, y_step) = (
            direction_rad.sin().round() as i32,
            direction_rad.cos().round() as i32,
        );

        Location(self.0 + num_steps * x_step, self.1 + num_steps * y_step)
    }

    fn distance_from_origin(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

// this would've been nicer with generators in python but :shrug:
fn follow_step_string<S: AsRef<str>>(step_string: S) -> Vec<Location> {
    let mut locations: Vec<Location> = Vec::from([Location(0, 0)]);
    let mut direction: i32 = 0;

    for instruction in step_string.as_ref().split(", ") {
        let (direction_str, num_steps) = instruction.split_at(1);
        direction = match direction_str {
            "L" => (direction - 90) % 360,
            "R" => (direction + 90) % 360,
            _ => direction,
        };

        for _ in 0..num_steps.parse::<i32>().unwrap() {
            locations.push(locations.last().unwrap().step(direction, 1));
        }
    }

    locations
}

fn distance_from_origin<S: AsRef<str>>(step_string: S) -> i32 {
    let visited_locations = follow_step_string(step_string);
    visited_locations.last().unwrap().distance_from_origin()
}

fn distance_from_first_location_revisited<S: AsRef<str>>(step_string: S) -> Option<i32> {
    let mut visited_locations: HashSet<Location> = HashSet::new();
    for location in follow_step_string(step_string) {
        if !visited_locations.insert(location.clone()) {
            return Some(location.distance_from_origin());
        }
    }

    None
}

fn main() {
    let step_string = read_file("01.txt");
    println!("{:?}", distance_from_origin(&step_string));
    println!("{:?}", distance_from_first_location_revisited(&step_string));
}

#[cfg(test)]
mod tests {
    use super::Location;
    use super::{distance_from_first_location_revisited, distance_from_origin};

    #[test]
    fn step_directions_are_consistent() {
        let location = Location(0, 0);

        assert_eq!(Location(0, 1), location.step(0, 1));
        assert_eq!(Location(1, 1), location.step(0, 1).step(90, 1));
        assert_eq!(Location(0, -1), location.step(180, 1));
        assert_eq!(Location(-2, 1), location.step(0, 1).step(270, 2));
    }

    #[test]
    fn test_distance_from_origin() {
        assert_eq!(5, distance_from_origin("R2, L3"));
        assert_eq!(2, distance_from_origin("R2, R2, R2"));
        assert_eq!(12, distance_from_origin("R5, L5, R5, R3"));
    }

    #[test]
    fn test_distance_from_first_location_revisited() {
        assert_eq!(
            Some(4),
            distance_from_first_location_revisited("R8, R4, R4, R8")
        );
    }
}
