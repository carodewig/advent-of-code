// Disk Fragmenter
// This implementation makes me sad, I wish I'd been able to reuse part1 for part2

use common::read_input_as_string;

fn main() {
    println!("{}", part1::run(&read_input_as_string(2024, 9).unwrap()));
    println!("{}", part2::run(&read_input_as_string(2024, 9).unwrap()));
}

mod part1 {
    use std::fmt::{Display, Formatter, Write};

    pub fn run(input: &str) -> usize {
        let mut file_system = FileSystem::from(input);
        file_system.compact();
        file_system.checksum()
    }

    type Block = Option<usize>;

    #[derive(Debug)]
    pub(crate) struct FileSystem(Vec<Block>);

    impl FileSystem {
        fn compact(&mut self) {
            let mut left_pointer = 0;
            let mut right_pointer = self.0.len() - 1;

            while left_pointer < right_pointer {
                let left_block = self.0[left_pointer];
                let right_block = self.0[right_pointer];
                match (left_block, right_block) {
                    (Some(_), _) => left_pointer += 1,
                    (_, None) => right_pointer -= 1,
                    (None, Some(_)) => {
                        // empty space on left, value on right - move block into left space
                        self.0[left_pointer] = right_block;
                        self.0[right_pointer] = None;
                        left_pointer += 1;
                        right_pointer -= 1;
                    }
                }
            }
        }

        fn checksum(&self) -> usize {
            self.0
                .iter()
                .filter_map(|block| *block)
                .enumerate()
                .map(|(index, id)| id * index)
                .sum()
        }
    }

    impl<S: AsRef<str>> From<S> for FileSystem {
        fn from(input: S) -> Self {
            let mut disk_map = Vec::default();
            let mut id = 0;

            for (char, &is_file) in input
                .as_ref()
                .trim()
                .chars()
                .zip([true, false].iter().cycle())
            {
                let num_blocks = char.to_string().parse().unwrap();

                let block = if is_file { Some(id) } else { None };
                for _ in 0..num_blocks {
                    disk_map.push(block);
                }

                if is_file {
                    id += 1;
                }
            }

            Self(disk_map)
        }
    }

    impl Display for FileSystem {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for block in &self.0 {
                match block {
                    Some(id) => f.write_str(&id.to_string())?,
                    None => f.write_char('.')?,
                }
            }

            Ok(())
        }
    }
}

mod part2 {
    use std::fmt::{Display, Formatter, Write};

    pub fn run(input: &str) -> usize {
        let mut file_system = FileSystem::from(input);
        file_system.compact();
        file_system.checksum()
    }

    #[derive(Copy, Clone, Debug)]
    struct Block {
        length: usize,
        id: Option<usize>,
    }

    #[derive(Debug)]
    pub(crate) struct FileSystem(Vec<Block>);

    impl FileSystem {
        fn compact(&mut self) {
            let mut right_pointer = self.0.len() - 1;
            loop {
                let right_block = self.0[right_pointer];
                if right_block.id.is_none() {
                    if right_pointer == 0 {
                        break;
                    }

                    right_pointer -= 1;
                    continue;
                }

                let mut left_pointer = 0;
                while left_pointer < right_pointer {
                    let left_block = self.0[left_pointer];
                    if left_block.id.is_some() {
                        left_pointer += 1;
                        continue;
                    }

                    match (left_block.id, right_block.id) {
                        (Some(_), _) => left_pointer += 1,
                        (_, None) => unreachable!(),
                        (None, Some(_)) => {
                            // empty space on left, value on right - verify that the block will fit, then
                            // attempt to move value into left space
                            if left_block.length >= right_block.length {
                                let remaining_free = left_block.length - right_block.length;
                                self.0[left_pointer] = right_block;
                                self.0[right_pointer] = Block {
                                    id: None,
                                    ..right_block
                                };

                                if remaining_free > 0 {
                                    let new_block = Block {
                                        length: remaining_free,
                                        id: None,
                                    };
                                    self.0.insert(left_pointer + 1, new_block);
                                    right_pointer += 1;
                                }
                                break;
                            }

                            // right block didn't fit into this gap, move to next block on right
                            left_pointer += 1;
                        }
                    }
                }

                if right_pointer == 0 {
                    break;
                }

                right_pointer -= 1;
            }
        }

        fn checksum(&self) -> usize {
            let mut total = 0;
            let mut index = 0;

            for block in &self.0 {
                for _ in 0..block.length {
                    if let Some(id) = block.id {
                        total += index * id;
                    }
                    index += 1;
                }
            }

            total
        }
    }

    impl<S: AsRef<str>> From<S> for FileSystem {
        fn from(input: S) -> Self {
            let mut disk_map = Vec::default();
            let mut id = 0;

            for (char, &is_file) in input
                .as_ref()
                .trim()
                .chars()
                .zip([true, false].iter().cycle())
            {
                let num_blocks = char.to_string().parse().unwrap();
                if num_blocks > 0 {
                    let block_id = if is_file { Some(id) } else { None };
                    disk_map.push(Block {
                        length: num_blocks,
                        id: block_id,
                    });
                }

                if is_file {
                    id += 1;
                }
            }

            Self(disk_map)
        }
    }

    impl Display for FileSystem {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for block in &self.0 {
                for _ in 0..block.length {
                    match block.id {
                        Some(id) => f.write_str(&id.to_string())?,
                        None => f.write_char('.')?,
                    }
                }
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const SAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_parse() {
        let file_system = part1::FileSystem::from(SAMPLE);
        let expected_result = "00...111...2...333.44.5555.6666.777.888899";
        assert_eq!(&file_system.to_string(), expected_result);

        let file_system = part2::FileSystem::from(SAMPLE);
        assert_eq!(&file_system.to_string(), expected_result);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1::run(SAMPLE), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::run(SAMPLE), 2858);
    }
}
