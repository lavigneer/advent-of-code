use std::char;
use std::str::FromStr;

use anyhow::{Error, Result};

struct Round {
    elf: char,
    personal: char,
}

const A_ASCII_DIGIT: usize = 65;
const X_ASCII_DIGIT: usize = 88;

impl Round {
    fn get_score(&self) -> usize {
        let shape_score = self.personal as usize - X_ASCII_DIGIT + 1;
        let outcome_mod = ((((self.elf as isize - A_ASCII_DIGIT as isize)
            - (self.personal as isize - X_ASCII_DIGIT as isize))
            % 3)
            + 3)
            % 3;
        return match outcome_mod {
            0 => 3 + shape_score,
            1 => shape_score,
            2 => 6 + shape_score,
            _ => unreachable!(),
        };
    }

    fn get_score_part_two(&self) -> usize {
        let elf_offset = self.elf as usize - A_ASCII_DIGIT;
        let personal_shape = match self.personal {
            'X' => ((elf_offset + 2) % 3) + X_ASCII_DIGIT,
            'Y' => elf_offset + X_ASCII_DIGIT,
            'Z' => ((elf_offset + 1) % 3) + X_ASCII_DIGIT,
            _ => unreachable!(),
        };

        // Just reuse part 1 logic by converting the second column to what we should play
        return Round {
            elf: self.elf,
            personal: char::from_u32(personal_shape as u32).unwrap(),
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
            elf: e.chars().next().unwrap(),
            personal: s.chars().next().unwrap(),
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
