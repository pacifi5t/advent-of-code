use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs;

fn main() -> Result<()> {
    let data = fs::read_to_string("data/day3.txt")?;
    let elves = data.trim().split('\n');

    let sets = elves.clone().map(|e| {
        let (s1, s2) = e.split_at(e.len() / 2);
        s1.chars()
            .filter(|c| s2.contains(*c))
            .collect::<HashSet<char>>()
    });

    let sum = sets
        .map(|s| {
            s.into_iter()
                .filter_map(|c| char_to_priority(c).ok())
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("Item priority sum: {sum}");

    let mut sum = 0;
    let mut elves = elves.map(|s| s.chars().collect::<HashSet<char>>());
    while let Some(elf1) = elves.next() {
        // All groups are full so those elves exist
        let (elf2, elf3) = (elves.next().unwrap(), elves.next().unwrap());
        let badge = elf1
            .intersection(&elf2)
            .find(|c| elf3.contains(c))
            .expect("All elves have a badge");
        sum += char_to_priority(*badge).unwrap();
    }
    println!("Badge priority sum: {sum}");

    Ok(())
}

fn char_to_priority(c: char) -> Result<u32> {
    match c {
        'a'..='z' => Ok(c as u32 - 96),
        'A'..='Z' => Ok(c as u32 - 38),
        _ => Err(anyhow!("Invalid char. Expected a-z or A-Z.")),
    }
}

#[cfg(test)]
mod day3_tests {
    use crate::char_to_priority;

    #[test]
    fn test_char_to_priority() {
        assert_eq!(char_to_priority('p').unwrap_or_default(), 16);
        assert_eq!(char_to_priority('L').unwrap_or_default(), 38);
        assert_eq!(char_to_priority('P').unwrap_or_default(), 42);
        assert_eq!(char_to_priority('v').unwrap_or_default(), 22);
        assert_eq!(char_to_priority('t').unwrap_or_default(), 20);
        assert_eq!(char_to_priority('s').unwrap_or_default(), 19);

        // unwrap_or_default call on Err will return 0
        assert_eq!(char_to_priority('1').unwrap_or_default(), 0);
        assert_eq!(char_to_priority('#').unwrap_or_default(), 0);
    }
}
