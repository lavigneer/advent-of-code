use std::{collections::VecDeque, fmt::Display, str::FromStr};

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

struct CRT {
    pixels: [[Option<char>; 40]; 6],
}

impl CRT {
    fn run_program(&mut self, program: Program) {
        self.pixels[0][0] = Some('#');
        program.enumerate().for_each(|(idx, register)| {
            let idx = idx + 1;
            if idx < 40 * 6 {
                let row = idx / 40;
                let col = idx % 40;
                if register >= -1 && register <= 40 {
                    self.pixels[row][col] = match register - col as isize {
                        -1 | 0 | 1 => Some('#'),
                        _ => Some('.'),
                    }
                } else {
                    self.pixels[row][col] = Some('.');
                }
            }
        })
    }
}

impl Display for CRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.pixels {
            for pixel in row {
                write!(f, "{}", pixel.unwrap())?;
            }
            write!(f, "\n")?;
        }
        return Ok(());
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

    let instructions = include_str!("./day10.prod")
        .split("\n")
        .flat_map(|line| line.parse::<Instruction>())
        .collect();

    let program = Program {
        register: 1,
        instructions,
        current_instruction_cycles_run: 0,
    };
    let mut crt = CRT {
        pixels: [[None; 40]; 6],
    };
    crt.run_program(program);

    print!("Part 2: \n{}", crt);

    return Ok(());
}
