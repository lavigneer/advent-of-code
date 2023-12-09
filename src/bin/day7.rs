use anyhow::Error;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Eq, PartialEq)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    High,
}

impl HandType {
    fn get_hand_value(&self) -> usize {
        match self {
            HandType::High => 1,
            HandType::OnePair => 2,
            HandType::TwoPair => 3,
            HandType::ThreeOfKind => 4,
            HandType::FullHouse => 5,
            HandType::FourOfKind => 6,
            HandType::FiveOfKind => 7,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_hand_value().cmp(&other.get_hand_value())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: Vec<usize>,
    bid: usize,
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(" ").unwrap();
        let cards = cards
            .chars()
            .map(|c| match c {
                'J' => 1,
                '2'..='9' => c.to_string().parse::<usize>().unwrap(),
                'T' => 10,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            })
            .collect::<Vec<usize>>();
        let bid = bid.parse::<usize>().unwrap();
        Ok(Self { cards, bid })
    }
}

impl Hand {
    fn get_hand_type(&self) -> HandType {
        let mut count_map: HashMap<&usize, usize> = HashMap::new();
        for card in self.cards.iter() {
            if let Some(v) = count_map.get_mut(card) {
                *v += 1;
            } else {
                count_map.insert(card, 1);
            }
        }
        let number_of_jokers = count_map.remove(&1);
        let mut sets = count_map.drain().collect::<Vec<(&usize, usize)>>();
        sets.sort_by_key(|(_, count)| Reverse(*count));
        let mut sets = sets.iter();
        match number_of_jokers {
            Some(5) | Some(4) => HandType::FiveOfKind,
            Some(3) => match sets.next().unwrap().1 {
                2 => HandType::FiveOfKind,
                _ => HandType::FourOfKind,
            },
            Some(2) => match sets.next().unwrap().1 {
                3 => HandType::FiveOfKind,
                2 => HandType::FourOfKind,
                _ => HandType::ThreeOfKind,
            },
            Some(1) => match sets.next().unwrap().1 {
                4 => HandType::FiveOfKind,
                3 => HandType::FourOfKind,
                2 => match sets.next().unwrap().1 {
                    2 => HandType::FullHouse,
                    _ => HandType::ThreeOfKind,
                },
                _ => HandType::OnePair,
            },
            None => match sets.next().unwrap().1 {
                5 => HandType::FiveOfKind,
                4 => HandType::FourOfKind,
                3 => match sets.next().unwrap().1 {
                    2 => HandType::FullHouse,
                    _ => HandType::ThreeOfKind,
                },
                2 => match sets.next().unwrap().1 {
                    2 => HandType::TwoPair,
                    _ => HandType::OnePair,
                },
                _ => HandType::High,
            },
            _ => unreachable!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.get_hand_type().cmp(&other.get_hand_type()) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => {
                for i in 0..self.cards.len() {
                    let self_card = self.cards[i];
                    let other_card = other.cards[i];
                    if self_card != other_card {
                        return self_card.cmp(&other_card);
                    }
                }
                return std::cmp::Ordering::Equal;
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut result = include_str!("day7.prod")
        .lines()
        .map(|l| l.parse::<Hand>().unwrap())
        .collect::<Vec<Hand>>();
    result.sort();
    let result: usize = result
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum();
    println!("{:?}", result);
}
