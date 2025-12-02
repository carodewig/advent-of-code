/// day 2: rock paper scissors
use common::read_input_as_string;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => {
                panic!("Unexpected shape string")
            }
        }
    }

    fn score_shape(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn play_vs_other_for_result(other: Self, result: GameResult) -> Self {
        const SHAPES: [Shape; 3] = [Shape::Rock, Shape::Paper, Shape::Scissors];
        SHAPES
            .iter()
            .filter_map(|s| {
                if s.vs(other) == result {
                    Some(s)
                } else {
                    None
                }
            })
            .next()
            .unwrap()
            .clone()
    }

    fn vs(self, other: Self) -> GameResult {
        let order = [Self::Rock, Self::Paper, Self::Scissors, Self::Rock];
        let losing_combos: Vec<(Shape, Shape)> = order
            .clone()
            .into_iter()
            .zip(order.into_iter().skip(1))
            .collect();
        if self == other {
            GameResult::Draw
        } else if losing_combos.contains(&(self, other)) {
            GameResult::Loss
        } else {
            GameResult::Win
        }
    }

    fn score_vs(self, other: Self) -> u32 {
        match self.vs(other) {
            GameResult::Loss => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        }
    }

    fn score_round(self, other: Self) -> u32 {
        self.score_shape() + self.score_vs(other)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum GameResult {
    Win,
    Draw,
    Loss,
}
impl GameResult {
    fn from_str(s: &str) -> Self {
        match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => {
                panic!("Unexpected game result string")
            }
        }
    }
}

fn total_score_part1(input: &str) -> u32 {
    input
        .split('\n')
        .map(|line| {
            let elems: Vec<Shape> = line.split(' ').map(|s| Shape::from_str(s)).collect();
            elems[1].score_round(elems[0])
        })
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}
fn total_score_part2(input: &str) -> u32 {
    input
        .split('\n')
        .map(|line| {
            let mut elems = line.split(' ');
            let opponent = Shape::from_str(elems.next().unwrap());
            let target_result = GameResult::from_str(elems.next().unwrap());
            let play = Shape::play_vs_other_for_result(opponent, target_result);
            play.score_round(opponent)
        })
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}

fn main() {
    let input = read_input_as_string(2022, 2).unwrap();
    println!("{}", total_score_part1(&input));
    println!("{}", total_score_part2(&input));
}

#[cfg(test)]
mod test {
    use super::{total_score_part1, total_score_part2};
    const SAMPLE: &str = "A Y\nB X\nC Z";

    #[test]
    fn test() {
        assert_eq!(total_score_part1(SAMPLE), 15);
        assert_eq!(total_score_part2(SAMPLE), 12);
    }
}
