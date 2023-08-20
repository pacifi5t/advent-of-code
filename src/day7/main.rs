use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

use crate::file_tree::{Directory, File, Node};

mod file_tree;

static SMALL_DIR_SIZE: usize = 100_000;
static DISK_CAPACITY: usize = 70_000_000;
static SPACE_REQUIRED_FOR_UPDATE: usize = 30_000_000;

fn main() -> Result<()> {
    let data = fs::read_to_string("data/day7.txt")?;
    let lines: Vec<_> = data.split('\n').filter(|e| !e.is_empty()).collect();
    let (mut root, visited) = build_file_tree(&lines)?;

    let dirs_map: HashMap<PathBuf, usize> = visited
        .into_iter()
        .map(|path| {
            let size = root.get_child_dir(&path).expect("exists").size();
            (path, size)
        })
        .collect();

    let total_size = dirs_map
        .values()
        .filter(|&e| *e <= SMALL_DIR_SIZE)
        .sum::<usize>();
    println!("Total size of directories with size smaller then {SMALL_DIR_SIZE}: {total_size}");

    let space_to_free = root.size() - (DISK_CAPACITY - SPACE_REQUIRED_FOR_UPDATE);
    let smallest_file_size = dirs_map
        .into_iter()
        .filter(|e| e.1 > space_to_free)
        .min_by_key(|e| e.1)
        .expect("not empty")
        .1;
    println!("Size of the smallest file to delete: {smallest_file_size}");
    Ok(())
}

fn build_file_tree(lines: &[&str]) -> Result<(Directory, HashSet<PathBuf>)> {
    let mut root = Directory::new(OsString::from("/"), HashMap::new());
    let mut current_dir = PathBuf::from("");
    let mut temp_elems = HashMap::<OsString, Box<dyn Node>>::new();
    let mut visited = HashSet::new();

    for line in lines {
        let mut tokens = line.split_whitespace();
        match tokens.next().unwrap() {
            "$" => match tokens.next().unwrap() {
                "cd" => {
                    save(&mut root, &current_dir, &mut temp_elems)?;
                    match tokens.next().unwrap() {
                        ".." => {
                            current_dir.pop();
                        }
                        dir => current_dir.push(dir),
                    }
                    visited.insert(current_dir.clone());
                }
                "ls" => {}
                _ => bail!("Undefined command"),
            },
            "dir" => {
                let name = OsString::from(tokens.next().unwrap());
                temp_elems.insert(name.clone(), Box::new(Directory::new(name, HashMap::new())));
            }
            size => {
                let name = OsString::from(tokens.next().unwrap());
                temp_elems.insert(
                    name.clone(),
                    Box::new(File::new(name, size.parse::<usize>()?)),
                );
            }
        }
    }

    save(&mut root, &current_dir, &mut temp_elems)?;
    Ok((root, visited))
}

fn save(
    root: &mut Directory,
    current_dir: &Path,
    temp_elems: &mut HashMap<OsString, Box<dyn Node>>,
) -> Result<()> {
    if !temp_elems.is_empty() {
        root.get_child_dir(current_dir)?
            .contents()
            .extend(temp_elems.drain());
    }
    Ok(())
}
