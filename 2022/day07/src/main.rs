use std::{
    iter::Peekable,
    path::PathBuf,
    str::{FromStr, Lines},
};

use anyhow::Result;

#[derive(Debug)]
enum CdSpecial {
    Root,
    Back,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    directories: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {
    fn new_empty(name: String) -> Self {
        Self {
            name,
            directories: vec![],
            files: vec![],
        }
    }
    fn new_root() -> Self {
        Self::new_empty("root".to_string())
    }
    fn cd(&mut self, cd_dir_name: &str, lines: &mut Peekable<Lines>) -> Option<CdSpecial> {
        match cd_dir_name {
            ".." => Some(CdSpecial::Back),
            "/" => Some(CdSpecial::Root),
            _ => {
                let child_dir = self
                    .directories
                    .iter_mut()
                    .find(|dir| dir.name == cd_dir_name)
                    .expect("cannot find child dir with name");
                child_dir.from_lines(lines)
            }
        }
    }
    fn ls(&mut self, lines: &mut Peekable<Lines>) {
        while let Some(line) = lines.peek() {
            match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                ["dir", dir_name] => self.directories.push(Self::new_empty(dir_name.to_string())),
                [size, file_name] => self.files.push(File::new(
                    file_name.to_string(),
                    size.parse().expect("size must be numeric"),
                )),
                _ => return,
            }
            lines.next();
        }
    }
    fn from_lines(&mut self, lines: &mut Peekable<Lines>) -> Option<CdSpecial> {
        while let Some(line) = lines.next() {
            match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                ["$", "cd", cd_dir_name @ _] => match self.cd(cd_dir_name, lines) {
                    Some(CdSpecial::Back) => return None,
                    Some(CdSpecial::Root) => return Some(CdSpecial::Root),
                    _ => (),
                },
                ["$", "ls"] => self.ls(lines),
                _ => panic!("invalid line: {}", line),
            }
        }
        None
    }
    fn from_command_file(filename: &str) -> Result<Self> {
        let path = PathBuf::from_str(filename)?;
        let content = std::fs::read_to_string(path)?;
        let lines = &mut content.lines().peekable();

        let mut root = Self::new_root();
        while root.from_lines(lines).is_some() {}

        Ok(root)
    }
    fn flatten_dirs(&self) -> Vec<&Directory> {
        let children = self
            .directories
            .iter()
            .map(|dir| dir.flatten_dirs())
            .flatten()
            .collect::<Vec<_>>();

        vec![vec![self], children].concat()
    }
    fn size(&self) -> usize {
        self.directories.iter().map(|dir| dir.size()).sum::<usize>()
            + self.files.iter().map(|file| file.size).sum::<usize>()
    }
    fn size_with_max(&self, max: usize) -> usize {
        self.flatten_dirs()
            .iter()
            .filter_map(|dir| (dir.size() <= max).then_some(dir.size()))
            .sum::<usize>()
    }
    fn reduce_size_by_deleting_this_directory(
        &self,
        file_system_size: usize,
        needed_space: usize,
    ) -> Option<&Self> {
        let space_to_remove = needed_space - (file_system_size - self.size());
        dbg!(space_to_remove);
        self.flatten_dirs()
            .into_iter()
            .filter_map(|dir| (dir.size() >= space_to_remove).then_some(dir))
            .min_by_key(|dir| dir.size())
    }
}

fn main() -> Result<()> {
    let directory = Directory::from_command_file("input.txt")?;
    dbg!(directory.size_with_max(100_000));
    let big_dir = directory
        .reduce_size_by_deleting_this_directory(70_000_000, 30_000_000)
        .unwrap();
    dbg!(big_dir.size());
    Ok(())
}
