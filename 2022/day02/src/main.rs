use anyhow::{anyhow, Error, Result};
use std::{fs::read_to_string, path::PathBuf, str::FromStr};

#[derive(PartialEq, Eq)]
enum GameResult {
    Win,
    Draw,
    Lose,
}

impl GameResult {
    fn points(&self) -> u32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
    fn invert(&self) -> Self {
        match self {
            Self::Lose => Self::Win,
            Self::Draw => Self::Draw,
            Self::Win => Self::Lose,
        }
    }
}

impl FromStr for GameResult {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(match string {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => return Err(anyhow!("invalid game result string")),
        })
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl Hand {
    fn all() -> Vec<Hand> {
        vec![Self::Rock, Self::Paper, Self::Scissor]
    }
    const fn wins(&self) -> Self {
        match self {
            Self::Rock => Self::Scissor,
            Self::Paper => Self::Rock,
            Self::Scissor => Self::Paper,
        }
    }
    fn draws(&self) -> Self {
        self.clone()
    }
    fn loses(&self) -> Self {
        Self::all()
            .into_iter()
            .find(|hand| &hand.wins() == self)
            .unwrap()
    }
    fn create_from_result(&self, game_state: &GameResult) -> Self {
        match game_state {
            GameResult::Lose => self.loses(),
            GameResult::Draw => self.draws(),
            GameResult::Win => self.wins(),
        }
    }
    const fn points(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(match string {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => return Err(anyhow!("invalid hand string")),
        })
    }
}

struct RockPaperScissors {
    you: Hand,
    opponent: Hand,
}

impl FromStr for RockPaperScissors {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string_split = string.split_whitespace().collect::<Vec<_>>();
        let opponent_str = string_split
            .get(0)
            .ok_or(anyhow!("cannot find your hand"))?;
        let you_str = string_split
            .get(1)
            .ok_or(anyhow!("cannot find your hand"))?;
        Ok(RockPaperScissors::new(
            Hand::from_str(you_str)?,
            Hand::from_str(opponent_str)?,
        ))
    }
}

impl RockPaperScissors {
    fn new(you: Hand, opponent: Hand) -> Self {
        Self { you, opponent }
    }
    fn game_result(&self) -> GameResult {
        if self.you == self.opponent {
            GameResult::Draw
        } else if self.you.wins() == self.opponent {
            GameResult::Win
        } else {
            GameResult::Lose
        }
    }
    fn points(&self) -> u32 {
        self.you.points() + self.game_result().points()
    }
    fn from_str_2(str: &str) -> Result<Self> {
        let string_split = str.split_whitespace().collect::<Vec<_>>();
        let opponent_str = string_split
            .get(0)
            .ok_or(anyhow!("cannot find your hand"))?;
        let game_result_str = string_split
            .get(1)
            .ok_or(anyhow!("cannot find your hand"))?;

        let opponent = Hand::from_str(opponent_str)?;
        let game_result = GameResult::from_str(game_result_str)?;
        let you = opponent.create_from_result(&game_result.invert());

        Ok(RockPaperScissors::new(you, opponent))
    }
}

struct RockPaperScissorsCollection {
    collection: Vec<RockPaperScissors>,
}

impl RockPaperScissorsCollection {
    fn new(collection: Vec<RockPaperScissors>) -> Self {
        Self { collection }
    }
    fn try_from_string_vec_1(string_vec: Vec<&str>) -> Result<Self> {
        let collection = string_vec
            .into_iter()
            .map(|str| RockPaperScissors::from_str(str))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self::new(collection))
    }
    fn try_from_string_vec_2(string_vec: Vec<&str>) -> Result<Self> {
        let collection = string_vec
            .into_iter()
            .map(|str| RockPaperScissors::from_str_2(str))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self::new(collection))
    }
    fn points(&self) -> u32 {
        self.collection.iter().map(|game| game.points()).sum()
    }
}

fn read_file(filename: &str) -> Result<RockPaperScissorsCollection> {
    let path = PathBuf::from(filename);
    let content = read_to_string(path)?;
    let string_vec = content.lines().collect();
    RockPaperScissorsCollection::try_from_string_vec_2(string_vec)
}

fn main() -> Result<()> {
    let collection = read_file("input.txt")?;
    dbg!(collection.points());
    Ok(())
}
