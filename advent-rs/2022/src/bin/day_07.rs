/// day 7: no space left on device
use common::read_input_as_string;

#[derive(Default)]
struct FSNode {
    id: usize,
    is_dir: bool,
    name: String,
    size: usize, // 0 for dir
    parent_id: Option<usize>,
    child_ids: Vec<usize>, // empty for file
}

impl FSNode {
    fn new_file(id: usize, name: &str, size: usize, parent_id: usize) -> Self {
        Self {
            id,
            name: name.to_string(),
            size,
            parent_id: Some(parent_id),
            is_dir: false,
            ..Self::default()
        }
    }

    fn new_dir(id: usize, name: &str, parent_id: Option<usize>) -> Self {
        Self {
            id,
            parent_id,
            name: name.to_string(),
            is_dir: true,
            ..Self::default()
        }
    }
}

struct FileSystem(Vec<FSNode>);
impl FileSystem {
    fn size_of(&self, node_id: usize) -> usize {
        match self.0.get(node_id) {
            Some(node) => {
                let children_size = node
                    .child_ids
                    .iter()
                    .map(|child_id| self.size_of(*child_id))
                    .sum::<usize>();
                node.size + children_size
            }
            None => 0,
        }
    }

    fn subdir(&self, current: usize, name: &str) -> Option<usize> {
        self.0[current].child_ids.iter().find_map(|c| {
            let child = &self.0[*c];
            if child.name == name {
                Some(child.id)
            } else {
                None
            }
        })
    }

    fn add(&mut self, node: FSNode) {
        let id = node.id;
        if let Some(parent_id) = node.parent_id {
            self.0.get_mut(parent_id).unwrap().child_ids.push(id);
        }
        self.0.push(node);
    }

    fn next_id(&self) -> usize {
        self.0.len()
    }

    fn add_dir(&mut self, name: &str, parent_id: Option<usize>) -> usize {
        let id = self.next_id();
        self.add(FSNode::new_dir(id, name, parent_id));
        id
    }

    fn add_file(&mut self, name: &str, parent_id: usize, size: usize) -> usize {
        let id = self.next_id();
        self.add(FSNode::new_file(id, name, size, parent_id));
        id
    }
}

// NB: this would be significantly better with regex, but I'm feeling lazy
fn read_filesystem(input: &str) -> FileSystem {
    let mut fs = FileSystem(Vec::default());
    let mut current = fs.add_dir("/", None);
    for line in input.split('\n') {
        if line.starts_with('$') {
            if line == "$ cd /" {
                current = 0;
            } else if line == "$ cd .." {
                current = fs.0[current].parent_id.unwrap_or(current);
            } else if line == "$ ls" {
            } else {
                let dirname = line.split(' ').nth(2).unwrap();
                current = fs.subdir(current, dirname).unwrap();
            }
        } else if line.starts_with("dir") {
            let dirname = line.split(' ').nth(1).unwrap();
            fs.add_dir(dirname, Some(current));
        } else {
            let size = usize::from_str_radix(line.split(' ').nth(0).unwrap(), 10).unwrap();
            let name = line.split(' ').nth(1).unwrap();
            fs.add_file(name, current, size);
        }
    }

    fs
}

fn part1(input: &str) -> usize {
    let fs = read_filesystem(input);
    fs.0.iter()
        .filter(|node| node.is_dir)
        .filter_map(|node| {
            let size = fs.size_of(node.id);
            if size <= 100000 {
                Some(size)
            } else {
                None
            }
        })
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let fs = read_filesystem(input);
    let total_size = fs.size_of(0);
    let min_size_to_delete = 30000000 - (70000000 - total_size);

    let mut to_delete = total_size;
    for node in fs.0.iter() {
        if node.is_dir {
            let size = fs.size_of(node.id);
            if size < to_delete && size >= min_size_to_delete {
                to_delete = size;
            }
        }
    }

    to_delete
}

fn main() {
    let input = read_input_as_string(2022, 7).unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use indoc::indoc;
    const SAMPLE: &str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k"};

    #[test]
    fn test() {
        assert_eq!(95437, part1(SAMPLE));
        assert_eq!(24933642, part2(SAMPLE));
    }
}
