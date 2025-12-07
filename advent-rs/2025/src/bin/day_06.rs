use common::read_input_as_string;

#[derive(Copy, Clone, Debug)]
enum Operation {
    Addition,
    Multiplication,
}
impl TryFrom<char> for Operation {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Addition),
            '*' => Ok(Self::Multiplication),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
struct Problem {
    values: Vec<u64>,
    operation: Operation,
}

impl From<Operation> for Problem {
    fn from(operation: Operation) -> Self {
        Self {
            operation,
            values: Vec::new(),
        }
    }
}

impl Problem {
    fn solve(&self) -> u64 {
        let reduce = match self.operation {
            Operation::Addition => |acc, e| acc + e,
            Operation::Multiplication => |acc, e| acc * e,
        };
        self.values.iter().copied().reduce(reduce).unwrap()
    }
}

fn as_number(v: &[char]) -> Option<u64> {
    // remove spaces and parse char iter as u64 if possible
    v.iter()
        .filter(|c| **c != ' ')
        .collect::<String>()
        .parse()
        .ok()
}

fn parse_part1(input: &str) -> Vec<Problem> {
    let mut values: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<Operation> = Vec::new();
    for line in input.split('\n') {
        for (index, value) in line.split_whitespace().enumerate() {
            match value {
                "*" => operations.push(Operation::Multiplication),
                "+" => operations.push(Operation::Addition),
                v => {
                    let v: u64 = v.parse().unwrap();
                    if let Some(problem_values) = values.get_mut(index) {
                        problem_values.push(v);
                    } else {
                        values.push(vec![v]);
                    }
                }
            }
        }
    }

    values
        .into_iter()
        .zip(operations)
        .map(|(values, operation)| Problem { values, operation })
        .collect()
}

fn parse_part2(input: &str) -> Vec<Problem> {
    let lines: Vec<Vec<char>> = input.split('\n').map(|l| l.chars().collect()).collect();
    let num_columns = lines.iter().map(Vec::len).max().expect("no lines");

    let mut problems: Vec<Problem> = Vec::new();

    let num_values = lines.len() - 1;
    let mut problem = None;

    for column_index in 0..num_columns {
        let column: Vec<char> = lines
            .iter()
            .map(|l| l.get(column_index).copied().unwrap_or(' '))
            .collect();

        // new operation - push prev problem and reinitialize
        if let Ok(operation) = Operation::try_from(column[num_values]) {
            if let Some(p) = problem {
                problems.push(p);
            }
            problem = Some(Problem::from(operation));
        }

        if let Some(value) = as_number(&column[..num_values])
            && let Some(p) = problem.as_mut()
        {
            p.values.push(value);
        }
    }

    if let Some(p) = problem {
        problems.push(p);
    }
    problems
}

fn solve(problems: &[Problem]) -> u64 {
    problems.iter().map(Problem::solve).sum()
}

fn main() {
    let problems = parse_part1(&read_input_as_string(2025, 6).unwrap());
    assert_eq!(solve(&problems), 5_524_274_308_182);

    let problems = parse_part2(&read_input_as_string(2025, 6).unwrap());
    assert_eq!(solve(&problems), 8_843_673_199_391);
}

#[cfg(test)]
mod tests {
    use super::{parse_part1, parse_part2, solve};

    const SAMPLE: &str = "\
123 328  51 64 
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        let problems = parse_part1(SAMPLE);
        assert_eq!(solve(&problems), 4_277_556);
    }

    #[test]
    fn test_part2() {
        let problems = parse_part2(SAMPLE);
        assert_eq!(solve(&problems), 3_263_827);
    }
}
