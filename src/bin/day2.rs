use std::char;
use std::str::FromStr;

use anyhow::{Error, Result};

struct Round {
    e: char,
    s: char,
}

impl Round {
    fn get_score(&self) -> usize {
        let shape_score = self.s as usize - 87;
        let outcome_mod = ((((self.e as isize - 65) - (self.s as isize - 88)) % 3) + 3) % 3;
        return match outcome_mod {
            0 => 3 + shape_score,
            1 => shape_score,
            2 => 6 + shape_score,
            _ => unreachable!(),
        };
    }

    fn get_score_part_two(&self) -> usize {
        let elf_offset = self.e as usize - 65;
        let personal_shape = match self.s {
            'X' => ((elf_offset + 2) % 3) + 88,
            'Y' => elf_offset + 88,
            'Z' => ((elf_offset + 1) % 3) + 88,
            _ => unreachable!(),
        };
        return Round {
            e: self.e,
            s: char::from_u32(personal_shape as u32).unwrap(),
        }
        .get_score();
    }
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (e, s) = match s.split_once(" ") {
            Some((e, s)) => (e, s),
            None => return Err(anyhow::anyhow!("invalid input")),
        };

        return Ok(Round {
            e: e.chars().next().unwrap(),
            s: s.chars().next().unwrap(),
        });
    }
}

fn main() -> Result<()> {
    let score = include_str!("./day2.prod")
        .split("\n")
        .flat_map(|rd| rd.parse::<Round>())
        .map(|rd| rd.get_score())
        .sum::<usize>();

    println!("Part one: {:?}", score);

    let score = include_str!("./day2.prod")
        .split("\n")
        .flat_map(|rd| rd.parse::<Round>())
        .map(|rd| rd.get_score_part_two())
        .sum::<usize>();

    println!("Part two: {:?}", score);

    return Ok(());
}
