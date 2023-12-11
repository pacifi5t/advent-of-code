use anyhow::Result;
use std::collections::HashSet;
use std::fs;

fn main() -> Result<()> {
    let data: Vec<char> = fs::read_to_string("data/day6.txt")?.chars().collect();

    let (chars, seq) = find_sequence_of_unique_chars(&data, 4).unwrap();
    println!("Packet marker: {chars} chars, \"{seq}\"");

    let (chars, seq) = find_sequence_of_unique_chars(&data, 14).unwrap();
    println!("Message marker: {chars} chars, \"{seq}\"");

    Ok(())
}

fn find_sequence_of_unique_chars(data: &[char], len: usize) -> Option<(usize, String)> {
    for (i, each) in data.windows(len).enumerate() {
        let set: HashSet<_> = each.iter().collect();
        if set.len() == len {
            return Some((i + len, String::from_iter(each)));
        }
    }

    None
}
