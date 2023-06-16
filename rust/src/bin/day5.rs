use std::str::FromStr;

use anyhow::{Error, Result};

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn perform_move(&mut self, mv: &Move) {
        for _ in 0..mv.count {
            let item = self.stacks[mv.start_stack - 1].pop().expect("should exist");
            self.stacks[mv.end_stack - 1].push(item);
        }
    }

    fn perform_multi_move(&mut self, mv: &Move) {
        let len = self.stacks[mv.start_stack - 1].len();
        let items = self.stacks[mv.start_stack - 1].split_off(len - mv.count);
        self.stacks[mv.end_stack - 1].extend(items);
    }

    fn get_tops(&self) -> String {
        return self
            .stacks
            .clone()
            .into_iter()
            .map(|stack| stack.into_iter().last().unwrap())
            .collect::<String>();
    }
}

impl FromStr for Stacks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reverse_lines = s.lines().rev();
        let number_row = reverse_lines.next().unwrap();
        let num_indices = number_row.trim().split_whitespace().count();
        let mut stacks = vec![vec![]; num_indices];
        reverse_lines.for_each(|line| {
            for idx in 0..num_indices {
                let line_char = line.chars().nth(idx * 4 + 1).unwrap();
                if line_char.is_alphabetic() {
                    stacks[idx].push(line_char);
                }
            }
        });
        return Ok(Stacks { stacks });
    }
}

struct Move {
    count: usize,
    start_stack: usize,
    end_stack: usize,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_split = s.split(" ");
        let count = s_split.nth(1).unwrap().parse::<usize>().unwrap();
        let start_stack = s_split.nth(1).unwrap().parse::<usize>().unwrap();
        let end_stack = s_split.nth(1).unwrap().parse::<usize>().unwrap();
        return Ok(Move {
            count,
            start_stack,
            end_stack,
        });
    }
}

fn main() -> Result<()> {
    let (stacks, moves) = include_str!("./day5.prod").split_once("\n\n").unwrap();
    let mut stacks = stacks.parse::<Stacks>().unwrap();

    let moves = moves
        .lines()
        .map(|ln| ln.parse::<Move>().unwrap())
        .collect::<Vec<Move>>();
    for m in moves {
        stacks.perform_move(&m);
    }
    println!("Part 1: {}", stacks.get_tops());

    let (stacks, moves) = include_str!("./day5.prod").split_once("\n\n").unwrap();
    let mut stacks = stacks.parse::<Stacks>().unwrap();
    let moves = moves
        .lines()
        .map(|ln| ln.parse::<Move>().unwrap())
        .collect::<Vec<Move>>();
    for m in moves {
        stacks.perform_multi_move(&m);
    }
    println!("Part 2: {}", stacks.get_tops());
    return Ok(());
}
