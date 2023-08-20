use std::cmp::Ordering;
use std::fs;

use anyhow::Result;
use nalgebra::DMatrix;

fn main() -> Result<()> {
    let data = fs::read_to_string("data/day8.txt")?;
    let lines: Vec<&str> = data.split('\n').filter(|s| !s.is_empty()).collect();
    let grid = DMatrix::from_iterator(
        lines.len(),
        lines.first().expect("first line exists").len(),
        data.chars().filter(|c| *c != '\n').map(to_digit),
    )
    .transpose();
    println!("{grid}");

    let visible_trees = enumerate_visible_trees(&grid);
    println!("Trees visible outside the grid: {visible_trees}");

    let scenic_score = highest_scenic_score(&grid);
    println!("Highest scenic score: {scenic_score}");

    Ok(())
}

fn enumerate_visible_trees(grid: &DMatrix<u32>) -> usize {
    let mut visible = grid.ncols() * 2 + grid.nrows() * 2 - 4;

    for x in 1..(grid.ncols() - 1) {
        for y in 1..(grid.nrows() - 1) {
            let (col, row) = (grid.column(x), grid.row(y));
            let tree = *grid.index((y, x));

            let up = col.iter().take(y).all(|&t| t < tree);
            let down = col.iter().skip(y + 1).all(|&t| t < tree);
            let left = row.iter().take(x).all(|&t| t < tree);
            let right = row.iter().skip(x + 1).all(|&t| t < tree);
            visible += usize::from(up || down || left || right);
        }
    }

    visible
}

fn highest_scenic_score(grid: &DMatrix<u32>) -> usize {
    let mut scenic_scores = vec![];

    for x in 1..(grid.ncols() - 1) {
        for y in 1..(grid.nrows() - 1) {
            let (col, row) = (grid.column(x), grid.row(y));
            let tree = *grid.index((y, x));

            let up = look_for_trees_from(tree, col.iter().take(y).rev());
            let down = look_for_trees_from(tree, col.iter().skip(y + 1));
            let left = look_for_trees_from(tree, row.iter().take(x).rev());
            let right = look_for_trees_from(tree, row.iter().skip(x + 1));
            scenic_scores.push(up * down * left * right);
        }
    }

    *scenic_scores.iter().max().expect("not empty")
}

fn to_digit(c: char) -> u32 {
    c.to_digit(10).expect("definitely digit")
}

fn look_for_trees_from<'a>(tree: u32, iter: impl Iterator<Item = &'a u32>) -> usize {
    let mut trees = 0;
    for t in iter {
        match t.cmp(&tree) {
            Ordering::Less => trees += 1,
            Ordering::Equal | Ordering::Greater => {
                trees += 1;
                break;
            }
        }
    }
    trees
}
