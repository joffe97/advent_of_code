use anyhow::{anyhow, Ok, Result};
use itertools::Itertools;
use std::{cmp::Ordering, fs::read_to_string};

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_hand(hand: &Hand, include_joker: bool) -> Self {
        let mut card_labels = hand.labels();
        card_labels.sort();

        let mut joker_count = 0;
        let mut card_counts: Vec<u32> = vec![];
        let mut cur_card_label = '\0';

        for card_label in card_labels {
            if include_joker && card_label == 'J' {
                joker_count += 1;
            } else if card_label == cur_card_label {
                *card_counts.last_mut().unwrap() += 1;
            } else {
                cur_card_label = card_label;
                card_counts.push(1);
            }
        }

        let mut card_counts_sorted_decending_iter = card_counts.iter().sorted().rev();
        let highest_card_count =
            card_counts_sorted_decending_iter.next().unwrap_or(&0) + joker_count;
        let second_highest_card_count = card_counts_sorted_decending_iter.next().unwrap_or(&0);

        match (highest_card_count, second_highest_card_count) {
            (5, _) => Self::FiveOfAKind,
            (4, _) => Self::FourOfAKind,
            (3, 2) => Self::FullHouse,
            (3, _) => Self::ThreeOfAKind,
            (2, 2) => Self::TwoPair,
            (2, _) => Self::OnePair,
            (1, _) => Self::HighCard,
            _ => panic!("HandType is not implemented for hand"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    label: char,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl Card {
    fn new(label: char) -> Self {
        let card = Self { label };
        card.value(); // Panics if card value is not implemented for label
        card
    }
    fn value(&self) -> u8 {
        match self.label {
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            'T' => 9,
            'J' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => panic!("Card value not implemented"),
        }
    }
}

#[derive(Debug, Eq, Ord)]
struct Hand {
    cards: Vec<Card>,
    include_joker: bool,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards
            .iter()
            .zip(&other.cards)
            .all(|(card1, card2)| card1.label == card2.label)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = self.get_hand_type().cmp(&other.get_hand_type());
        Some(if let Ordering::Equal = ordering {
            self.values().cmp(&other.values())
        } else {
            ordering
        })
    }
}

impl Hand {
    fn new(cards: Vec<Card>, include_joker: bool) -> Self {
        Self {
            cards,
            include_joker,
        }
    }
    fn from_str(string: &str) -> Self {
        let cards = string
            .chars()
            .map(|card_label| Card::new(card_label))
            .collect_vec();
        Self::new(cards, false)
    }
    fn with_joker(&mut self) {
        self.include_joker = true;
    }
    fn labels(&self) -> Vec<char> {
        self.cards.iter().map(|card| card.label).collect_vec()
    }
    fn values(&self) -> Vec<u8> {
        self.cards
            .iter()
            .map(|card| {
                if self.include_joker && card.label == 'J' {
                    0
                } else {
                    card.value()
                }
            })
            .collect_vec()
    }
    fn get_hand_type(&self) -> HandType {
        HandType::from_hand(self, self.include_joker)
    }
}

#[derive(Debug)]
struct HandBid {
    hand: Hand,
    bid: u32,
}

impl HandBid {
    fn new(hand: Hand, bid: u32) -> Self {
        Self { hand, bid }
    }
    fn from_str(string: &str) -> Option<Self> {
        let (hand_str, bid_str) = string.split_whitespace().collect_tuple()?;
        let hand = Hand::from_str(hand_str);
        let bid = bid_str.parse().ok()?;
        Some(Self::new(hand, bid))
    }
}

#[derive(Debug)]
struct HandBids {
    hand_bids: Vec<HandBid>,
}

impl HandBids {
    fn new(hand_bids: Vec<HandBid>) -> Self {
        Self { hand_bids }
    }
    fn from_file(filepath: &str) -> Result<Self> {
        let content = read_to_string(filepath)?;
        let hand_bids = content
            .lines()
            .map(|line| HandBid::from_str(line))
            .collect::<Option<Vec<_>>>()
            .ok_or(anyhow!("Cannot create HandBid from string"))?;
        Ok(Self::new(hand_bids))
    }
    fn with_joker(mut self) -> Self {
        for hand_bids in &mut self.hand_bids {
            hand_bids.hand.with_joker();
        }
        self
    }
    fn get_hand_bids_sorted_by_hand(&self) -> Vec<&HandBid> {
        self.hand_bids
            .iter()
            .sorted_unstable_by_key(|hand_bid| &hand_bid.hand)
            .collect_vec()
    }
    fn get_winnings(&self) -> u32 {
        let hand_bids_sorted = self.get_hand_bids_sorted_by_hand();
        hand_bids_sorted
            .iter()
            .enumerate()
            .fold(0, |acc, (i, hand_bid)| acc + hand_bid.bid * (i as u32 + 1))
    }
}

fn task1() {
    let filename = "input.txt";
    let hand_bids = HandBids::from_file(filename).unwrap();
    dbg!(hand_bids.get_winnings());
}

fn task2() {
    let filename = "input.txt";
    let hand_bids = HandBids::from_file(filename).unwrap().with_joker();
    dbg!(hand_bids.get_winnings());
}

fn main() {
    task1();
    task2();
}
