use std::{collections::HashSet, str::FromStr};

use anyhow::{Error, Result};

struct Rucksack {
    first_compartment: HashSet<char>,
    second_compartment: HashSet<char>,
}

const LOWER_A_ASCII_DIGIT: usize = 97;
const LOWER_Z_ASCII_DIGIT: usize = 122;
const UPPER_A_ASCII_DIGIT: usize = 65;
const UPPER_Z_ASCII_DIGIT: usize = 90;

fn get_priority_for_char(ch: &char) -> usize {
    let ch_digit: usize = *ch as usize;
    let val: usize;
    if ch_digit >= UPPER_A_ASCII_DIGIT && ch_digit <= UPPER_Z_ASCII_DIGIT {
        val = ch_digit - UPPER_A_ASCII_DIGIT + 27;
    } else if ch_digit >= LOWER_A_ASCII_DIGIT && ch_digit <= LOWER_Z_ASCII_DIGIT {
        val = ch_digit - LOWER_A_ASCII_DIGIT + 1;
    } else {
        unreachable!()
    }
    return val;
}

impl Rucksack {
    fn get_priority(&self) -> usize {
        let diff = self
            .first_compartment
            .intersection(&self.second_compartment);
        return diff
            .into_iter()
            .map(|ch| get_priority_for_char(ch))
            .sum::<usize>();
    }
}

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (f, s) = s.split_at(s.len() / 2);
        return Ok(Rucksack {
            first_compartment: HashSet::from_iter(f.chars()),
            second_compartment: HashSet::from_iter(s.chars()),
        });
    }
}

struct ElfGroup {
    first: HashSet<char>,
    second: HashSet<char>,
    third: HashSet<char>,
}

impl ElfGroup {
    fn get_priority(&self) -> usize {
        let diff = self
            .first
            .intersection(&self.second)
            .cloned()
            .collect::<HashSet<char>>();
        let diff = diff.intersection(&self.third);
        return diff
            .into_iter()
            .map(|ch| get_priority_for_char(ch))
            .sum::<usize>();
    }
}

fn main() -> Result<()> {
    let answer = include_str!("./day3.prod")
        .split("\n")
        .map(|line| line.parse::<Rucksack>().unwrap().get_priority())
        .sum::<usize>();

    println!("Part 1: {}", answer);

    let answer = include_str!("./day3.prod")
        .strip_suffix("\n")
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            ElfGroup {
                first: HashSet::from_iter(group[0].chars()),
                second: HashSet::from_iter(group[1].chars()),
                third: HashSet::from_iter(group[2].chars()),
            }
            .get_priority()
        })
        .sum::<usize>();

    println!("Part 2: {}", answer);
    return Ok(());
}
