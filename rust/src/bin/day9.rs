use std::{collections::HashSet, str::FromStr};

use anyhow::{Error, Result};

struct Rope {
    positions: Vec<(isize, isize)>,
    positions_set: HashSet<(isize, isize)>,
}

impl Rope {
    fn do_move(&mut self, m: &Move) {
        for _c in 0..m.count {
            match m.direction {
                Direction::Up => {
                    self.positions[0].1 += 1;
                }
                Direction::Down => {
                    self.positions[0].1 -= 1;
                }
                Direction::Left => {
                    self.positions[0].0 -= 1;
                }
                Direction::Right => {
                    self.positions[0].0 += 1;
                }
            }
            for i in 1..self.positions.len() {
                match m.direction {
                    Direction::Up => {
                        while self.positions[i - 1].1 - self.positions[i].1 > 1 {
                            self.positions[i].1 += 1;
                        }
                        while self.positions[i].0 - self.positions[i - 1].0 > 1
                            || self.positions[i].0 - self.positions[i - 1].0 < -1
                        {
                            if self.positions[i].0 < self.positions[i - 1].0 {
                                self.positions[i].0 += 1;
                            } else if self.positions[i].0 > self.positions[i - 1].0 {
                                self.positions[i].0 -= 1;
                            }
                        }
                    }
                    Direction::Down => {
                        while self.positions[i - 1].1 - self.positions[i].1 < -1 {
                            self.positions[i].1 -= 1;
                        }
                        while self.positions[i].0 - self.positions[i - 1].0 > 1
                            || self.positions[i].0 - self.positions[i - 1].0 < -1
                        {
                            if self.positions[i].0 < self.positions[i - 1].0 {
                                self.positions[i].0 += 1;
                            } else if self.positions[i].0 > self.positions[i - 1].0 {
                                self.positions[i].0 -= 1;
                            }
                        }
                    }
                    Direction::Left => {
                        while self.positions[i - 1].0 - self.positions[i].0 < -1 {
                            self.positions[i].0 -= 1;
                        }
                        while self.positions[i].1 - self.positions[i - 1].1 > 1
                            || self.positions[i].1 - self.positions[i - 1].1 < -1
                        {
                            if self.positions[i].1 < self.positions[i - 1].1 {
                                self.positions[i].1 += 1;
                            } else if self.positions[i].1 > self.positions[i - 1].1 {
                                self.positions[i].1 -= 1;
                            }
                        }
                    }
                    Direction::Right => {
                        while self.positions[i - 1].0 - self.positions[i].0 > 1 {
                            self.positions[i].0 += 1;
                        }
                        while self.positions[i].1 - self.positions[i - 1].1 > 1
                            || self.positions[i].1 - self.positions[i - 1].1 < -1
                        {
                            if self.positions[i].1 < self.positions[i - 1].1 {
                                self.positions[i].1 += 1;
                            } else if self.positions[i].1 > self.positions[i - 1].1 {
                                self.positions[i].1 -= 1;
                            }
                        }
                    }
                }
            }
            self.positions_set.insert(*self.positions.last().unwrap());
            println!("{:?}", self.positions);
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    direction: Direction,
    count: usize,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, c) = s.split_once(" ").unwrap();
        let d = d.chars().next().unwrap();
        return Ok(Move {
            direction: match d {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!("Invalid input"),
            },
            count: c.parse().unwrap(),
        });
    }
}

fn main() -> Result<()> {
    /*let mut rope = Rope {
            positions: vec![(0, 0), (0, 0)],
            positions_set: HashSet::new(),
        };
        rope.positions_set.insert(rope.positions[1]);
        include_str!("./day9.prod")
            .lines()
            .flat_map(|ln| ln.parse::<Move>())
            .for_each(|mv| {
                rope.do_move(&mv);
            });
        println!("Part 1: {}", rope.positions_set.len());
    */

    // Part 2
    let mut rope = Rope {
        positions: vec![(0, 0); 10],
        positions_set: HashSet::new(),
    };
    rope.positions_set.insert(rope.positions[9]);
    include_str!("./day9.test2")
        .lines()
        .flat_map(|ln| ln.parse::<Move>())
        .for_each(|mv| {
            rope.do_move(&mv);
        });
    println!("Part 2: {}", rope.positions_set.len());
    return Ok(());
}
