use anyhow::{anyhow, Result};
use std::fs;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    pub fn play_with(&self, other: &Shape) -> (u32, u32) {
        let points = match self {
            _ if self == other => (3, 3),
            _ if self == &other.stronger() => (6, 0),
            _ => (0, 6),
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

    pub fn stronger(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
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
