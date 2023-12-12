use std::fs::read_to_string;

use itertools::Itertools;

fn main() {
    task1();
    task2();
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }
    fn from_filename(filename: &str) -> Self {
        let file_content = read_to_string(filename).unwrap();
        let (time_number, distance_number) = file_content
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .skip(1)
                    .join("")
                    .parse::<u64>()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();
        Self::new(time_number, distance_number)
    }
    fn calculate_distance_for_hold_time(&self, hold_time: u64) -> u64 {
        hold_time * (self.time - hold_time)
    }
    fn winning_hold_times(&self) -> Vec<u64> {
        let hold_time_test_range = (1)..(self.time);
        hold_time_test_range
            .into_iter()
            .map(|hold_time| self.calculate_distance_for_hold_time(hold_time))
            .filter(|distance| distance > &self.distance)
            .collect_vec()
    }
    fn winning_hold_times_count(&self) -> usize {
        self.winning_hold_times().len()
    }
}

#[derive(Debug)]
struct Races {
    races: Vec<Race>,
}

impl Races {
    fn new(races: Vec<Race>) -> Self {
        Self { races }
    }
    fn from_filename(filename: &str) -> Self {
        let file_content = read_to_string(filename).unwrap();
        let (time_numbers, distance_numbers) = file_content
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .skip(1)
                    .map(|numstr| numstr.parse::<u64>().unwrap())
            })
            .collect_tuple()
            .unwrap();
        let races_vec = time_numbers
            .zip(distance_numbers)
            .map(|(race_time, race_distance)| Race::new(race_time, race_distance))
            .collect::<Vec<_>>();
        Self::new(races_vec)
    }
    fn winning_hold_times_count_multiplied(&self) -> usize {
        self.races
            .iter()
            .map(|race| race.winning_hold_times_count())
            .product()
    }
}

fn task1() {
    let filename = "input1.txt";
    let races = Races::from_filename(filename);
    dbg!(races.winning_hold_times_count_multiplied());
}

fn task2() {
    let filename = "input1.txt";
    let race = Race::from_filename(filename);
    dbg!(race.winning_hold_times_count());
}
