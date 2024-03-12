#![cfg(test)]

use crate::{cols_without_beacon_in_row, find_distress_beacon_position, parse_data};
use anyhow::Result;
use aoc::read_puzzle_input;

const TARGET_ROW: i32 = 10;
const BOUNDARIES: (i32, i32) = (0, 20);

#[test]
fn part1() -> Result<()> {
    let lines = read_puzzle_input("test_data/day15.txt")?;
    let sensors = parse_data(&lines)?;
    let cols = cols_without_beacon_in_row(TARGET_ROW, &sensors);
    assert_eq!(26, cols);
    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let lines = read_puzzle_input("test_data/day15.txt")?;
    let sensors = parse_data(&lines)?;
    let position = find_distress_beacon_position(BOUNDARIES, &sensors)?;
    assert_eq!((14, 11), position);
    Ok(())
}
