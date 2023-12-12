use anyhow::Result;
use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Clone)]
struct Sequence {
    numbers: Vec<i32>,
}

impl Sequence {
    fn new(numbers: Vec<i32>) -> Self {
        Self { numbers }
    }
    fn get_step_difference_sequence(&self) -> Sequence {
        let differences = self
            .numbers
            .windows(2)
            .map(|numbers| numbers[1] - numbers[0])
            .collect_vec();
        Sequence::new(differences)
    }
    fn get_next_number(&self) -> i32 {
        if self.numbers.iter().all(|number| *number == 0) {
            return 0;
        }
        let last_step_difference = self.get_step_difference_sequence().get_next_number();
        self.numbers.last().unwrap() + last_step_difference
    }
    fn get_prev_number(&self) -> i32 {
        if self.numbers.iter().all(|number| *number == 0) {
            return 0;
        }
        let first_step_difference = self.get_step_difference_sequence().get_prev_number();
        self.numbers.first().unwrap() - first_step_difference
    }
}

struct Sequences {
    sequences: Vec<Sequence>,
}

impl Sequences {
    fn new(sequences: Vec<Sequence>) -> Self {
        Self { sequences }
    }
    fn from_file(filename: &str) -> Result<Self> {
        let content = read_to_string(filename)?;
        let sequences_vec = content
            .lines()
            .map(|line| {
                let numbers = line
                    .split_whitespace()
                    .map(|numstr| numstr.parse::<i32>().map_err(|err| err.into()))
                    .collect::<Result<Vec<_>>>()?;
                Ok(Sequence::new(numbers))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Sequences::new(sequences_vec))
    }
    fn get_last_number_sum(&self) -> i32 {
        self.sequences
            .iter()
            .fold(0, |acc, sequence| acc + sequence.get_next_number())
    }
    fn get_prev_number_sum(&self) -> i32 {
        self.sequences
            .iter()
            .fold(0, |acc, sequence| acc + sequence.get_prev_number())
    }
}

fn task1() {
    let filename = "input.txt";
    let sequences = Sequences::from_file(filename).unwrap();
    let last_number_sum = sequences.get_last_number_sum();
    dbg!(last_number_sum);
}

fn task2() {
    let filename = "input.txt";
    let sequences = Sequences::from_file(filename).unwrap();
    let last_number_sum = sequences.get_prev_number_sum();
    dbg!(last_number_sum);
}

fn main() {
    task1();
    task2();
}
