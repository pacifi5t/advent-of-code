use anyhow::Result;
use std::collections::BTreeMap;
use std::fs;

fn main() -> Result<()> {
    let data = fs::read_to_string("data/day5.txt")?;
    let mut split = data.split("\n\n");
    let stacks_data = split.next().expect("stacks should be there");
    let commands = split.next().expect("commands should be there");
    let movers = [mover_9000, mover_9001];

    for (i, mover) in movers.iter().enumerate() {
        let mut stacks = build_stacks_map(stacks_data);
        for each in commands.trim().split('\n') {
            let mut tokens = each.split_whitespace();
            let count = tokens.nth(1).unwrap().parse::<usize>()?;
            let src = tokens.nth(1).unwrap();
            let dst = tokens.nth(1).unwrap();

            mover(&mut stacks, count, src, dst);
        }
        println!("Part {}: {}", i + 1, extract_message(&stacks));
    }

    Ok(())
}

fn build_stacks_map(stacks_data: &str) -> BTreeMap<&str, Vec<char>> {
    let mut map = BTreeMap::new();
    let mut rows: Vec<_> = stacks_data.split('\n').rev().collect();
    let keys_row = rows.remove(0).trim();

    for each in keys_row.split(' ').filter(|e| !e.is_empty()) {
        map.insert(each, vec![]);
    }

    for key in 1..=map.len() {
        let vec = map.get_mut(&*key.to_string()).expect("vec should be there");
        for row in rows.iter() {
            let ch = row.chars().nth(1 + (key - 1) * 4).unwrap();
            if ch != ' ' {
                vec.push(ch);
            }
        }
    }

    map
}

fn mover_9000(stacks: &mut BTreeMap<&str, Vec<char>>, count: usize, src: &str, dst: &str) {
    let src_vec = stacks.get_mut(src).unwrap();
    let elements: Vec<_> = src_vec.drain((src_vec.len() - count)..).rev().collect();
    stacks.get_mut(dst).unwrap().extend(elements);
}

fn mover_9001(stacks: &mut BTreeMap<&str, Vec<char>>, count: usize, src: &str, dst: &str) {
    let src_vec = stacks.get_mut(src).unwrap();
    let elements: Vec<_> = src_vec.drain((src_vec.len() - count)..).collect();
    stacks.get_mut(dst).unwrap().extend(elements);
}

fn extract_message(stacks: &BTreeMap<&str, Vec<char>>) -> String {
    let chars = stacks.iter().filter_map(|vec| vec.1.last());
    String::from_iter(chars)
}
