// day 2: bathroom security

use advent_rs_2016::read_input::read_file;
use std::collections::HashMap;

#[derive(Eq, Hash, Clone, Debug, PartialEq)]
struct Location(i8, i8);
impl Location {
    fn step(&self, direction: char) -> Location {
        match direction {
            'U' => Location(self.0, self.1 - 1),
            'D' => Location(self.0, self.1 + 1),
            'L' => Location(self.0 - 1, self.1),
            'R' => Location(self.0 + 1, self.1),
            _ => self.clone(),
        }
    }
}

#[derive(Debug)]
struct Keypad(HashMap<Location, char>);
impl Keypad {
    fn step(&self, current: Location, direction: char) -> Location {
        let location = current.step(direction);
        if self.0.contains_key(&location) {
            location
        } else {
            current
        }
    }

    fn as_key(&self, location: &Location) -> String {
        self.0.get(location).unwrap().to_string()
    }

    fn starting_location(&self) -> Option<Location> {
        for (location, key) in &self.0 {
            if key == &'5' {
                return Some(location.clone());
            }
        }

        None
    }

    fn from_str<S: AsRef<str>>(keypad_str: S) -> Self {
        let mut map: HashMap<Location, char> = HashMap::new();
        for (y, keypad_line) in keypad_str
            .as_ref()
            .split('\n')
            .filter(|x| !x.is_empty())
            .enumerate()
        {
            for (x, key) in keypad_line.chars().enumerate() {
                if key != ' ' {
                    map.insert(Location(x.try_into().unwrap(), y.try_into().unwrap()), key);
                }
            }
        }

        Self(map)
    }
}

fn follow_instruction_line<S: AsRef<str>>(
    instructions: S,
    mut location: Location,
    keypad: &Keypad,
) -> Location {
    for instruction in instructions.as_ref().chars() {
        location = keypad.step(location, instruction);
    }

    location
}

fn bathroom_code<S: AsRef<str>>(instructions: S, keypad: &Keypad) -> String {
    let mut code = String::new();
    let mut location = keypad.starting_location().unwrap();
    for instruction_line in instructions.as_ref().split('\n').filter(|x| !x.is_empty()) {
        location = follow_instruction_line(instruction_line, location.clone(), keypad);
        code.push_str(&keypad.as_key(&location));
    }

    code
}

fn main() {
    let instructions = read_file("02.txt");

    let keypad_part1 = Keypad::from_str("123\n456\n789");
    println!("{}", bathroom_code(&instructions, &keypad_part1));

    let keypad_part2 = Keypad::from_str("  1  \n 234 \n56789\n ABC \n  D  ");
    println!("{}", bathroom_code(&instructions, &keypad_part2));
}

#[cfg(test)]
mod tests {
    use super::{bathroom_code, Keypad};

    #[test]
    fn test_bathroom_code() {
        let instructions = "ULL\nRRDDD\nLURDL\nUUUUD";
        assert_eq!(
            "1985",
            bathroom_code(&instructions, &Keypad::from_str("123\n456\n789"))
        );
        assert_eq!(
            "5DB3",
            bathroom_code(
                &instructions,
                &Keypad::from_str("  1  \n 234 \n56789\n ABC \n  D  ")
            )
        );
    }
}
