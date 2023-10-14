use std::{collections::HashSet, fs};

use anyhow::{bail, Result};

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance_from(self, other: Point) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }
}

fn main() -> Result<()> {
    let data = fs::read_to_string("data/day9.txt")?;
    let lines: Vec<&str> = data.split('\n').filter(|s| !s.is_empty()).collect();

    let (mut head, mut tail) = (Point::default(), Point::default());
    let mut visited = HashSet::<Point>::from_iter(vec![Point::default()]);

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

            let shift = calc_tail_shift(head, tail);
            tail.x += shift.0;
            tail.y += shift.1;
            visited.insert(tail);
        }
    }

    println!("{}", visited.len());

    Ok(())
}

pub fn calc_tail_shift(head: Point, tail: Point) -> (i32, i32) {
    let (x, y) = head.distance_from(tail);

    if x.abs() == 2 {
        match y.abs() {
            0 => return (x / 2, 0),
            1 => return (x / 2, y),
            _ => panic!(),
        }
    }

    if y.abs() == 2 {
        match x.abs() {
            0 => return (0, y / 2),
            1 => return (x, y / 2),
            _ => panic!(),
        }
    }

    (0, 0)
}

mod test {
    use crate::{calc_tail_shift, Point};

    #[test]
    fn test_tail_chift_calc() {
        assert_eq!(calc_tail_shift(Point::new(2, 0), Point::new(0, 0)), (1, 0));
        assert_eq!(calc_tail_shift(Point::new(2, 1), Point::new(0, 0)), (1, 1));
        assert_eq!(
            calc_tail_shift(Point::new(2, -1), Point::new(0, 0)),
            (1, -1)
        );

        assert_eq!(
            calc_tail_shift(Point::new(-2, 0), Point::new(0, 0)),
            (-1, 0)
        );
        assert_eq!(
            calc_tail_shift(Point::new(-2, 1), Point::new(0, 0)),
            (-1, 1)
        );
        assert_eq!(
            calc_tail_shift(Point::new(-2, -1), Point::new(0, 0)),
            (-1, -1)
        );

        assert_eq!(calc_tail_shift(Point::new(0, 2), Point::new(0, 0)), (0, 1));
        assert_eq!(calc_tail_shift(Point::new(1, 2), Point::new(0, 0)), (1, 1));
        assert_eq!(
            calc_tail_shift(Point::new(-1, 2), Point::new(0, 0)),
            (-1, 1)
        );

        assert_eq!(
            calc_tail_shift(Point::new(0, -2), Point::new(0, 0)),
            (0, -1)
        );
        assert_eq!(
            calc_tail_shift(Point::new(1, -2), Point::new(0, 0)),
            (1, -1)
        );
        assert_eq!(
            calc_tail_shift(Point::new(-1, -2), Point::new(0, 0)),
            (-1, -1)
        );
    }
}
