use std::{fs, str::FromStr};

#[derive(Debug)]
enum Operation {
    Add(Option<i64>, Option<i64>), // None = "old"
    Mul(Option<i64>, Option<i64>),
}

impl Operation {
    fn run(&self, old: i64) -> i64 {
        match self {
            Operation::Add(l, r) => l.unwrap_or(old) + r.unwrap_or(old),
            Operation::Mul(l, r) => l.unwrap_or(old) * r.unwrap_or(old),
        }
    }
}

impl FromStr for Operation {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, r) = s.split_once('=').ok_or("Missing =")?;
        let mut s = r.split_whitespace();
        let lhs = s
            .next()
            .ok_or("Missing lhs in operation")?
            .parse::<i64>()
            .ok();
        let op = s.next().ok_or("Missing op in operation")?;
        let rhs = s
            .next()
            .ok_or("Missing rhs in operation")?
            .parse::<i64>()
            .ok();
        Ok(match op {
            "+" => Self::Add(lhs, rhs),
            "*" => Self::Mul(lhs, rhs),
            _ => return Err("Invalid operation in input"),
        })
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    next: (usize, usize), // true, false
    inspections: u64,
}

fn first_number<T: FromStr>(s: &str) -> Option<T> {
    s.split_whitespace().flat_map(str::parse).next()
}

impl FromStr for Monkey {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items: Vec<i64> = vec![];
        let mut operation = Operation::Add(None, None);
        let mut test = 1;
        let mut next = (0, 0);
        for line in s.lines().skip(1) {
            let (attr, value) = line.split_once(": ").ok_or("Invalid input")?;
            match attr.trim() {
                "Starting items" => {
                    items = value
                        .split(", ")
                        .map(str::parse)
                        .collect::<Result<Vec<i64>, _>>()
                        .map_err(|_| "Invalid number in input")?
                }
                "Operation" => operation = Operation::from_str(value)?,
                "Test" => test = first_number(value).ok_or("Missing divisor in test")?,
                "If true" => next.0 = first_number(value).ok_or("Missing divisor in test")?,
                "If false" => next.1 = first_number(value).ok_or("Missing divisor in test")?,
                _ => return Err("Invalid input"),
            };
        }
        Ok(Monkey {
            items,
            operation,
            test,
            next,
            inspections: 0,
        })
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(Monkey::from_str)
        .collect::<Result<Vec<Monkey>, _>>()
        .expect("Invalid input")
}

fn run_inspections(input: &str, rounds: i32, divisor: i64) -> u64 {
    let mut monkeys = parse_input(input);
    let modulus: i64 = monkeys.iter().map(|m| m.test).product();

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            let mut items = std::mem::take(&mut monkeys[m].items);
            monkeys[m].inspections += items.len() as u64;

            let monkey = &monkeys[m];
            items.iter_mut().for_each(|item| {
                *item = (monkey.operation.run(*item) / divisor) % modulus;
            });

            let (m_true, m_false) = monkey.next;
            let test = monkey.test;
            for item in items.into_iter() {
                if item % test == 0 {
                    monkeys[m_true].items.push(item);
                } else {
                    monkeys[m_false].items.push(item);
                }
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    monkeys.iter().take(2).map(|m| m.inspections).product()
}

fn part1(input: &str) -> u64 {
    run_inspections(input, 20, 3)
}

fn part2(input: &str) -> u64 {
    run_inspections(input, 10000, 1)
}

fn main() {
    let input = fs::read_to_string("inputs/day11.txt").expect("Could not read input file");
    let part1 = part1(&input);
    let part2 = part2(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 10605);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2713310158);
    }
}
