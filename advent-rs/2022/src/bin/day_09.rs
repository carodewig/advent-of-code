/// day 9: rope bridge
use common::read_input_as_string;
use std::collections::HashSet;

type Location = (isize, isize);

struct Rope(Vec<Location>);
impl Rope {
    fn new(num_knots: usize) -> Self {
        Self((0..num_knots).map(|_| (0, 0)).collect())
    }

    fn tail(&self) -> Location {
        self.0.last().unwrap().clone()
    }

    fn move_head(&mut self, direction: char) {
        let head = self.0.get_mut(0).unwrap();
        match direction {
            'R' => head.0 += 1,
            'L' => head.0 -= 1,
            'U' => head.1 += 1,
            'D' => head.1 -= 1,
            _ => {}
        }
    }

    fn follow_head(&mut self) {
        let rope_length = self.0.len();
        for index in 1..rope_length {
            let prev_knot = self.0.get(index - 1).unwrap().clone();
            let knot = self.0.get_mut(index).unwrap();

            let x_delta = prev_knot.0 - knot.0;
            let y_delta = prev_knot.1 - knot.1;
            if x_delta.abs() > 1 || y_delta.abs() > 1 {
                knot.0 += 1 * x_delta.signum();
                knot.1 += 1 * y_delta.signum();
            }
        }
    }

    fn step(&mut self, direction: char) {
        self.move_head(direction);
        self.follow_head();
    }
}

fn num_positions_visited(input: &str, num_knots: usize) -> usize {
    let mut rope = Rope::new(num_knots);
    let mut visited: HashSet<Location> = HashSet::default();
    visited.insert(rope.tail());

    for line in input.split('\n') {
        let mut line_split = line.split(' ');
        let direction = line_split.next().unwrap().chars().next().unwrap();
        let num_steps = usize::from_str_radix(line_split.next().unwrap(), 10).unwrap();
        for _ in 0..num_steps {
            rope.step(direction);
            visited.insert(rope.tail());
        }
    }

    visited.len()
}

fn main() {
    let input = read_input_as_string(2022, 9).unwrap();
    println!("{}", num_positions_visited(&input, 2));
    println!("{}", num_positions_visited(&input, 10));
}

#[cfg(test)]
mod test {
    use crate::num_positions_visited;

    const SAMPLE1: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    const SAMPLE2: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

    #[test]
    fn test() {
        assert_eq!(13, num_positions_visited(SAMPLE1, 2));
        assert_eq!(36, num_positions_visited(SAMPLE2, 10));
    }
}
