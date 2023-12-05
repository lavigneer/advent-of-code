use anyhow::Error;
use std::{collections::HashSet, str::FromStr};

#[derive(Clone, Debug)]
struct Card {
    number: usize,
    winning_numbers: HashSet<usize>,
    card_numbers: HashSet<usize>,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_num, number_sets) = s.split_once(":").unwrap();
        let (_, card_num) = card_num.split_once(" ").unwrap();
        let card_num = card_num.trim().parse::<usize>().unwrap();
        let number_sets = number_sets.trim();
        let (winning_numbers, card_numbers) = number_sets.split_once("|").unwrap();
        let winning_numbers = winning_numbers
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let card_numbers = card_numbers
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        Ok(Self {
            number: card_num,
            winning_numbers,
            card_numbers,
        })
    }
}

impl Card {
    fn matches(&self) -> usize {
        self.card_numbers
            .intersection(&self.winning_numbers)
            .count()
    }

    fn prize(&self) -> u32 {
        let matches = self.matches();
        if matches == 0 {
            return 0;
        }
        2_u32.pow(matches as u32 - 1)
    }
}

fn main() {
    let prize: u32 = include_str!("./day4.prod")
        .lines()
        .map(|card| card.parse::<Card>().unwrap().prize())
        .sum();
    println!("Part 1: {}", prize);

    let cards = include_str!("./day4.prod")
        .lines()
        .map(|card| card.parse::<Card>().unwrap())
        .collect::<Vec<Card>>();
    let mut cards_to_process = cards.clone();

    let mut count = 0;
    while !cards_to_process.is_empty() {
        let current_card = cards_to_process.pop().unwrap();
        count += 1;
        let matches = current_card.matches();
        for i in 0..matches {
            cards_to_process.push(cards[current_card.number + i].clone());
        }
    }
    println!("Part 2: {}", count);
}
