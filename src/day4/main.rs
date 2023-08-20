use std::fs;
use std::ops::Range;

use anyhow::Result;

fn main() -> Result<()> {
    let data = fs::read_to_string("data/day4.txt")?;
    let pairs = data.trim().split('\n');
    let range_tuples: Vec<_> = pairs
        .map(|pair| {
            let mut ranges = pair.split(',').map(|rng| {
                let mut bounds = rng.split('-').filter_map(|e| e.parse::<i32>().ok());
                bounds.next().unwrap()..bounds.next().unwrap()
            });
            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .collect();

    let fully_contains = range_tuples
        .iter()
        .map(|t| i32::from(do_fully_contain(&t.0, &t.1)))
        .sum::<i32>();
    println!("Fully contains: {fully_contains}");

    let overlaps = range_tuples
        .iter()
        .map(|t| i32::from(do_overlap(&t.0, &t.1)))
        .sum::<i32>();
    println!("Overlaps: {overlaps}");

    Ok(())
}

fn do_fully_contain<T: Ord>(a: &Range<T>, b: &Range<T>) -> bool {
    a.start >= b.start && a.end <= b.end || a.start <= b.start && a.end >= b.end
}

fn do_overlap(a: &Range<i32>, b: &Range<i32>) -> bool {
    a.start >= b.start && a.start <= b.end || b.start >= a.start && b.start <= a.end
}

#[cfg(test)]
mod day4_tests {
    use crate::do_overlap;

    #[test]
    fn test_do_overlap() {
        assert!(do_overlap(&(5..7), &(7..9)));
        assert!(do_overlap(&(2..8), &(3..7)));
        assert!(do_overlap(&(6..6), &(4..6)));
        assert!(do_overlap(&(2..6), &(4..8)));

        assert!(do_overlap(&(3..7), &(2..8)));

        assert!(!do_overlap(&(3..6), &(8..10)));
    }
}
