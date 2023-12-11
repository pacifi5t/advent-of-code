use std::{collections::BTreeMap, fs, iter};

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let data = fs::read_to_string("data/day10.txt")?;
    let lines: Vec<&str> = data.split('\n').filter(|s| !s.is_empty()).collect();

    let mut current_cycle = 0;
    let mut x_reg = 1;
    let mut x_reg_history = BTreeMap::<i32, i32>::new();

    for line in lines {
        current_cycle += 1;
        x_reg_history.insert(current_cycle, x_reg);

        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens.first() {
            Some(&"noop") => {}
            Some(&"addx") => {
                current_cycle += 1;
                x_reg_history.insert(current_cycle, x_reg);
                x_reg += tokens[1].parse::<i32>()?;
            }
            _ => bail!("undefined instruction"),
        }
    }

    let sum: i32 = iter::successors(Some(20), |e| Some(e + 40))
        .take_while(|&e| e <= 220)
        .filter_map(|e| x_reg_history.get_key_value(&e))
        .map(|(cycle, x_reg)| cycle * x_reg)
        .sum();
    println!("Signal strength sum: {sum}\n");

    for row in 0..6 {
        for (cycle, x) in x_reg_history.iter().skip(row as usize * 40).take(40) {
            if (x - 1..=x + 1).contains(&(cycle - 1 - row * 40)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    Ok(())
}
