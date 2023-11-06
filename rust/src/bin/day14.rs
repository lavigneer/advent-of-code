use std::{
    cmp,
    collections::HashSet,
    str::FromStr,
};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: isize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some((y, x)) = s.split_once(",") {
            return Ok(Self {
                x: x.parse().map_err(|_| ParsePointError)?,
                y: y.parse().map_err(|_| ParsePointError)?,
            });
        }
        Err(ParsePointError)
    }
}

#[derive(Debug)]
struct SandMap {
    seen: HashSet<Point>,
    current_sand_point: Point,
    max_row: usize,
    floor: Option<usize>,
}

enum StepResult {
    Done,
    MoreStepsAvailable,
}

impl SandMap {
    pub fn new() -> Self {
        Self {
            seen: HashSet::new(),
            current_sand_point: Point { x: 0, y: 500 },
            max_row: 0,
            floor: None,
        }
    }

    pub fn add_lines(&mut self, lines: &SandMapStructure) {
        for l in lines.0.windows(2) {
            if let [a, b] = l {
                let start_x = cmp::min(a.x, b.x);
                let end_x = cmp::max(a.x, b.x);
                let start_y = cmp::min(a.y, b.y);
                let end_y = cmp::max(a.y, b.y);

                self.max_row = cmp::max(self.max_row, end_x);

                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        self.seen.insert(Point { x, y });
                    }
                }
            }
        }
    }

    pub fn set_floor(&mut self, floor: Option<usize>) {
        self.floor = floor;
        if let Some(floor) = floor {
            self.max_row = cmp::max(self.max_row, floor);
        }
    }

    pub fn step(&mut self) -> Option<bool> {
        if self.current_sand_point.x >= self.max_row {
            return None;
        }
        let below_point = Point {
            x: self.current_sand_point.x + 1,
            y: self.current_sand_point.y,
        };
        let below_left_point = Point {
            x: self.current_sand_point.x + 1,
            y: self.current_sand_point.y - 1,
        };
        let below_right_point = Point {
            x: self.current_sand_point.x + 1,
            y: self.current_sand_point.y + 1,
        };

        if let Some(floor) = self.floor {
            if below_point.x == floor {
                // We hit the floor, stop here
                return Some(false);
            }
        }
        if !self.seen.contains(&below_point) {
            self.current_sand_point = below_point;
        } else if !self.seen.contains(&below_left_point) {
            self.current_sand_point = below_left_point;
        } else if !self.seen.contains(&below_right_point) {
            self.current_sand_point = below_right_point;
        } else if self.current_sand_point.eq(&Point {x: 0, y: 500} ) {
            // There's nothing below us and we're at the starting point,
            // no more space to move
            return None;
        } else {
            // We've seen all three spots below us
            return Some(false);
        }
        return Some(true);
    }

    pub fn full_step(&mut self) -> StepResult {
        loop {
            match self.step() {
                None => return StepResult::Done,
                Some(false) => {
                    self.seen.insert(self.current_sand_point.clone());
                    self.current_sand_point = Point { x: 0, y: 500 };
                    return StepResult::MoreStepsAvailable;
                }
                Some(true) => continue,
            }
        }
    }
}

struct SandMapStructure(Vec<Point>);
struct ParseSandMapStructureError;

impl FromStr for SandMapStructure {
    type Err = ParseSandMapStructureError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let points = s
            .split(" -> ")
            .flat_map(|point_str| point_str.parse::<Point>())
            .collect();
        Ok(Self(points))
    }
}

fn main() -> Result<()> {
    let structures: Vec<SandMapStructure> = include_str!("./day14.prod")
        .lines()
        .flat_map(|line| line.parse::<SandMapStructure>())
        .collect();
    let mut map = SandMap::new();
    structures.iter().for_each(|s| map.add_lines(s));
    let mut count = 0;
    loop {
        match map.full_step() {
            StepResult::Done => break,
            _ => {
                count = count + 1;
            }
        }
    }
    println!("Part 1: {}", count);

    let structures: Vec<SandMapStructure> = include_str!("./day14.prod")
        .lines()
        .flat_map(|line| line.parse::<SandMapStructure>())
        .collect();
    let mut map = SandMap::new();
    structures.iter().for_each(|s| map.add_lines(s));
    map.set_floor(Some(map.max_row + 2));

    let mut count = 0;
    loop {
        match map.full_step() {
            StepResult::Done => break,
            _ => {
                count = count + 1;
            }
        }
    }
    println!("Part 2: {}", count + 1);

    return Ok(());
}
