mod test;

use anyhow::{bail, Result};
use aoc::{read_puzzle_input, PuzzleCli};
use regex::Regex;
use std::collections::HashSet;
use std::ops::{Add, Sub};

type Coordinates = (i32, i32);

const TARGET_ROW: i32 = 2_000_000;
const BOUNDARIES: (i32, i32) = (0, 4_000_000);

struct Sensor {
    coords: Coordinates,
    closest_beacon: Coordinates,
    distance: u32,
}

fn main() -> Result<()> {
    let args = PuzzleCli::new("day15").parse_args();
    let lines = read_puzzle_input(args.input)?;
    let sensors = parse_data(&lines)?;
    let cols = cols_without_beacon_in_row(TARGET_ROW, &sensors);
    println!("{cols} positions where beacon cannot be present in row {TARGET_ROW}");

    let pos = find_distress_beacon_position(BOUNDARIES, &sensors)?;
    println!("Distress beacon position: x={}, y={}", pos.0, pos.1);

    Ok(())
}

fn cols_without_beacon_in_row(row: i32, sensors: &[Sensor]) -> usize {
    let cols_with_beacons: Vec<i32> = sensors
        .iter()
        .filter(|e| e.closest_beacon.1 == row)
        .map(|e| e.closest_beacon.0)
        .collect();
    let mut cols_without_beacons = HashSet::new();

    for sensor in sensors {
        let target_row_distance = manhattan_distance(sensor.coords, (sensor.coords.0, row));

        if target_row_distance > sensor.distance {
            continue;
        }

        let res_distance = (sensor.distance - target_row_distance) as i32;
        for col in sensor.coords.0 - res_distance..=sensor.coords.0 + res_distance {
            if !cols_with_beacons.contains(&col) {
                cols_without_beacons.insert(col);
            }
        }
    }

    cols_without_beacons.len()
}

fn find_distress_beacon_position(
    boundaries: (i32, i32),
    sensors: &[Sensor],
) -> Result<Coordinates> {
    for (i, s) in sensors.iter().enumerate() {
        let edge_distance = (s.distance + 1) as i32;
        let mut coords_to_check = Vec::new();
        let rows_range = s.coords.1.sub(edge_distance).max(boundaries.0)
            ..=s.coords.1.add(edge_distance).min(boundaries.1);

        for row in rows_range {
            let res_distance =
                edge_distance - manhattan_distance(s.coords, (s.coords.0, row)) as i32;
            let cols = [s.coords.0 - res_distance, s.coords.0 + res_distance];
            for col in cols {
                if col < boundaries.0 || col > boundaries.1 {
                    continue;
                }
                coords_to_check.push((col, row));
            }
        }

        'outer: for c in &coords_to_check {
            for (j, e) in sensors.iter().enumerate() {
                if i == j {
                    continue;
                }

                if manhattan_distance(e.coords, *c) <= e.distance {
                    continue 'outer;
                }
            }
            return Ok(*c);
        }
    }

    bail!("Can't find distress beacon");
}

fn parse_data(lines: &[String]) -> Result<Vec<Sensor>> {
    let re = Regex::new(r"-?\d+")?;
    let mut sensors = vec![];
    for (i, line) in lines.iter().enumerate() {
        let items: Vec<_> = re
            .find_iter(line)
            .filter_map(|m| m.as_str().parse::<i32>().ok())
            .collect();

        if items.len() != 4 {
            bail!("parsing error on line {i}");
        }

        let (sensor, beacon) = ((items[0], items[1]), (items[2], items[3]));
        sensors.push(Sensor {
            coords: sensor,
            closest_beacon: beacon,
            distance: manhattan_distance(sensor, beacon),
        });
    }
    Ok(sensors)
}

fn manhattan_distance(a: Coordinates, b: Coordinates) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}
