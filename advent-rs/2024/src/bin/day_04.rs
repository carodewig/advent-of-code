// Ceres Search

use common::read_input_as_string;
use std::convert::Infallible;
use std::str::FromStr;

fn main() {
    let word_search = WordSearch::from_str(&read_input_as_string(2024, 4).unwrap()).unwrap();
    println!("{}", word_search.find_all_part1());
    println!("{}", word_search.find_all_part2());
}

#[derive(Debug)]
struct WordSearch(Vec<Vec<char>>);
impl FromStr for WordSearch {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WordSearch(
            s.split('\n').map(|l| l.trim().chars().collect()).collect(),
        ))
    }
}

impl WordSearch {
    fn char_at(&self, row: i64, column: i64) -> Option<char> {
        let row = usize::try_from(row).ok()?;
        let column = usize::try_from(column).ok()?;
        self.0.get(row)?.get(column).copied()
    }

    fn char_at_eq(&self, row: i64, column: i64, target: char) -> bool {
        self.char_at(row, column) == Some(target)
    }

    fn search_for_xmas_from(&self, row: i64, column: i64) -> i64 {
        if !self.char_at_eq(row, column, 'X') {
            return 0;
        }

        let mut matches = 0;
        for row_step in [-1, 0, 1] {
            for column_step in [-1, 0, 1] {
                // look for M, A, S
                if !self.char_at_eq(row + row_step, column + column_step, 'M') {
                    continue;
                }
                if !self.char_at_eq(row + 2 * row_step, column + 2 * column_step, 'A') {
                    continue;
                }
                if !self.char_at_eq(row + 3 * row_step, column + 3 * column_step, 'S') {
                    continue;
                }
                matches += 1;
            }
        }

        matches
    }

    fn search_for_mas_from(&self, row: i64, column: i64) -> i64 {
        if !self.char_at_eq(row, column, 'A') {
            return 0;
        }

        let ms = ['M', 'S'];
        let opposite = |c: &char| {
            if c == &'M' { 'S' } else { 'M' }
        };

        match (
            self.char_at(row - 1, column - 1),
            self.char_at(row - 1, column + 1),
        ) {
            (Some(nw), Some(ne)) if ms.contains(&nw) && ms.contains(&ne) => {
                let sw = self.char_at(row + 1, column - 1);
                let se = self.char_at(row + 1, column + 1);
                if sw == Some(opposite(&ne)) && se == Some(opposite(&nw)) {
                    return 1;
                }
            }
            _ => {}
        }

        0
    }

    #[allow(clippy::cast_possible_wrap)]
    fn find_all_part1(&self) -> i64 {
        let mut matches = 0;
        for row in 0..self.0.len() {
            for column in 0..self.0[row].len() {
                matches += self.search_for_xmas_from(row as i64, column as i64);
            }
        }

        matches
    }

    #[allow(clippy::cast_possible_wrap)]
    fn find_all_part2(&self) -> i64 {
        let mut matches = 0;
        for row in 0..self.0.len() {
            for column in 0..self.0[row].len() {
                matches += self.search_for_mas_from(row as i64, column as i64);
            }
        }

        matches
    }
}

#[cfg(test)]
mod tests {
    use crate::WordSearch;
    use std::str::FromStr;

    const SAMPLE: &str = "\
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX";

    #[test]
    fn test_part1() {
        let word_search = WordSearch::from_str(SAMPLE).unwrap();
        assert_eq!(word_search.find_all_part1(), 18);
    }
    #[test]
    fn test_part2() {
        let word_search = WordSearch::from_str(SAMPLE).unwrap();
        assert_eq!(word_search.find_all_part2(), 9);
    }
}
