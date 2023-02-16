use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let data = fs::read_to_string("data/day1.txt")?;

    let max = data
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .filter_map(|e| e.parse::<u32>().ok())
                .sum::<u32>()
        })
        .max_by(|a, b| a.cmp(b))
        .expect("should be fine after filtering");

    println!("Max: {max}");
    Ok(())
}
