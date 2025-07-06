use common::read_input_as_string;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", part1(&read_input_as_string(2023, 10).unwrap()));
    println!("{}", part2(&read_input_as_string(2023, 10).unwrap()));
}

fn part1(input: &str) -> usize {
    let (pipes, squirrel) = parse(input);

    let main_loop = find_loop(&pipes, squirrel);
    main_loop.len() / 2
}

fn part2(input: &str) -> usize {
    let rows = num_rows(input) as isize;
    let columns = num_columns(input) as isize;

    let (pipes, squirrel) = parse(input);
    let main_loop = find_loop(&pipes, squirrel);

    // walk the loop clockwise -- all interior locations will be on the right
    // ... but which way is clockwise
    // actually, walk the loop both ways, keeping track of locations on the right (including borders ie -1)
    // whichever has all the -1s is the exterior set, the other will be the interior set
    // BFS out from the interior set locations to find all locations not touching the loop
    todo!()
}

fn find_loop(pipes: &Pipes, starting_location: Location) -> HashSet<Location> {
    let mut loop_pipes = HashSet::default();
    loop_pipes.insert(starting_location);

    let starting_pipe = pipes.get(&starting_location).unwrap();
    let mut previous = starting_location;
    let mut current = starting_pipe.0.iter().next().unwrap().clone();

    let find_next = |current: &Location, previous: &Location| {
        pipes
            .get(current)?
            .0
            .iter()
            .find(|p| p != &previous)
            .copied()
    };

    while current != starting_location {
        loop_pipes.insert(current);

        let next = find_next(&current, &previous).unwrap();
        previous = current.clone();
        current = next;
    }

    loop_pipes
}

fn num_rows(input: &str) -> usize {
    input.trim().split_whitespace().count()
}

fn num_columns(input: &str) -> usize {
    let first_line = input.trim().split_whitespace().next().unwrap();
    first_line.chars().count()
}

fn all_possible_spaces(num_rows: isize, num_columns: isize) -> HashSet<Location> {
    let mut spaces = HashSet::default();
    for row in 0..num_rows {
        for column in 0..num_columns {
            let location = Location { row, column };
            spaces.insert(location);
        }
    }
    spaces
}

fn exterior_border(num_rows: isize, num_columns: isize) -> HashSet<Location> {
    let mut locations: HashSet<Location> = HashSet::default();
    // outside map bounds is guaranteed to be external
    for row in [-1, num_rows] {
        for column in 0..num_columns {
            locations.insert(Location { row, column });
        }
    }
    for column in [-1, num_columns] {
        for row in 0..num_rows {
            locations.insert(Location { row, column });
        }
    }
    locations
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Location {
    row: isize,
    column: isize,
}
impl Location {
    fn north(&self) -> Self {
        Self {
            row: self.row - 1,
            column: self.column,
        }
    }

    fn west(&self) -> Self {
        Self {
            row: self.row,
            column: self.column - 1,
        }
    }

    fn east(&self) -> Self {
        Self {
            row: self.row,
            column: self.column + 1,
        }
    }

    fn south(&self) -> Self {
        Self {
            row: self.row + 1,
            column: self.column,
        }
    }

    fn all_directions(&self) -> Vec<Self> {
        vec![self.north(), self.west(), self.south(), self.east()]
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Pipe(HashSet<Location>);
type Pipes = HashMap<Location, Pipe>;

impl Pipe {
    fn new(char: char, location: Location) -> Option<Self> {
        match char {
            '|' => Some(Pipe(HashSet::from([location.north(), location.south()]))),
            '-' => Some(Pipe(HashSet::from([location.west(), location.east()]))),
            'L' => Some(Pipe(HashSet::from([location.north(), location.east()]))),
            'J' => Some(Pipe(HashSet::from([location.north(), location.west()]))),
            '7' => Some(Pipe(HashSet::from([location.south(), location.west()]))),
            'F' => Some(Pipe(HashSet::from([location.east(), location.south()]))),
            _ => None,
        }
    }

    fn connects_to(&self, location: Location) -> bool {
        self.0.contains(&location)
    }
}

fn parse(input: &str) -> (Pipes, Location) {
    let mut pipes = HashMap::default();
    let mut squirrel = None;

    for (row, line) in input
        .trim()
        .split_whitespace()
        .filter(|l| !l.is_empty())
        .enumerate()
    {
        for (column, char) in line.chars().enumerate() {
            let location = Location {
                row: row as isize,
                column: column as isize,
            };
            if char == 'S' {
                squirrel = Some(location);
            } else if let Some(pipe) = Pipe::new(char, location) {
                pipes.insert(location, pipe);
            }
        }
    }

    let squirrel = squirrel.unwrap();

    // figure out the pipe that the squirrel's in
    let mut squirrel_pipe = Pipe::default();
    for possible_neighbor in squirrel.all_directions() {
        if let Some(neighboring_pipe) = pipes.get(&possible_neighbor) {
            if neighboring_pipe.connects_to(squirrel) {
                squirrel_pipe.0.insert(possible_neighbor);
            }
        }
    }

    assert_eq!(squirrel_pipe.0.len(), 2);
    pipes.insert(squirrel, squirrel_pipe);

    (pipes, squirrel)
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const SAMPLE1: &str = "
        .....
        .S-7.
        .|.|.
        .L-J.
        .....";

    const SAMPLE2: &str = "
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ";

    const SAMPLE3: &str = "
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";

    const SAMPLE4: &str = "
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE1), 4);
        assert_eq!(part1(SAMPLE2), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE1), 1);
        assert_eq!(part2(SAMPLE3), 4);
        assert_eq!(part2(SAMPLE4), 4);
    }
}
