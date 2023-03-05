use std::{collections::VecDeque, str::FromStr};

/**
 * Goal: Use custom iterator to not have to track a counter variable
 */
use anyhow::Result;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(isize),
}

struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Instruction::Noop),
            _ => {
                let (_, x) = s.split_once(" ").ok_or(ParseInstructionError)?;
                let parsed_num = x.parse::<isize>().map_err(|_| ParseInstructionError)?;
                return Ok(Instruction::Addx(parsed_num));
            }
        }
    }
}

struct Program {
    register: isize,
    instructions: VecDeque<Instruction>,
    current_instruction_cycles_run: usize,
}

impl Iterator for Program {
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        match (
            self.current_instruction_cycles_run,
            self.instructions.front(),
        ) {
            (0, Some(Instruction::Noop)) => {
                self.instructions.pop_front();
            }
            (0, Some(Instruction::Addx(_x))) => self.current_instruction_cycles_run += 1,
            (1, Some(Instruction::Addx(x))) => {
                self.current_instruction_cycles_run = 0;
                self.register += x;
                self.instructions.pop_front();
            }
            (_, None) => return None,
            _ => unreachable!("OOPS"),
        }

        return Some(self.register);
    }
}

fn main() -> Result<()> {
    let instructions = include_str!("./day10.prod")
        .split("\n")
        .flat_map(|line| line.parse::<Instruction>())
        .collect();

    let program = Program {
        register: 1,
        instructions,
        current_instruction_cycles_run: 0,
    };

    let signal: isize = program
        .skip(18)
        .step_by(40)
        .enumerate()
        .map(|(i, item)| {
            return item * (20 + (i * 40) as isize);
        })
        .sum();

    println!("Part 1: {}", signal);

    return Ok(());
}
