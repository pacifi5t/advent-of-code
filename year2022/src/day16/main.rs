mod test;

use anyhow::{anyhow, bail, Result};
use aoc::{read_puzzle_input, PuzzleCli};
use regex::Regex;

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    leads_to: Vec<String>,
}

fn main() -> Result<()> {
    let args = PuzzleCli::new("day16").parse_args();
    let lines = read_puzzle_input(args.input)?;
    let valves = parse_input(&lines)?;

    for each in valves {
        println!("{:?}", each);
    }

    Ok(())
}

fn parse_input(lines: &[String]) -> Result<Vec<Valve>> {
    let regex = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)")?;
    let mut valves = Vec::<Valve>::new();
    for line in lines {
        let captures = regex.captures(line).expect("regex must match the line");
        let (_, [name, flow_rate, leads_to]) = captures.extract();
        valves.push(Valve {
            name: name.to_string(),
            flow_rate: flow_rate.parse()?,
            leads_to: leads_to.split(", ").map(str::to_string).collect(),
        });
    }
    Ok(valves)
}
