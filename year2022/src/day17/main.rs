use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use aoc::{read_puzzle_input, PuzzleCli};

const CHAMBER_WIDTH: usize = 7;

type Coordinates = (i32, i32);

#[derive(Clone, Debug)]
struct Rock {
    position: Coordinates,
    width: i32,
    top_parts: Vec<Coordinates>,
    bottom_parts: Vec<Coordinates>,
}

impl Rock {
    pub fn new(parts: impl IntoIterator<Item = Coordinates>) -> Rock {
        let mut y_min_max_map: HashMap<_, Coordinates> = HashMap::new();
        for (x, y) in parts {
            match y_min_max_map.get(&x) {
                Some(&(y_min, y_max)) => y_min_max_map.insert(x, (y_min.min(y), y_max.max(y))),
                None => y_min_max_map.insert(x, (y, y)),
            };
        }
        let width = *y_min_max_map.keys().max().expect("map should not be empty") + 1;
        let top_parts = y_min_max_map
            .iter()
            .map(|(&x, &(_, y_max))| (x, y_max))
            .collect();
        let bottom_parts = y_min_max_map
            .iter()
            .map(|(&x, &(y_min, _))| (x, y_min))
            .collect();
        Rock {
            position: (0, 0),
            width,
            top_parts,
            bottom_parts,
        }
    }

    pub fn bottom_parts(&self) -> &[Coordinates] {
        &self.bottom_parts
    }

    pub fn bottom_point(&self) -> Option<Coordinates> {
        self.bottom_parts.iter().min_by_key(|(_, y)| y).copied().map(|(x, y)| (self.position.0 + x, self.position.1 + y))
    }

    pub fn move_by(&mut self, offset: Coordinates) {
        let (p_x, p_y) = self.position;
        let (o_x, o_y) = offset;
        self.position = (
            (p_x + o_x).clamp(0, CHAMBER_WIDTH as i32 - self.width),
            (p_y + o_y),
        )
    }
}

fn main() -> Result<()> {
    let args = PuzzleCli::new("day17").parse_args();
    let lines = read_puzzle_input(args.input)?;
    let mut chars = lines
        .first()
        .expect("first line should exist")
        .chars()
        .cycle();

    let rocks = [
        Rock::new((0..4).map(|e| (e, 0))),
        Rock::new([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        Rock::new([(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)]),
        Rock::new((0..4).map(|e| (0, e))),
        Rock::new([(0, 0), (0, 1), (1, 0), (1, 1)]),
    ];
    let mut rocks_iter = rocks.into_iter().cycle();

    let mut floor_heights = [0; CHAMBER_WIDTH];

    for _ in 0..2022 {
        let mut rock = rocks_iter.next().expect("rocks_iter should never end");
        // let rock_bottom = rock.bottom_point().ok_or(anyhow!("bottom not found"))?;
        let (_, floor_top_y) = get_floor_top(&floor_heights);
        rock.move_by((2, floor_top_y + 4));

        for chr in chars.by_ref() {
            let offset_x = match chr {
                '>' => 1,
                '<' => -1,
                _ => bail!("unexpected char {chr}"),
            };
            rock.move_by((offset_x, -1));

            let (_, rock_bottom_y) = rock.bottom_point().expect("shoud exist");
            if rock_bottom_y == floor_top_y {
                // TODO: finish this crap
                break;
            }
        }
    }

    Ok(())
}

fn get_floor_top(floor_heights: &[i32]) -> Coordinates {
    floor_heights
        .iter()
        .enumerate()
        .max_by_key(|&(_, &y)| y)
        .map(|e| (e.0 as i32, *e.1))
        .expect("floor_heights should not be empty")
}
