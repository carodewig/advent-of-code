/// day 11: monkey in the middle

#[derive(Clone, PartialEq, Eq, Debug)]
struct Monkey {
    items: Vec<u32>,
    check_operation: fn(u32) -> u32,
    new_recipient_test: fn(u32) -> bool,
    recipients: (usize, usize),
    num_inspections: u32,
}

impl Monkey {
    fn new(
        items: Vec<u32>,
        check_operation: fn(u32) -> u32,
        new_recipient_test: fn(u32) -> bool,
        recipients: (usize, usize),
    ) -> Self {
        Self {
            items,
            check_operation,
            new_recipient_test,
            recipients,
            num_inspections: 0,
        }
    }

    fn take_turn(&mut self) -> Vec<(usize, u32)> {
        let mut outputs = Vec::default();
        for worry_level in self.items.drain(..) {
            self.num_inspections += 1;
            let new_worry_level = (self.check_operation)(worry_level) / 3;
            let recipient = if (self.new_recipient_test)(new_worry_level) {
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
        Monkey::new(vec![98, 89, 52], |x| x * 2, |x| x % 5 == 0, (6, 1)),
        Monkey::new(
            vec![57, 95, 80, 92, 57, 78],
            |x| x * 13,
            |x| x % 2 == 0,
            (2, 6),
        ),
        Monkey::new(
            vec![82, 74, 97, 75, 51, 92, 83],
            |x| x + 5,
            |x| x % 19 == 0,
            (7, 5),
        ),
        Monkey::new(vec![97, 88, 51, 68, 76], |x| x + 6, |x| x % 7 == 0, (0, 4)),
        Monkey::new(vec![63], |x| x + 1, |x| x % 17 == 0, (0, 1)),
        Monkey::new(vec![94, 91, 51, 63], |x| x + 4, |x| x % 13 == 0, (4, 3)),
        Monkey::new(
            vec![61, 54, 94, 71, 74, 68, 98, 83],
            |x| x + 2,
            |x| x % 3 == 0,
            (2, 7),
        ),
        Monkey::new(vec![90, 56], |x| x * x, |x| x % 11 == 0, (3, 5)),
    ]
}

fn monkey_business(monkeys: &mut Vec<Monkey>, num_rounds: u32) -> u32 {
    for round in 1..=num_rounds {
        take_round(monkeys);

        println!("\nAfter round {}...", round);
        for index in 0..monkeys.len() {
            println!("Monkey {}: {:?}", index, monkeys[index].items);
        }
    }

    let mut inspections: Vec<u32> = monkeys.iter().map(|m| m.num_inspections).collect();
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn main() {
    println!("{}", monkey_business(&mut monkeys(), 20));
}
#[cfg(test)]
mod test {
    use crate::{monkey_business, Monkey};

    fn test_monkeys() -> Vec<Monkey> {
        vec![
            Monkey::new(vec![79, 98], |x| x * 19, |x| x % 23 == 0, (2, 3)),
            Monkey::new(vec![54, 65, 75, 74], |x| x + 6, |x| x % 19 == 0, (2, 0)),
            Monkey::new(vec![79, 60, 97], |x| x * x, |x| x % 13 == 0, (1, 3)),
            Monkey::new(vec![74], |x| x + 3, |x| x % 17 == 0, (0, 1)),
        ]
    }

    #[test]
    fn test() {
        assert_eq!(10605, monkey_business(&mut test_monkeys(), 20));
    }
}
