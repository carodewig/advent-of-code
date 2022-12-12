/// day 8: treetop tree house
use common::read_input_as_string;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

struct Tree {
    height: usize,
}
impl Tree {
    fn from_char(c: char) -> Self {
        Self {
            height: usize::from_str_radix(&format!("{}", c), 10).unwrap(),
        }
    }
}

fn insert_from<T: Eq + Hash>(h: &mut HashSet<T>, mut i: HashSet<T>) {
    for item in i.drain() {
        h.insert(item);
    }
}

struct TreeMap(HashMap<(isize, isize), Tree>);
impl TreeMap {
    fn from_str(input: &str) -> Self {
        let mut tree_map = Self(HashMap::default());
        for (line_idx, line) in input
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .enumerate()
        {
            for (char_idx, c) in line.trim().chars().enumerate() {
                tree_map
                    .0
                    .insert((char_idx as isize, line_idx as isize), Tree::from_char(c));
            }
        }

        tree_map
    }

    fn max_x(&self) -> isize {
        self.0.keys().map(|k| k.0).max().unwrap_or(0)
    }
    fn max_y(&self) -> isize {
        self.0.keys().map(|k| k.1).max().unwrap_or(0)
    }

    fn get(&self, x: isize, y: isize) -> &Tree {
        self.0.get(&(x, y)).unwrap()
    }

    fn get_height(&self, x: isize, y: isize) -> usize {
        self.0.get(&(x, y)).unwrap().height
    }

    fn visible_from_outside(
        &self,
        mut x: isize,
        mut y: isize,
        x_step: isize,
        y_step: isize,
    ) -> HashSet<(isize, isize)> {
        let max_x = self.max_x();
        let max_y = self.max_y();

        let mut visible = HashSet::default();
        visible.insert((x, y));

        let mut max_height = self.get(x, y).height;

        while 0 <= x && x <= max_x && 0 <= y && y <= max_y {
            let tree_height = self.get_height(x, y);
            if tree_height > max_height {
                max_height = tree_height;
                visible.insert((x, y));
            }

            x += x_step;
            y += y_step;
        }

        visible
    }

    fn num_visible_from_outside(&self) -> usize {
        let max_x = self.max_x();
        let max_y = self.max_y();

        let mut visible: HashSet<(isize, isize)> = HashSet::default();
        for x in 0..=max_x {
            insert_from(&mut visible, self.visible_from_outside(x, 0, 0, 1));
            insert_from(&mut visible, self.visible_from_outside(x, max_y, 0, -1));
        }
        for y in 0..=max_y {
            insert_from(&mut visible, self.visible_from_outside(0, y, 1, 0));
            insert_from(&mut visible, self.visible_from_outside(max_x, y, -1, 0));
        }

        visible.len()
    }

    fn visible_from_tree_house(
        &self,
        mut x: isize,
        mut y: isize,
        x_step: isize,
        y_step: isize,
        max_x: isize,
        max_y: isize,
    ) -> usize {
        let mut visible = 0;
        let treehouse_height = self.get_height(x, y);
        x += x_step;
        y += y_step;

        while 0 <= x && x <= max_x && 0 <= y && y <= max_y {
            visible += 1;
            if treehouse_height <= self.get_height(x, y) {
                break;
            }

            x += x_step;
            y += y_step;
        }

        visible
    }

    fn best_scenic_score(&self) -> usize {
        let max_x = self.max_x();
        let max_y = self.max_y();
        self.0
            .keys()
            .map(|(x, y)| {
                self.visible_from_tree_house(*x, *y, 1, 0, max_x, max_y)
                    * self.visible_from_tree_house(*x, *y, -1, 0, max_x, max_y)
                    * self.visible_from_tree_house(*x, *y, 0, 1, max_x, max_y)
                    * self.visible_from_tree_house(*x, *y, 0, -1, max_x, max_y)
            })
            .max()
            .unwrap_or(0)
    }
}

fn main() {
    let input = read_input_as_string(2022, 8).unwrap();
    let tree_map = TreeMap::from_str(&input);
    println!("{}", tree_map.num_visible_from_outside());
    println!("{}", tree_map.best_scenic_score());
}

#[cfg(test)]
mod test {
    use crate::TreeMap;
    use indoc::indoc;
    const SAMPLE: &str = indoc! {"
        30373
        25512
        65332
        33549
        35390"
    };

    #[test]
    fn test() {
        let tree_map = TreeMap::from_str(SAMPLE);
        assert_eq!(21, tree_map.num_visible_from_outside());
        assert_eq!(8, tree_map.best_scenic_score());
    }
}
