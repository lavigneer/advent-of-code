use std::str::FromStr;

use anyhow::{Error, Result};

struct ElfPair {
    first: (usize, usize),
    second: (usize, usize),
}

impl ElfPair {
    fn has_containment(&self) -> bool {
        return (self.first.0 >= self.second.0 && self.first.1 <= self.second.1)
            || (self.second.0 >= self.first.0 && self.second.1 <= self.first.1);
    }

    fn has_overlap(&self) -> bool {
        return (self.first.0 >= self.second.0 && self.first.0 <= self.second.1)
            || (self.second.0 >= self.first.0 && self.second.0 <= self.first.1);
    }
}

impl FromStr for ElfPair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(",").unwrap();
        let (first_start, first_end) = first.split_once("-").unwrap();
        let (second_start, second_end) = second.split_once("-").unwrap();
        return Ok(ElfPair {
            first: (
                first_start.parse::<usize>().unwrap(),
                first_end.parse::<usize>().unwrap(),
            ),
            second: (
                second_start.parse::<usize>().unwrap(),
                second_end.parse::<usize>().unwrap(),
            ),
        });
    }
}

fn main() -> Result<()> {
    let result = include_str!("./day4.prod")
        .lines()
        .filter(|line| line.parse::<ElfPair>().unwrap().has_containment())
        .count();

    println!("Part 1: {}", result);

    let result = include_str!("./day4.prod")
        .lines()
        .filter(|line| line.parse::<ElfPair>().unwrap().has_overlap())
        .count();

    println!("Part 2: {}", result);

    return Ok(());
}
