fn main() {
    let mut monkeys = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        if line.starts_with("Monkey ") {
            assert_eq!(line[7..], format!("{}:", monkeys.len()));
            monkeys.push(Monkey::default());
        } else if line.starts_with("  Starting items: ") {
            let items = line[18..].split(", ").map(|s| s.parse::<usize>().unwrap());
            monkeys.last_mut().unwrap().items.extend(items);
        } else if line.starts_with("  Operation: new = old ") {
            let op = &line[23..24];
            let rhs = &line[25..];
            if op == "*" && rhs == "old" {
                monkeys.last_mut().unwrap().operation = Operation::Square;
            } else {
                let rhs = rhs.parse::<usize>().unwrap();
                monkeys.last_mut().unwrap().operation = if op == "+" {
                    Operation::Add(rhs)
                } else {
                    Operation::Multiply(rhs)
                };
            }
        } else if line.starts_with("  Test: divisible by ") {
            monkeys.last_mut().unwrap().threshold = line[21..].parse::<usize>().unwrap();
        } else if line.starts_with("    If true: throw to monkey ") {
            monkeys.last_mut().unwrap().if_true = line[29..].parse::<usize>().unwrap();
        } else if line.starts_with("    If false: throw to monkey ") {
            monkeys.last_mut().unwrap().if_false = line[30..].parse::<usize>().unwrap();
        }
    }

    println!("Part 1: {}", monkey_business(monkeys.clone(), 20, true));
    println!("Part 2: {}", monkey_business(monkeys, 10_000, false));
}

#[derive(Default, Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    threshold: usize,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

#[derive(Default, Debug, Clone)]
enum Operation {
    #[default]
    Square,
    Add(usize),
    Multiply(usize),
}

impl Operation {
    fn apply(&self, rhs: usize) -> usize {
        match self {
            Self::Square => &rhs * &rhs,
            Self::Add(n) => *n + rhs,
            Self::Multiply(n) => *n * rhs,
        }
    }
}

fn monkey_business(mut monkeys: Vec<Monkey>, rounds: usize, should_divide: bool) -> usize {
    let all_monkeys_thresholds = monkeys.iter().map(|m| m.threshold).product::<usize>();

    for _round in 0..rounds {
        for monkey in 0..monkeys.len() {
            let thresh = monkeys[monkey].threshold;

            while let Some(mut item) = monkeys[monkey].items.pop() {
                monkeys[monkey].inspections += 1;
                item = monkeys[monkey].operation.apply(item);
                if should_divide {
                    item /= 3;
                }
                let to = if &item % thresh == 0 {
                    monkeys[monkey].if_true
                } else {
                    monkeys[monkey].if_false
                };
                monkeys[to].items.push(item % all_monkeys_thresholds);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspections);
    monkeys.reverse();
    monkeys[0].inspections * monkeys[1].inspections
}
