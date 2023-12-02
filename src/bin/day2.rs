use anyhow::Error;
use std::str::FromStr;

#[derive(Debug)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl Round {
    fn possible(&self, red: usize, green: usize, blue: usize) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for cube in s.split(", ") {
            let (num, color) = cube.split_once(" ").unwrap();
            let num = num.parse::<usize>().unwrap();
            match color {
                "red" => {
                    red += num;
                }
                "green" => {
                    green += num;
                }
                "blue" => {
                    blue += num;
                }
                _ => unreachable!(),
            }
        }
        Ok(Round { red, green, blue })
    }
}

#[derive(Debug)]
struct Game {
    number: usize,
    rounds: Vec<Round>,
}

impl Game {
    fn possible(&self, red: usize, green: usize, blue: usize) -> bool {
        self.rounds.iter().all(|r| r.possible(red, green, blue))
    }

    fn power_of_min(&self) -> usize {
        let max_red = self.rounds.iter().map(|r| r.red).max().unwrap();
        let max_green = self.rounds.iter().map(|r| r.green).max().unwrap();
        let max_blue = self.rounds.iter().map(|r| r.blue).max().unwrap();
        max_red * max_green * max_blue
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(": ").unwrap();
        let (_, game_number) = a.split_once(" ").unwrap();
        let game_number = game_number.parse::<usize>().unwrap();
        let rounds = b
            .split("; ")
            .map(|r| r.parse::<Round>().unwrap())
            .collect::<Vec<Round>>();
        return Ok(Game {
            number: game_number,
            rounds,
        });
    }
}

fn main() {
    let red = 12;
    let green = 13;
    let blue = 14;
    let result = include_str!("./day2.prod")
        .lines()
        .into_iter()
        .map(|l| l.parse::<Game>().unwrap())
        .filter(|g| g.possible(red, green, blue))
        .fold(0, |acc, g| acc + g.number);
    println!("Part 1: {}", result);

    let result = include_str!("./day2.prod")
        .lines()
        .into_iter()
        .map(|l| l.parse::<Game>().unwrap().power_of_min())
        .fold(0, |acc, p| acc + p);
    println!("Part 2: {}", result);
}
