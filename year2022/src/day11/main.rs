use std::cell::RefCell;
use std::collections::LinkedList;
use std::fs;
use std::str::FromStr;

use anyhow::{bail, Result};

use monkey::Monkey;

pub mod monkey;

fn main() -> Result<()> {
    let data = fs::read_to_string("data/day11.txt")?;
    let lines: Vec<&str> = data
        .split('\n')
        .map(|e| e.trim())
        .filter(|s| !s.is_empty())
        .collect();

    println!("Part 1: {}", part1(&lines)?);
    println!("Part 2: {}", part2(&lines)?);
    Ok(())
}

fn part1(lines: &[&str]) -> Result<usize> {
    let mut monkeys = Vec::<RefCell<Monkey>>::new();
    for chunk in lines.chunks(6) {
        let (monkey, _) = parse_monkey(chunk)?;
        monkeys.push(RefCell::new(monkey));
    }

    for _ in 0..20 {
        for monkey in &monkeys {
            while monkey.borrow().has_items() {
                let catcher_id = monkey.borrow_mut().inspect_next_item(None).unwrap();
                let item = monkey.borrow_mut().throw_item().unwrap();
                monkeys[catcher_id].borrow_mut().catch_item(item);
            }
        }
    }

    let mut inspection_counts = monkeys
        .iter()
        .map(|m| m.borrow().inspections())
        .collect::<Vec<_>>();
    inspection_counts.sort_by(|a, b| b.cmp(a));
    Ok(inspection_counts[0] * inspection_counts[1])
}

fn part2(lines: &[&str]) -> Result<usize> {
    let mut gcd = 1;
    let mut monkeys = Vec::<RefCell<Monkey>>::new();
    for chunk in lines.chunks(6) {
        let (monkey, divider) = parse_monkey(chunk)?;
        monkeys.push(RefCell::new(monkey));
        gcd *= divider;
    }

    for _ in 0..10_000 {
        for monkey in &monkeys {
            while monkey.borrow().has_items() {
                let catcher_id = monkey.borrow_mut().inspect_next_item(Some(gcd)).unwrap();
                let item = monkey.borrow_mut().throw_item().unwrap();
                monkeys[catcher_id].borrow_mut().catch_item(item);
            }
        }
    }

    let mut inspection_counts = monkeys
        .iter()
        .map(|m| m.borrow().inspections())
        .collect::<Vec<_>>();
    inspection_counts.sort_by(|a, b| b.cmp(a));
    Ok(inspection_counts[0] * inspection_counts[1])
}

fn parse_monkey(chunk: &[&str]) -> Result<(Monkey, u64)> {
    let id = match chunk[0].split_whitespace().nth(1) {
        None => bail!("monkey id not found"),
        Some(s) => usize::from_str(&s.replace(':', ""))?,
    };
    let items: LinkedList<u64> = chunk[1]
        .strip_prefix("Starting items:")
        .unwrap()
        .split(',')
        .filter_map(|e| u64::from_str(e.trim()).ok())
        .collect();
    let op_tokens: Vec<&str> = chunk[2].split_whitespace().collect();
    let second_operand = match op_tokens.last().unwrap() {
        &"old" => None,
        _ => match u64::from_str(op_tokens.last().unwrap()) {
            Ok(value) => Some(value),
            Err(_) => bail!("error parsing second operand"),
        },
    };
    let o = op_tokens[4].to_owned();
    let op = move |x| {
        let y = match second_operand {
            None => x,
            Some(value) => value,
        };

        let res = match o.as_str() {
            "+" => x + y,
            "*" => x * y,
            _ => panic!("unknown operation"),
        };
        res
    };
    let divider: u64 = chunk[3].split_whitespace().last().unwrap().parse()?;
    let true_monkey_id: usize = chunk[4].split_whitespace().last().unwrap().parse()?;
    let false_monkey_id: usize = chunk[5].split_whitespace().last().unwrap().parse()?;
    let test = move |x| {
        if x % divider == 0 {
            true_monkey_id
        } else {
            false_monkey_id
        }
    };

    Ok((
        Monkey::new(id, items, Box::new(op), Box::new(test)),
        divider,
    ))
}
