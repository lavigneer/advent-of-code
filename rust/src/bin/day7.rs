use std::{collections::HashMap, str::FromStr};

use anyhow::{Error, Result};

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug)]
enum Response {
    Dir(String),
    File(usize, String),
}

#[derive(Debug)]
enum TerminalLine {
    Command(Command),
    Response(Response),
}

impl FromStr for TerminalLine {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut s_split = s.split_whitespace();
        match s_split.next() {
            Some("$") => match s_split.next() {
                Some("cd") => Ok(TerminalLine::Command(Command::Cd(
                    s_split.next().unwrap().to_owned(),
                ))),
                Some("ls") => Ok(TerminalLine::Command(Command::Ls)),
                _ => unreachable!(),
            },
            Some("dir") => Ok(TerminalLine::Response(Response::Dir(
                s_split.next().unwrap().to_owned(),
            ))),
            Some(size) => Ok(TerminalLine::Response(Response::File(
                size.parse().unwrap(),
                s_split.next().unwrap().to_owned(),
            ))),
            _ => unreachable!(),
        }
    }
}

fn main() -> Result<()> {
    let lines = include_str!("./day7.prod")
        .lines()
        .map(|ln| ln.parse::<TerminalLine>().unwrap());
    let mut sizes: HashMap<String, usize> = HashMap::new();
    let mut current_path: Vec<String> = vec![];
    for line in lines {
        match line {
            TerminalLine::Command(Command::Cd(dir)) => match dir.as_str() {
                ".." => {
                    current_path.pop();
                }
                _ => {
                    current_path.push(dir.clone());
                }
            },
            TerminalLine::Response(Response::File(size, _)) => {
                for i in 0..current_path.len() {
                    let path = current_path[0..=i].join("/");
                    let size = sizes.get(&path).unwrap_or(&0usize) + size;
                    sizes.insert(path, size);
                }
            }
            _ => {}
        }
    }
    let total_size = sizes
        .values()
        .filter(|size| **size <= 100000)
        .sum::<usize>();
    println!("Part 1: {}", total_size);

    let unused_space = 70000000 - sizes.get("/").unwrap_or(&0usize);
    let needs_space = 30000000 - unused_space;

    let smalled_free_size = sizes
        .values()
        .filter(|size| **size >= needs_space)
        .min()
        .unwrap();
    println!("Part 2: {}", *smalled_free_size);

    Ok(())
}
