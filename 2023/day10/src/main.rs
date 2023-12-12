use std::{
    collections::{hash_map::RandomState, HashSet},
    fs::read_to_string,
    vec,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use anyhow::{anyhow, Error, Result};

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Hash)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn all() -> Vec<Self> {
        Self::iter().collect::<Vec<_>>()
    }
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash)]
enum PipeType {
    Ground,
    Vertical,
    Horizontal,
    DownLeft,
    DownRight,
    UpLeft,
    UpRight,
    Start,
}

impl TryFrom<char> for PipeType {
    type Error = Error;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        Ok(match character {
            '.' => Self::Ground,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'J' => Self::UpLeft,
            'L' => Self::UpRight,
            '7' => Self::DownLeft,
            'F' => Self::DownRight,
            'S' => Self::Start,
            _ => return Err(anyhow!("PipeType not implemented for character")),
        })
    }
}

impl PipeType {
    fn get_directions(&self) -> Vec<Direction> {
        match self {
            PipeType::Vertical => vec![Direction::Up, Direction::Down],
            PipeType::Horizontal => vec![Direction::Left, Direction::Right],
            PipeType::DownLeft => vec![Direction::Down, Direction::Left],
            PipeType::DownRight => vec![Direction::Down, Direction::Right],
            PipeType::UpLeft => vec![Direction::Up, Direction::Left],
            PipeType::UpRight => vec![Direction::Up, Direction::Right],
            PipeType::Start => Direction::all(),
            PipeType::Ground => vec![],
        }
    }
    fn from_directions(directions: &[Direction]) -> Option<Self> {
        Self::iter()
            .filter(|pipe_type| {
                let pipe_type_directions = pipe_type.get_directions();
                directions
                    .iter()
                    .all(|direction| pipe_type_directions.contains(direction))
            })
            .next()
    }
    fn to_char(&self) -> char {
        match self {
            Self::Ground => '.',
            Self::Vertical => '|',
            Self::Horizontal => '-',
            Self::UpLeft => 'J',
            Self::UpRight => 'L',
            Self::DownLeft => '7',
            Self::DownRight => 'F',
            Self::Start => 'S',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pipe {
    pos: (usize, usize),
    pipe_type: PipeType,
}

impl Pipe {
    fn new(pos: (usize, usize), pipe_type: PipeType) -> Self {
        Self { pos, pipe_type }
    }
    fn is_pipe_connected_neighbour(&self, pipe: &Self) -> bool {
        self.pipe_type.get_directions().iter().any(|direction| {
            self.get_pos_at_direction(direction)
                .is_some_and(|neighbour_pos| neighbour_pos == pipe.pos)
                && pipe
                    .pipe_type
                    .get_directions()
                    .iter()
                    .any(|neighbour_direction| {
                        if let Some(pos) = pipe.get_pos_at_direction(&neighbour_direction) {
                            pos == self.pos
                        } else {
                            false
                        }
                    })
        })
    }
    fn get_pos_at_direction(&self, direction: &Direction) -> Option<(usize, usize)> {
        let target_pos = match direction {
            Direction::Up if self.pos.1 != 0 => (self.pos.0, self.pos.1 - 1),
            Direction::Left if self.pos.0 != 0 => (self.pos.0 - 1, self.pos.1),
            Direction::Right => (self.pos.0 + 1, self.pos.1),
            Direction::Down => (self.pos.0, self.pos.1 + 1),
            _ => return None,
        };
        Some(target_pos)
    }
}

#[derive(Debug)]
struct PipeMap {
    pipe_map: Vec<Vec<Pipe>>,
    start_pipe_pos: (usize, usize),
}

impl PipeMap {
    fn new(pipe_map: Vec<Vec<Pipe>>, start_pipe_pos: (usize, usize)) -> Self {
        Self {
            pipe_map,
            start_pipe_pos,
        }
    }

    fn from_file(filename: &str) -> Self {
        let content = read_to_string(filename).unwrap();
        let mut start_pipe_pos = (0, 0);
        let pipe_map_vec = content
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, character)| {
                        let pipe_type = PipeType::try_from(character)?;
                        let pos = (x, y);
                        if pipe_type == PipeType::Start {
                            start_pipe_pos = pos;
                        }
                        Ok(Pipe::new(pos, pipe_type))
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()
            .unwrap();
        PipeMap::new(pipe_map_vec, start_pipe_pos).without_start_pipe_type()
    }
    fn get_start_pipe(&self) -> &Pipe {
        let (x, y) = self.start_pipe_pos;
        &self.pipe_map[y][x]
    }
    fn get_start_pipe_mut(&mut self) -> &mut Pipe {
        let (x, y) = self.start_pipe_pos;
        &mut self.pipe_map[y][x]
    }
    fn without_start_pipe_type(mut self) -> Self {
        let start_pipe = self.get_start_pipe();
        let new_directions = Direction::all()
            .into_iter()
            .filter(|direction| {
                let neighbour_pipe_option = start_pipe
                    .get_pos_at_direction(direction)
                    .and_then(|neighbour_pos| self.get_pipe_at_pos(neighbour_pos));
                let neighbour_pipe = match neighbour_pipe_option {
                    Some(neighbour_pipe_some) => neighbour_pipe_some,
                    None => return false,
                };
                neighbour_pipe
                    .pipe_type
                    .get_directions()
                    .into_iter()
                    .any(|neighbour_direction| &neighbour_direction.opposite() == direction)
            })
            .collect::<Vec<_>>();
        let new_pipe_type = PipeType::from_directions(&new_directions).unwrap();
        self.get_start_pipe_mut().pipe_type = new_pipe_type;
        self
    }
    fn get_pipe_at_pos(&self, pos: (usize, usize)) -> Option<&Pipe> {
        self.pipe_map.get(pos.1).and_then(|row| row.get(pos.0))
    }
    fn get_pipe_neighbour(&self, pipe: &Pipe, direction: &Direction) -> Option<&Pipe> {
        let target_pos = pipe.get_pos_at_direction(direction)?;
        self.get_pipe_at_pos(target_pos)
    }
    fn get_connected_pipe_neighbours(&self, pipe: &Pipe) -> Vec<&Pipe> {
        pipe.pipe_type
            .get_directions()
            .iter()
            .filter_map(|direction| {
                let pipe_neighbour = self.get_pipe_neighbour(pipe, direction)?;
                if pipe.is_pipe_connected_neighbour(pipe_neighbour) {
                    Some(pipe_neighbour)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
    fn get_pipe_furthest_away_count(&self) -> u32 {
        let start_pipe = self.get_start_pipe();

        let mut next_pipes = vec![];
        let mut current_pipes = vec![start_pipe];
        let mut traversed_pipes = HashSet::from([start_pipe]);
        let mut current_count = 0;

        while let Some(cur_pipe) = current_pipes.pop() {
            let neighbours = self.get_connected_pipe_neighbours(cur_pipe);
            let new_neighbours = neighbours
                .into_iter()
                .filter(|neighbour| !traversed_pipes.contains(neighbour))
                .collect::<Vec<_>>();
            for neighbour in new_neighbours {
                next_pipes.push(neighbour);
            }
            traversed_pipes.insert(cur_pipe);
            if current_pipes.is_empty() {
                if next_pipes.is_empty() {
                    break;
                }
                current_pipes = next_pipes.drain(0..).collect();
                current_count += 1;
            }
        }
        current_count
    }
    fn get_pipes_in_loop(&self) -> Vec<&Pipe> {
        let start_pipe = self.get_start_pipe();
        let mut current_pipes = vec![start_pipe];
        let mut traversed_pipes = HashSet::from([start_pipe]);
        while let Some(cur_pipe) = current_pipes.pop() {
            let neighbours = self.get_connected_pipe_neighbours(cur_pipe);
            let new_neighbours = neighbours
                .into_iter()
                .filter(|neighbour| !traversed_pipes.contains(neighbour))
                .collect::<Vec<_>>();
            for neighbour in new_neighbours {
                current_pipes.push(neighbour);
            }
            traversed_pipes.insert(cur_pipe);
        }
        return traversed_pipes.into_iter().collect::<Vec<_>>();
    }
    fn print(&self, highlight_pipes: Vec<&Pipe>) {
        let pipes_in_loop_hash: HashSet<&Pipe, RandomState> =
            HashSet::from_iter(highlight_pipes.into_iter());
        for row in self.pipe_map.iter() {
            for pipe in row {
                if pipes_in_loop_hash.contains(pipe) {
                    print!("\x1b[91m{}\x1b[0m", pipe.pipe_type.to_char())
                } else {
                    print!("{}", pipe.pipe_type.to_char())
                }
            }
            println!()
        }
    }
    fn find_enclosed_pipes(&self) -> Vec<&Pipe> {
        let pipes_in_loop = self.get_pipes_in_loop();
        let pipes_in_loop_hash: HashSet<&Pipe, RandomState> =
            HashSet::from_iter(pipes_in_loop.iter().cloned());
        let mut enclosed_pipes = vec![];
        for row_index in 1..(self.pipe_map.len() - 1) {
            let row = &self.pipe_map[row_index];
            let mut is_inside = false;
            for col_index in 0..(row.len() - 1) {
                let pipe = &row[col_index];
                if pipes_in_loop_hash.contains(pipe) {
                    if pipe.pipe_type == PipeType::Vertical
                        || pipe.pipe_type == PipeType::DownLeft
                        || pipe.pipe_type == PipeType::DownRight
                    {
                        is_inside = !is_inside
                    }
                } else if is_inside {
                    enclosed_pipes.push(pipe)
                }
            }
        }
        enclosed_pipes
    }
    fn find_enclosed_pipes_count(&self) -> usize {
        self.find_enclosed_pipes().len()
    }
}

fn task1() {
    let pipe_map = PipeMap::from_file("input.txt");
    dbg!(pipe_map.get_pipe_furthest_away_count());
}

fn task2() {
    let pipe_map = PipeMap::from_file("input.txt");
    let enclosed_pipes = pipe_map.find_enclosed_pipes();
    pipe_map.print(enclosed_pipes);
    dbg!(pipe_map.find_enclosed_pipes_count());
}

fn main() {
    task1();
    task2();
}
