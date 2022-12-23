/// day 11: monkey in the middle (part 1)
const VALID_PRIMES: [u32; 9] = [2, 3, 5, 7, 11, 13, 17, 19, 23];

#[derive(Clone, PartialEq, Eq, Debug)]
struct WorryLevel(Vec<u32>);
impl WorryLevel {
    fn check(self, check_operation: fn(u32) -> u32) -> Self {
        Self(
            self.0
                .into_iter()
                .zip(VALID_PRIMES)
                .map(|(x, p)| check_operation(x) % p)
                .collect(),
        )
    }

    fn mod_(&self, factor: u32) -> bool {
        let idx = VALID_PRIMES
            .iter()
            .enumerate()
            .find(|(_, f)| f == &&factor)
            .unwrap()
            .0;
        self.0[idx] == 0
    }
}

impl From<u32> for WorryLevel {
    fn from(value: u32) -> Self {
        Self(VALID_PRIMES.iter().map(|p| value % p).collect())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Monkey {
    items: Vec<WorryLevel>,
    mod_factor: u32,
    check_operation: fn(u32) -> u32,
    recipients: (usize, usize),
    num_inspections: usize,
}

impl Monkey {
    fn new(
        items: Vec<u32>,
        mod_factor: u32,
        check_operation: fn(u32) -> u32,
        recipients: (usize, usize),
    ) -> Self {
        Self {
            items: items.into_iter().map(|item| item.into()).collect(),
            mod_factor,
            check_operation,
            recipients,
            num_inspections: 0,
        }
    }

    fn take_turn(&mut self) -> Vec<(usize, WorryLevel)> {
        let mut outputs = Vec::default();
        for worry_level in self.items.drain(..) {
            self.num_inspections += 1;
            let new_worry_level = worry_level.check(self.check_operation);
            let recipient = if new_worry_level.mod_(self.mod_factor) {
                self.recipients.0
            } else {
                self.recipients.1
            };
            outputs.push((recipient, new_worry_level));
        }
        outputs
    }
}

fn take_round(monkeys: &mut Vec<Monkey>) {
    for index in 0..monkeys.len() {
        let thrown_objects = monkeys.get_mut(index).unwrap().take_turn();
        for (recipient, worry_level) in thrown_objects.into_iter() {
            monkeys.get_mut(recipient).unwrap().items.push(worry_level);
        }
    }
}

// would be good to use regex/etc to parse these rather than creating manually
fn monkeys() -> Vec<Monkey> {
    vec![
        Monkey::new(vec![98, 89, 52], 5, |x| x * 2, (6, 1)),
        Monkey::new(vec![57, 95, 80, 92, 57, 78], 2, |x| x * 13, (2, 6)),
        Monkey::new(vec![82, 74, 97, 75, 51, 92, 83], 19, |x| x + 5, (7, 5)),
        Monkey::new(vec![97, 88, 51, 68, 76], 7, |x| x + 6, (0, 4)),
        Monkey::new(vec![63], 17, |x| x + 1, (0, 1)),
        Monkey::new(vec![94, 91, 51, 63], 13, |x| x + 4, (4, 3)),
        Monkey::new(vec![61, 54, 94, 71, 74, 68, 98, 83], 3, |x| x + 2, (2, 7)),
        Monkey::new(vec![90, 56], 11, |x| x * x, (3, 5)),
    ]
}

fn monkey_business(monkeys: &mut Vec<Monkey>, num_rounds: u32) -> usize {
    for _ in 1..=num_rounds {
        take_round(monkeys);
    }

    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.num_inspections).collect();
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn main() {
    println!("{}", monkey_business(&mut monkeys(), 10000));
}

#[cfg(test)]
mod test {
    use crate::{monkey_business, Monkey};

    fn test_monkeys() -> Vec<Monkey> {
        vec![
            Monkey::new(vec![79, 98], 23, |x| x * 19, (2, 3)),
            Monkey::new(vec![54, 65, 75, 74], 19, |x| x + 6, (2, 0)),
            Monkey::new(vec![79, 60, 97], 13, |x| x * x, (1, 3)),
            Monkey::new(vec![74], 17, |x| x + 3, (0, 1)),
        ]
    }

    #[test]
    fn test() {
        assert_eq!(2713310158, monkey_business(&mut test_monkeys(), 10000));
    }
}
