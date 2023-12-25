#![cfg(test)]

use crate::{indices_of_correctly_ordered_packets, parse_list, Item, List, Number};
use anyhow::Result;
use aoc::read_puzzle_input;

#[test]
fn test_part1() -> Result<()> {
    let lines = read_puzzle_input("test_data/day13.txt")?;
    let packets: Vec<Vec<Item>> = lines.iter().filter_map(|s| parse_list(s).ok()).collect();
    let indices = indices_of_correctly_ordered_packets(&packets);
    assert_eq!(13usize, indices.iter().sum());
    Ok(())
}

#[test]
fn test_parse_list() {
    assert_eq!(Vec::<Item>::new(), parse_list("[]").unwrap());

    assert_eq!(vec![Number(9)], parse_list("[9]").unwrap());

    assert_eq!(
        vec![Number(1), Number(1), Number(3), Number(1), Number(1)],
        parse_list("[1,1,3,1,1]").unwrap()
    );

    assert_eq!(
        vec![
            List(vec![Number(1)]),
            List(vec![Number(2), Number(3), Number(4)])
        ],
        parse_list("[[1],[2,3,4]]").unwrap()
    );

    assert_eq!(
        vec![List(vec![Number(1)]), Number(4)],
        parse_list("[[1],4]").unwrap()
    );

    assert_eq!(
        vec![List(vec![List(vec![])])],
        parse_list("[[[]]]").unwrap()
    );

    assert_eq!(
        vec![
            Number(1),
            List(vec![
                Number(2),
                List(vec![
                    Number(3),
                    List(vec![Number(4), List(vec![Number(5), Number(6), Number(7)])])
                ])
            ]),
            Number(8),
            Number(9)
        ],
        parse_list("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap()
    );
}
