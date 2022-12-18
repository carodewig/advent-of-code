/// day 10: cathode-ray tube
use common::read_input_as_string;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Cpu {
    register_x: i32,
    pixel_position: u32,
    clock: u32,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            register_x: 1,
            pixel_position: 0,
            clock: 0,
        }
    }
}

impl Cpu {
    fn tick(&mut self) -> Option<i64> {
        self.clock += 1;
        let pixel_x = self.pixel_position % 40;
        if self.register_x <= pixel_x as i32 + 1 && pixel_x as i32 <= self.register_x + 1 {
            print!("#");
        } else {
            print!(".");
        }

        self.pixel_position += 1;
        if self.pixel_position % 40 == 0 {
            println!("");
        }

        if [20, 60, 100, 140, 180, 220].contains(&self.clock) {
            Some(self.clock as i64 * self.register_x as i64)
        } else {
            None
        }
    }

    fn noop(&mut self) -> Option<i64> {
        self.tick()
    }

    fn addx(&mut self, value: i32) -> Option<i64> {
        let tick1 = self.tick();
        let tick2 = self.tick();
        self.register_x += value;
        match (tick1, tick2) {
            (Some(t), None) => Some(t),
            (None, Some(t)) => Some(t),
            (None, None) => None,
            _ => panic!("Should be impossible to get two values"),
        }
    }
}

fn process_program(input: &str) -> i64 {
    let mut cpu = Cpu::default();
    input
        .split('\n')
        .filter_map(|line| {
            if line == "noop" {
                cpu.noop()
            } else {
                let value: i32 = line.split(' ').nth(1).unwrap().parse().unwrap();
                cpu.addx(value)
            }
        })
        .sum()
}

fn main() {
    let input = read_input_as_string(2022, 10).unwrap();
    println!("{}", process_program(&input));
}
