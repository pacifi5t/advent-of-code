mod test;

use crate::Item::{List, Number};
use anyhow::{bail, Result};
use aoc::{read_puzzle_input, PuzzleCli};
use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
enum Item {
    Number(i32),
    List(Vec<Item>),
}

fn main() -> Result<()> {
    let args = PuzzleCli::new("day13").parse_args();
    let lines = read_puzzle_input(args.input)?;

    let packets: Vec<Vec<Item>> = lines.iter().filter_map(|s| parse_list(s).ok()).collect();
    let indices = indices_of_correctly_ordered_packets(&packets);
    println!("Indices sum: {}", indices.iter().sum::<usize>());

    let mut flagged_packets = Vec::from_iter(packets.into_iter().map(|e| (false, e)));
    let (div1, div2) = (parse_list("[[2]]")?, parse_list("[[6]]")?);
    flagged_packets.extend([(true, div1), (true, div2)]);
    flagged_packets.sort_by(|a, b| lists_ordering(&a.1, &b.1));

    let mut flag_indices = Vec::new();
    for (i, &(flagged, _)) in flagged_packets.iter().enumerate() {
        if flagged {
            flag_indices.push(i + 1);
        }

        if flag_indices.len() == 2 {
            break;
        }
    }

    println!(
        "Divider package indices product: {}",
        flag_indices[0] * flag_indices[1]
    );
    Ok(())
}

fn indices_of_correctly_ordered_packets(packets: &[Vec<Item>]) -> Vec<usize> {
    let mut indices = Vec::new();
    for (i, pair) in packets.chunks_exact(2).enumerate() {
        if lists_ordering(&pair[0], &pair[1]) == Ordering::Less {
            indices.push(i + 1);
        }
    }
    indices
}

fn lists_ordering(left: &[Item], right: &[Item]) -> Ordering {
    for (l, r) in left.iter().zip(right) {
        let ordering = match l {
            List(l_lst) => match r {
                List(r_lst) => lists_ordering(l_lst, r_lst),
                Number(r_num) => lists_ordering(l_lst, &[Number(*r_num)]),
            },
            Number(l_num) => match r {
                List(r_lst) => lists_ordering(&[Number(*l_num)], r_lst),
                Number(r_num) => l_num.cmp(r_num),
            },
        };

        if ordering != Ordering::Equal {
            return ordering;
        }
    }

    left.len().cmp(&right.len())
}

fn parse_list(s: &str) -> Result<Vec<Item>> {
    let (index, items) = parse_nested_list(s, 0)?;
    if index < s.len() - 1 {
        bail!("Malformed string: too short")
    } else {
        Ok(items)
    }
}

fn parse_nested_list(s: &str, index: usize) -> Result<(usize, Vec<Item>)> {
    let mut items = Vec::new();
    let mut buf = String::new();
    let mut i = index + 1;
    while (index..s.len()).contains(&i) {
        let ch = s.chars().nth(i).expect("always present in string");
        match ch {
            ',' => match get_previous_char(s, i)? {
                ']' => {}
                _ => {
                    items.push(Number(buf.parse()?));
                    buf.clear();
                }
            },
            '[' => {
                let (index, list) = parse_nested_list(s, i)?;
                i = index;
                items.push(List(list));
            }
            ']' => {
                return match get_previous_char(s, i)? {
                    '[' | ']' => Ok((i, items)),
                    _ => {
                        items.push(Number(buf.parse()?));
                        Ok((i, items))
                    }
                }
            }
            _ => match ch.is_ascii_digit() {
                true => buf.push(ch),
                false => bail!("Malformed string: unknown character at {i}"),
            },
        }
        i += 1;
    }
    Ok((i, items))
}

fn get_previous_char(s: &str, index: usize) -> Result<char> {
    let ch = s.chars().nth(index - 1);
    match ch {
        Some(ch) => Ok(ch),
        None => bail!("Malformed string: unexpected ',' or ']' at {index}"),
    }
}
