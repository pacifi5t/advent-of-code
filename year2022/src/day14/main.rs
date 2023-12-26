use std::fmt::{Display, Formatter};

use anyhow::Result;
use nalgebra::DMatrix;

use aoc::{read_puzzle_input, PuzzleCli};

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Air,
    Sand,
    Rock,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Tile::Air => '.',
            Tile::Rock => '#',
            Tile::Sand => 'O',
        };
        write!(f, "{ch}")
    }
}

type Coordinates = (usize, usize);

const SAND_POURING_START: Coordinates = (0, 500);

fn main() -> Result<()> {
    let args = PuzzleCli::new("day14").parse_args();
    let lines = read_puzzle_input(args.input)?;
    let (rock_coords, max_coords, min_coords) = parse_rock_coords(&lines)?;
    let start = (
        SAND_POURING_START.0 - min_coords.0,
        SAND_POURING_START.1 - min_coords.1 + max_coords.0,
    );

    let mut grid = create_grid(&rock_coords, &max_coords, &min_coords);
    let nrows = grid.nrows();
    let sand_poured = pour_sand(&mut grid, start, |sand_co| sand_co.0 != nrows - 1) - 1;
    println!("Sand tiles poured before falling to abyss: {sand_poured}");

    let mut grid = create_grid(&rock_coords, &max_coords, &min_coords);
    grid = grid.insert_row(nrows, Tile::Air);
    grid = grid.insert_row(nrows + 1, Tile::Rock);
    let sand_poured = pour_sand(&mut grid, start, |sand_co| sand_co != start);
    println!("Sand tiles poured to fill the cave: {sand_poured}");

    Ok(())
}

fn pour_sand<F>(grid: &mut DMatrix<Tile>, start: Coordinates, predicate: F) -> usize
where
    F: Fn(Coordinates) -> bool,
{
    let mut sand_tiles_poured = 0;
    'pouring_sand: loop {
        let mut sand_coords = start;
        'sand_tile_fall: loop {
            let tiles_with_air: Vec<Coordinates> = tiles_underneath(sand_coords, grid)
                .into_iter()
                .filter(|&e| grid[e] == Tile::Air)
                .collect();

            if let Some(c) = tiles_with_air.first() {
                sand_coords = *c;
            } else {
                break 'sand_tile_fall;
            }
        }

        sand_tiles_poured += 1;
        grid[sand_coords] = Tile::Sand;

        if !predicate(sand_coords) {
            break 'pouring_sand;
        }
    }
    sand_tiles_poured
}

fn tiles_underneath(coords: Coordinates, grid: &DMatrix<Tile>) -> Vec<Coordinates> {
    let offsets = [(1, 0), (1, -1), (1, 1)];
    let mut tiles = Vec::new();

    for o in offsets {
        let neighbor_row = coords.0.checked_add_signed(o.0);
        let neighbor_col = coords.1.checked_add_signed(o.1);
        if let Some((col, row)) = neighbor_col.zip(neighbor_row) {
            if col < grid.ncols() && row < grid.nrows() {
                tiles.push((row, col));
            }
        }
    }
    tiles
}

fn create_grid(
    rocks: &[Vec<Coordinates>],
    max_coords: &Coordinates,
    min_coords: &Coordinates,
) -> DMatrix<Tile> {
    let mut grid = DMatrix::from_element(
        1 + max_coords.0 - min_coords.0,
        1 + max_coords.1 - min_coords.1 + max_coords.0 * 2,
        Tile::Air,
    );
    for path in rocks {
        for points in path.windows(2) {
            let points = [
                (points[0].0.min(points[1].0), points[0].1.min(points[1].1)),
                (points[0].0.max(points[1].0), points[0].1.max(points[1].1)),
            ];
            for row in points[0].0..=points[1].0 {
                for col in points[0].1..=points[1].1 {
                    let each = (row - min_coords.0, col - min_coords.1 + max_coords.0);
                    grid[each] = Tile::Rock;
                }
            }
        }
    }
    grid
}

fn parse_rock_coords(
    lines: &[String],
) -> Result<(Vec<Vec<Coordinates>>, Coordinates, Coordinates)> {
    let (mut min_coords, mut max_coords) = ((0, usize::MAX), (0, 0));
    let mut rock_paths: Vec<Vec<Coordinates>> = vec![];

    for (i, line) in lines.iter().enumerate() {
        rock_paths.push(vec![]);
        for point in line.split(" -> ") {
            let coords: Vec<&str> = point.split(',').collect();
            let c = (coords[1].parse::<usize>()?, coords[0].parse::<usize>()?); // Reverse x and y
            rock_paths[i].push(c);

            if min_coords.1 > c.1 {
                min_coords.1 = c.1;
            }
            if max_coords.0 < c.0 {
                max_coords.0 = c.0;
            }
            if max_coords.1 < c.1 {
                max_coords.1 = c.1;
            }
        }
    }
    Ok((rock_paths, max_coords, min_coords))
}
