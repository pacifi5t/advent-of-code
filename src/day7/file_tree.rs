use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::path::Path;

use anyhow::{bail, Result};

pub trait FileTreeElem {
    fn size(&self) -> usize;

    fn name(&self) -> &OsStr;

    fn get_child_dir(&mut self, path: &Path) -> Result<&mut Directory>;
}

pub struct Directory {
    name: OsString,
    contents: HashMap<OsString, Box<dyn FileTreeElem>>,
}

impl Directory {
    pub fn new(name: OsString, contents: HashMap<OsString, Box<dyn FileTreeElem>>) -> Self {
        Self { name, contents }
    }

    pub fn contents(&mut self) -> &mut HashMap<OsString, Box<dyn FileTreeElem>> {
        &mut self.contents
    }
}

impl FileTreeElem for Directory {
    fn size(&self) -> usize {
        self.contents.values().map(|e| e.size()).sum()
    }

    fn name(&self) -> &OsStr {
        &self.name
    }

    fn get_child_dir(&mut self, path: &Path) -> Result<&mut Directory> {
        if path.as_os_str() == self.name() {
            return Ok(self);
        }

        let child_name = path.iter().nth(1).expect("should exist");
        let name = self.name().to_owned();

        match self.contents.get_mut(child_name) {
            None => bail!("Directory not found!"),
            Some(child) => {
                let path = path.strip_prefix(&name)?;
                Ok(child.get_child_dir(path)?)
            }
        }
    }
}

pub struct File {
    name: OsString,
    size: usize,
}

impl File {
    pub fn new(name: OsString, size: usize) -> Self {
        Self { name, size }
    }
}

impl FileTreeElem for File {
    fn size(&self) -> usize {
        self.size
    }

    fn name(&self) -> &OsStr {
        &self.name
    }

    fn get_child_dir(&mut self, _path: &Path) -> Result<&mut Directory> {
        bail!("Not implemented!");
    }
}
