use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::VecDeque, num::ParseFloatError, str::FromStr};

#[derive(Debug)]
struct ParseMonkeyError;

impl From<ParseFloatError> for ParseMonkeyError {
    fn from(_value: ParseFloatError) -> Self {
        return ParseMonkeyError;
    }
}

#[derive(Debug)]
enum MonkeyOperation {
    Add(f64),
    AddSelf,
    Multiply(f64),
    MultiplySelf,
}

impl FromStr for MonkeyOperation {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut line_split = s.split_whitespace().rev();
        let other = line_split.next().ok_or(ParseMonkeyError)?;
        let operator = line_split.next().ok_or(ParseMonkeyError)?;
        match (other, operator) {
            ("old", "*") => Ok(MonkeyOperation::MultiplySelf),
            ("old", "+") => Ok(MonkeyOperation::AddSelf),
            (_, "*") => Ok(MonkeyOperation::Multiply(other.parse::<f64>()?)),
            (_, "+") => Ok(MonkeyOperation::Add(other.parse::<f64>()?)),
            _ => Err(ParseMonkeyError),
        }
    }
}

#[derive(Debug)]
enum MonkeyTest {
    DivisibleBy(f64),
}

impl FromStr for MonkeyTest {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.contains("divisible by") {
            let num = s
                .split_whitespace()
                .last()
                .ok_or(ParseMonkeyError)?
                .parse::<f64>()?;

            return Ok(MonkeyTest::DivisibleBy(num));
        }
        return Err(ParseMonkeyError);
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<f64>,
    operation: MonkeyOperation,
    test: MonkeyTest,
    test_true_monkey: usize,
    test_false_monkey: usize,
    inspection_count: usize,
}

impl FromStr for Monkey {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            static ref MONKEY_LINE_RE: Regex = Regex::new(r"\d+").unwrap();
        }
        let mut lines = s.split("\n");
        lines.next();
        let starting_items_line = lines.next().ok_or(ParseMonkeyError)?;
        let starting_items = MONKEY_LINE_RE
            .find_iter(starting_items_line)
            .flat_map(|item| item.as_str().parse::<f64>())
            .collect();
        let operation_line = lines.next().ok_or(ParseMonkeyError)?;
        let operation = operation_line.parse::<MonkeyOperation>()?;
        let test_line = lines.next().ok_or(ParseMonkeyError)?;
        let test = test_line.parse::<MonkeyTest>()?;
        let test_true_line = lines.next().ok_or(ParseMonkeyError)?;
        let test_true = MONKEY_LINE_RE
            .find_iter(test_true_line)
            .last()
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let test_false_line = lines.next().ok_or(ParseMonkeyError)?;
        let test_false = MONKEY_LINE_RE
            .find_iter(test_false_line)
            .last()
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        return Ok(Monkey {
            items: starting_items,
            operation,
            test,
            test_true_monkey: test_true,
            test_false_monkey: test_false,
            inspection_count: 0,
        });
    }
}

fn main() -> Result<()> {
    let mut monkeys = include_str!("./day11.prod")
        .split("\n\n")
        .map(|line| line.parse::<Monkey>().unwrap())
        .collect::<Vec<Monkey>>();

    for _i in 0..20 {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let monkey = monkeys.get_mut(i).unwrap();
                let item = monkey.items.pop_front().unwrap();
                let item = match monkey.operation {
                    MonkeyOperation::AddSelf => item + item,
                    MonkeyOperation::MultiplySelf => item * item,
                    MonkeyOperation::Add(num) => item + num,
                    MonkeyOperation::Multiply(num) => item * num,
                };
                let item = (item / 3.0).floor();
                let monkey_to_throw_to = match monkey.test {
                    MonkeyTest::DivisibleBy(divisor) if item % divisor == 0.0 => {
                        monkey.test_true_monkey
                    }
                    _ => monkey.test_false_monkey,
                };
                monkey.inspection_count += 1;
                let other_monkey = monkeys.get_mut(monkey_to_throw_to).unwrap();
                other_monkey.items.push_back(item);
            }
        }
    }

    monkeys.sort_by_cached_key(|k| k.inspection_count);
    let result: usize = monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspection_count)
        .product();

    println!("Part 1: {}", result);

    let mut monkeys = include_str!("./day11.prod")
        .split("\n\n")
        .map(|line| line.parse::<Monkey>().unwrap())
        .collect::<Vec<Monkey>>();

    let mod_num: f64 = monkeys
        .iter()
        .map(|m| match m.test {
            MonkeyTest::DivisibleBy(x) => x,
        })
        .product();

    for _i in 0..10000 {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let monkey = monkeys.get_mut(i).unwrap();
                let item = monkey.items.pop_front().unwrap();
                let item = match monkey.operation {
                    MonkeyOperation::AddSelf => item + item,
                    MonkeyOperation::MultiplySelf => item * item,
                    MonkeyOperation::Add(num) => item + num,
                    MonkeyOperation::Multiply(num) => item * num,
                };
                let item = item % mod_num;
                let monkey_to_throw_to = match monkey.test {
                    MonkeyTest::DivisibleBy(divisor) if item % divisor == 0.0 => {
                        monkey.test_true_monkey
                    }
                    _ => monkey.test_false_monkey,
                };
                monkey.inspection_count += 1;
                let other_monkey = monkeys.get_mut(monkey_to_throw_to).unwrap();
                other_monkey.items.push_back(item);
            }
        }
    }

    monkeys.sort_by_cached_key(|k| k.inspection_count);
    let result: usize = monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspection_count)
        .product();

    println!("Part 2: {}", result);
    return Ok(());
}
