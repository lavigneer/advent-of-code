use std::cmp::Ordering;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
enum Signal {
    Item(usize),
    List(Vec<Signal>),
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Signal::Item(self_val), Signal::Item(other_val)) => self_val.partial_cmp(other_val),
            (Signal::List(self_val), Signal::List(other_val)) => {
                self_val.iter().partial_cmp(other_val.iter())
            }
            (Signal::Item(self_val), other_val) => {
                Signal::List(vec![Signal::Item(*self_val)]).partial_cmp(other_val)
            }
            (self_val, Signal::Item(other_val)) => {
                self_val.partial_cmp(&Signal::List(vec![Signal::Item(*other_val)]))
            }
        }
    }
}

fn main() -> Result<()> {
    let sum: usize = include_str!("./day13.prod")
        .split("\n\n")
        .enumerate()
        .map(|(index, signals)| {
            let (left, right) = signals.split_once('\n').unwrap();
            let left: Signal = serde_json::from_str(left).unwrap();
            let right: Signal = serde_json::from_str(right).unwrap();
            match left.partial_cmp(&right) {
                Some(Ordering::Less) => index + 1,
                Some(Ordering::Equal) => index + 1,
                _ => 0,
            }
        })
        .sum();
    println!("Part 1: {}", sum);

    let mut lines: Vec<Signal> = include_str!("./day13.prod")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|signal| serde_json::from_str(signal).unwrap())
        .collect();
    let distress_one: Signal = serde_json::from_str("[[2]]").unwrap();
    let distress_two: Signal = serde_json::from_str("[[6]]").unwrap();
    lines.push(distress_one.clone());
    lines.push(distress_two.clone());
    lines.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut distress_one_index = 0;
    let mut distress_two_index = 0;
    for (index, item) in lines.iter().enumerate() {
        if item == &distress_one {
            distress_one_index = index + 1;
        } else if item == &distress_two {
            distress_two_index = index + 1;
        }
    }
    println!("Part 2: {}", distress_one_index * distress_two_index);

    Ok(())
}
