use anyhow::Error;
use rayon::prelude::*;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct Map {
    value: String,
    left: String,
    right: String,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, rest) = s.split_once(" = ").unwrap();
        let (left, right) = rest
            .trim_matches(|c: char| !c.is_alphanumeric())
            .split_once(", ")
            .unwrap();
        Ok(Self {
            value: value.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        })
    }
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn main() {
    let (directions, mappings) = include_str!("./day8.prod").split_once("\n\n").unwrap();
    let mut directions = directions.chars().cycle();
    let mappings =
        mappings
            .lines()
            .map(|l| l.parse::<Map>().unwrap())
            .fold(HashMap::new(), |mut acc, m| {
                acc.insert(m.value.clone(), m);
                acc
            });

    let mut steps = 0;
    let mut current_map = mappings.get("AAA").unwrap();
    loop {
        steps += 1;
        let next_direction = directions.next().unwrap();
        current_map = match next_direction {
            'L' => mappings.get(&current_map.left).unwrap(),
            'R' => mappings.get(&current_map.right).unwrap(),
            _ => unreachable!(),
        };
        if current_map.value == "ZZZ" {
            break;
        }
    }
    println!("Part 1: {}", steps);

    let (directions, mappings) = include_str!("./day8.prod").split_once("\n\n").unwrap();
    let mut directions = directions.chars().cycle();
    let mappings =
        mappings
            .lines()
            .map(|l| l.parse::<Map>().unwrap())
            .fold(HashMap::new(), |mut acc, m| {
                acc.insert(m.value.clone(), m);
                acc
            });

    let mut steps = Vec::new();
    for map in mappings.values().filter(|v| v.value.ends_with('A')) {
        let mut step_count = 0;
        let mut current_map = map;
        loop {
            step_count += 1;
            let next_direction = directions.next().unwrap();
            current_map = match next_direction {
                'L' => mappings.get(&current_map.left).unwrap(),
                'R' => mappings.get(&current_map.right).unwrap(),
                _ => unreachable!(),
            };
            if current_map.value.ends_with('Z') {
                break;
            }
        }
        steps.push(step_count);
    }
    println!("Part 2: {:?}", lcm(&steps));
}
