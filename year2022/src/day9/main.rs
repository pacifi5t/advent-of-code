use anyhow::Error;
use std::{collections::HashSet, fs};

use anyhow::{bail, Result};

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance_from(self, other: Position) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }

    pub fn shift(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

fn main() -> Result<()> {
    let data = fs::read_to_string("data/day9.txt")?;
    let lines: Vec<&str> = data.split('\n').filter(|s| !s.is_empty()).collect();

    simulate_2_knots(&lines)?;
    simulate_10_knots(&lines)?;

    Ok(())
}

fn simulate_2_knots(lines: &[&str]) -> Result<(), Error> {
    let (mut head, mut tail) = (Position::default(), Position::default());
    let mut visited = HashSet::<Position>::from_iter(vec![Position::default()]);

    for line in lines {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let (direction, steps) = (tokens[0], tokens[1].parse::<i32>()?);

        for _ in 0..steps {
            match direction {
                "U" => head.y += 1,
                "D" => head.y -= 1,
                "R" => head.x += 1,
                "L" => head.x -= 1,
                _ => bail!("Undefined direction"),
            }

            let shift = pick_knot_shift(head, tail);
            tail.shift(shift.0, shift.1);
            visited.insert(tail);
        }
    }
    println!("2 knots, tail visited {} positions", visited.len());
    Ok(())
}

fn simulate_10_knots(lines: &[&str]) -> Result<(), Error> {
    let mut knots = [Position::default(); 10];
    let mut visited = HashSet::<Position>::from_iter(vec![Position::default()]);

    for line in lines {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let (direction, steps) = (tokens[0], tokens[1].parse::<i32>()?);

        for _ in 0..steps {
            let head = knots.first_mut().unwrap();
            match direction {
                "U" => head.y += 1,
                "D" => head.y -= 1,
                "R" => head.x += 1,
                "L" => head.x -= 1,
                _ => bail!("Undefined direction"),
            }

            for i in 1..knots.len() {
                let prev = knots[i - 1];
                let current = knots.get_mut(i).unwrap();
                let shift = pick_knot_shift(prev, *current);
                current.shift(shift.0, shift.1);
            }

            visited.insert(*knots.last().unwrap());
        }
    }

    println!("10 knots, tail visited {} positions", visited.len());
    Ok(())
}

pub fn pick_knot_shift(head: Position, tail: Position) -> (i32, i32) {
    let (x, y) = head.distance_from(tail);
    match (x.abs(), y.abs()) {
        (2, 0) => (x / 2, 0),
        (2, 1) => (x / 2, y),
        (0, 2) => (0, y / 2),
        (1, 2) => (x, y / 2),
        (2, 2) => (x / 2, y / 2),
        _ => (0, 0),
    }
}

#[cfg(test)]
mod test {
    use crate::{pick_knot_shift, Position};

    #[test]
    fn test_tail_shift_calculation() {
        let tail = Position::default();
        assert_eq!(pick_knot_shift(Position::new(2, 0), tail), (1, 0));
        assert_eq!(pick_knot_shift(Position::new(2, 1), tail), (1, 1));
        assert_eq!(pick_knot_shift(Position::new(2, -1), tail), (1, -1));

        assert_eq!(pick_knot_shift(Position::new(-2, 0), tail), (-1, 0));
        assert_eq!(pick_knot_shift(Position::new(-2, 1), tail), (-1, 1));
        assert_eq!(pick_knot_shift(Position::new(-2, -1), tail), (-1, -1));

        assert_eq!(pick_knot_shift(Position::new(0, 2), tail), (0, 1));
        assert_eq!(pick_knot_shift(Position::new(1, 2), tail), (1, 1));
        assert_eq!(pick_knot_shift(Position::new(-1, 2), tail), (-1, 1));

        assert_eq!(pick_knot_shift(Position::new(0, -2), tail), (0, -1));
        assert_eq!(pick_knot_shift(Position::new(1, -2), tail), (1, -1));
        assert_eq!(pick_knot_shift(Position::new(-1, -2), tail), (-1, -1));
    }
}
