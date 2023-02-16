use anyhow::{anyhow, Result};
use std::cmp::Ordering;
use std::fs;

#[derive(Eq, Ord, PartialEq, PartialOrd, Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    pub fn play_with(&self, other: &Shape) -> (u32, u32) {
        let points = match self.cmp(other) {
            _ if (self, other) == (&Shape::Rock, &Shape::Scissors) => (6, 0),
            _ if (self, other) == (&Shape::Scissors, &Shape::Rock) => (0, 6),
            Ordering::Equal => (3, 3),
            Ordering::Less => (0, 6),
            Ordering::Greater => (6, 0),
        };

        (points.0 + *self as u32, points.1 + *other as u32)
    }

    pub fn from_char(c: char) -> Result<Shape> {
        let shape = match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => return Err(anyhow!("Invalid character. Expected A, B, C, X, Y or Z")),
        };
        Ok(shape)
    }
}

fn main() -> Result<()> {
    let data = fs::read_to_string("data/day2.txt")?;

    let points = data
        .trim()
        .split('\n')
        .map(|s| s.chars())
        .map(|mut c| (c.next().unwrap(), c.nth(1).unwrap()))
        .map(|(c1, c2)| (Shape::from_char(c1).unwrap(), Shape::from_char(c2).unwrap()))
        .map(|(opponents, yours)| yours.play_with(&opponents).0)
        .sum::<u32>();

    println!("Points: {points}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Shape::*;

    #[test]
    fn test_rock() {
        assert_eq!(Rock.play_with(&Rock), (4, 4));
        assert_eq!(Rock.play_with(&Paper), (1, 8));
        assert_eq!(Rock.play_with(&Scissors), (7, 3));
    }

    #[test]
    fn test_paper() {
        assert_eq!(Paper.play_with(&Rock), (8, 1));
        assert_eq!(Paper.play_with(&Paper), (5, 5));
        assert_eq!(Paper.play_with(&Scissors), (2, 9));
    }

    #[test]
    fn test_scissors() {
        assert_eq!(Scissors.play_with(&Rock), (3, 7));
        assert_eq!(Scissors.play_with(&Paper), (9, 2));
        assert_eq!(Scissors.play_with(&Scissors), (6, 6));
    }
}
