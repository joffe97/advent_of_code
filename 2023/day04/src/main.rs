use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Debug)]
struct Card {
    card_id: u32,
    winning: HashSet<u32>,
    numbers: Vec<u32>,
}

impl TryFrom<&str> for Card {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"Card\s+(.+):\s*([\s\d]+)\s+\|\s*([\s\d]+)$").unwrap();
        let captures = re.captures(value).ok_or(anyhow!("cannot capture string"))?;
        let card_id = captures[1].parse::<u32>()?;
        let winning = captures[2]
            .split_whitespace()
            .map(|numstr| numstr.parse::<u32>())
            .collect::<Result<HashSet<_>, _>>()?;
        let numbers = captures[3]
            .split_whitespace()
            .map(|numstr| numstr.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Card::new(card_id, winning, numbers))
    }
}

impl Card {
    fn new(card_id: u32, winning: HashSet<u32>, numbers: Vec<u32>) -> Self {
        Self {
            card_id,
            winning,
            numbers,
        }
    }
    fn get_winning_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter_map(|number| self.winning.contains(number).then_some(*number))
            .collect_vec()
    }
    fn get_winning_numbers_count(&self) -> u32 {
        self.get_winning_numbers().len() as u32
    }
    fn get_points(&self) -> u32 {
        let winning_numbers_count = self.get_winning_numbers_count();
        if winning_numbers_count == 0 {
            return 0;
        }
        2_u32.pow(winning_numbers_count - 1)
    }
}

#[derive(Debug)]
struct Cards {
    cards: HashMap<u32, Card>,
}

impl Cards {
    fn new(cards: HashMap<u32, Card>) -> Self {
        Self { cards }
    }
    fn try_from_file(filename: &str) -> Result<Self> {
        let file_content = read_to_string(filename).unwrap();
        file_content
            .lines()
            .map(|line| Card::try_from(line).and_then(|card| Ok((card.card_id, card))))
            .collect::<Result<HashMap<_, _>>>()
            .and_then(|cards_vec| Ok(Cards::new(cards_vec)))
    }
    fn get_points(&self) -> u32 {
        self.cards.iter().map(|(_, card)| card.get_points()).sum()
    }
    fn get_total_scratchcards_count(&self) -> u32 {
        let mut card_counts = vec![1; self.cards.len()];
        for i in 0..card_counts.len() {
            let cur_card_count = card_counts[i];
            let cur_card_id = i + 1;
            let cur_card = self.cards.get(&(cur_card_id as u32)).unwrap();
            for increment_index_diff in 1..=cur_card.get_winning_numbers_count() {
                let increment_card_id = i + increment_index_diff as usize;
                if increment_card_id >= card_counts.len() {
                    break;
                }
                card_counts[increment_card_id] += cur_card_count;
            }
        }
        card_counts.into_iter().sum()
    }
}

fn main() {
    task1();
    task2();
}

fn task1() {
    let filename = "input.txt";
    let cards = Cards::try_from_file(filename).unwrap();
    dbg!(cards.get_points());
}

fn task2() {
    let filename = "input.txt";
    let cards = Cards::try_from_file(filename).unwrap();
    dbg!(cards.get_total_scratchcards_count());
}
