use std::collections::LinkedList;

use anyhow::{bail, Result};
use aoc::{read_puzzle_input, PuzzleCli};
use nalgebra::DMatrix;

type Position = (usize, usize);

fn main() -> Result<()> {
    let args = PuzzleCli::new("day12").parse_args();
    let lines = read_puzzle_input(args.input)?;

    let (nrows, ncols) = (lines.len(), lines.first().expect("first line exists").len());
    let grid = DMatrix::from_iterator(ncols, nrows, lines.join("").chars()).transpose();

    let (start, end) = start_and_end_positions(&grid)?;
    let steps = shortest_path(start, end, &grid).expect("shortest path exists");
    println!("Shortest path from S to E is {steps} steps");

    let min_steps = shortest_path_from_min_elevation(grid, end);
    println!("Shortest path from elevation a to E is {min_steps} steps");

    Ok(())
}

fn shortest_path_from_min_elevation(grid: DMatrix<char>, end: (usize, usize)) -> i32 {
    let min_elevation_positions = all_min_elevation_positions(&grid);
    let steps_vec = min_elevation_positions
        .into_iter()
        .filter_map(|start| shortest_path(start, end, &grid))
        .collect::<Vec<_>>();
    steps_vec.into_iter().min().expect("not empty")
}

fn all_min_elevation_positions(grid: &DMatrix<char>) -> Vec<Position> {
    let mut positions = Vec::new();
    for (nrow, row) in grid.row_iter().enumerate() {
        for (ncol, &each) in row.iter().enumerate() {
            if each == 'a' {
                positions.push((nrow, ncol));
            }
        }
    }
    positions
}

fn shortest_path(start: Position, end: Position, grid: &DMatrix<char>) -> Option<i32> {
    let mut queue = LinkedList::from([start]);
    let mut distance_grid = DMatrix::from_element(grid.nrows(), grid.ncols(), None);
    distance_grid[start] = Some(0);

    while !queue.is_empty() {
        let current_pos = queue.pop_front().expect("non empty");
        let current_dis = distance_grid[current_pos].expect("already calculated");
        let current_el = char_to_elevation(grid[current_pos]);

        for each in neighbors(current_pos, grid.nrows(), grid.ncols()) {
            if distance_grid[each].is_some() {
                continue;
            }

            if (0..=current_el + 1).contains(&char_to_elevation(grid[each])) {
                distance_grid[each] = Some(current_dis + 1);
                queue.push_back(each);
            }
        }
    }
    distance_grid[end]
}

fn neighbors(pos: Position, nrows: usize, ncols: usize) -> Vec<Position> {
    let offsets = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut neighbors = Vec::new();

    for o in offsets {
        let neighbor_row = pos.0.checked_add_signed(o.0);
        let neighbor_col = pos.1.checked_add_signed(o.1);
        if let Some((col, row)) = neighbor_col.zip(neighbor_row) {
            if col < ncols && row < nrows {
                neighbors.push((row, col));
            }
        }
    }
    neighbors
}

fn start_and_end_positions(grid: &DMatrix<char>) -> Result<(Position, Position)> {
    let (mut start, mut end) = (None, None);
    for (nrow, row) in grid.row_iter().enumerate() {
        for (ncol, each) in row.iter().enumerate() {
            match each {
                'S' => start = Some((nrow, ncol)),
                'E' => end = Some((nrow, ncol)),
                _ => continue,
            }
            let start = match start {
                Some(s) => s,
                None => continue,
            };
            let end = match end {
                Some(e) => e,
                None => continue,
            };
            return Ok((start, end));
        }
    }
    bail!("Failed to find S or E symbols");
}

fn char_to_elevation(c: char) -> u8 {
    let c = match c {
        'S' => 'a',
        'E' => 'z',
        _ => c,
    };
    *(c as u32).to_le_bytes().first().expect("always present")
}
