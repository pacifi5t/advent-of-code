use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let data = fs::read_to_string("data/day1.txt")?;

    let mut elves = data
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .filter_map(|e| e.parse::<u32>().ok())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    elves.sort_by(|a, b| b.cmp(a));

    let top3_sum = elves.iter().take(3).sum::<u32>();
    println!("Top 3: {top3_sum}");
    Ok(())
}
