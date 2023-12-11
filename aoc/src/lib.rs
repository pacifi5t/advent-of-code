use std::{fs, io, path::Path};

use clap::{Arg, Command};

pub fn read_puzzle_input<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let string = fs::read_to_string(path)?;
    let lines: Vec<String> = string
        .split('\n')
        .map(|e| e.trim().to_owned())
        .filter(|s| !s.is_empty())
        .collect();
    Ok(lines)
}

pub struct PuzzleCli {
    command: Command,
}

impl PuzzleCli {
    pub fn new(name: &str) -> Self {
        let command = Command::new(name.to_owned()).arg(Arg::new("input").required(true));
        Self { command }
    }

    pub fn parse_args(self) -> PuzzleArgs {
        let matches = self.command.get_matches();
        let input = matches
            .get_one::<String>("input")
            .expect("required")
            .to_owned();
        PuzzleArgs { input }
    }
}

pub struct PuzzleArgs {
    pub input: String,
}
