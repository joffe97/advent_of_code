use anyhow::{Error, Result};
use enum_iterator::{all, Sequence};
use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Deref},
    path::PathBuf,
    str::FromStr,
};

#[derive(Sequence)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn to_tuple(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Tree {
    position: (usize, usize),
    height: u8,
}

impl Tree {
    fn new(position: (usize, usize), height: u8) -> Self {
        Self { position, height }
    }
}

struct Forest {
    matrix: Vec<Vec<Tree>>,
}

impl Forest {
    fn new(matrix: Vec<Vec<Tree>>) -> Self {
        Self { matrix }
    }
    fn try_from_filename(filename: &str) -> Result<Self> {
        let path = PathBuf::from_str(filename)?;
        let content = std::fs::read_to_string(path)?;
        Self::from_str(&content)
    }
    fn x_len(&self) -> usize {
        self.matrix.first().expect("matrix must have content").len()
    }
    fn y_len(&self) -> usize {
        self.matrix.len()
    }
    fn x_iter(&self, y_index: usize) -> impl DoubleEndedIterator<Item = &Tree> {
        self.matrix[y_index].iter().map(|i| i)
    }
    fn y_iter(&self, x_index: usize) -> impl DoubleEndedIterator<Item = &Tree> {
        self.matrix
            .iter()
            .map(move |row| row.get(x_index).expect("index_must_exist"))
    }
    fn get_tree(&self, position: (isize, isize)) -> Option<&Tree> {
        let (x_pos, y_pos) = position;
        if x_pos.is_negative() || y_pos.is_negative() {
            return None;
        }
        self.matrix.get(y_pos as usize)?.get(x_pos as usize)
    }
    fn visible_trees_in_direction_at_index(
        &self,
        direction: &Direction,
        index: usize,
    ) -> Vec<&Tree> {
        let mut biggest_found_height = -1;
        let tree_closure = |tree: &&Tree| {
            let tree_height_i8 = tree.height as i8;
            let is_higher = tree_height_i8 > biggest_found_height;
            if is_higher {
                biggest_found_height = tree_height_i8;
            }
            is_higher
        };
        match direction {
            Direction::North => self.y_iter(index).rev().filter(tree_closure).collect(),
            Direction::South => self.y_iter(index).filter(tree_closure).collect(),
            Direction::West => self.x_iter(index).rev().filter(tree_closure).collect(),
            Direction::East => self.x_iter(index).filter(tree_closure).collect(),
        }
    }
    fn visible_trees_in_direction(&self, direction: &Direction) -> HashSet<&Tree> {
        let direction_len = match direction {
            Direction::North | Direction::South => self.y_len(),
            Direction::West | Direction::East => self.x_len(),
        };
        let mut visible_trees = HashSet::new();
        (0..direction_len).for_each(|i| {
            let visible_trees_in_direction_at_index =
                self.visible_trees_in_direction_at_index(direction, i);
            visible_trees.extend(visible_trees_in_direction_at_index.into_iter());
        });
        visible_trees
    }
    fn visible_trees(&self) -> HashSet<&Tree> {
        let mut visible_trees = HashSet::new();
        for direction in all::<Direction>() {
            let visible_trees_in_direction = self.visible_trees_in_direction(&direction);
            visible_trees.extend(visible_trees_in_direction.into_iter());
        }
        visible_trees
    }
    fn scenic_score_of_tree_at_position(&self, position: (usize, usize)) -> Option<u32> {
        let tree_at_pos = self.get_tree((position.0 as isize, position.1 as isize))?;

        let scenic_score = all::<Direction>()
            .into_iter()
            .map(|direction| {
                let (x_direction, y_direction) = &direction.to_tuple();
                let mut cur_x_pos = position.0 as isize;
                let mut cur_y_pos = position.1 as isize;

                cur_x_pos += x_direction;
                cur_y_pos += y_direction;

                let mut viewing_distance = 0;

                while let Some(cur_tree) = self.get_tree((cur_x_pos, cur_y_pos)) {
                    viewing_distance += 1;
                    if cur_tree.height >= tree_at_pos.height {
                        break;
                    }
                    cur_x_pos += x_direction;
                    cur_y_pos += y_direction;
                }

                viewing_distance
            })
            .product::<u32>();

        Some(scenic_score)
    }
    fn highest_scenic_score(&self) -> u32 {
        let mut highest = 0;
        for y in 0..self.y_len() {
            for x in 0..self.x_len() {
                let cur_score = self.scenic_score_of_tree_at_position((x, y)).unwrap();
                highest = highest.max(cur_score)
            }
        }
        highest
    }
}

impl FromStr for Forest {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let matrix = string
            .lines()
            .enumerate()
            .map(|(y_index, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x_index, chr)| {
                        let tree_height = chr.to_string().parse::<u8>()?;
                        let tree = Tree::new((x_index, y_index), tree_height);
                        Ok(tree)
                    })
                    .collect::<Result<Vec<Tree>>>()
            })
            .collect::<Result<Vec<Vec<Tree>>>>()?;
        Ok(Self::new(matrix))
    }
}

fn main() -> Result<()> {
    let forest = Forest::try_from_filename("input.txt")?;
    let visible_trees = forest.visible_trees();
    dbg!(visible_trees.len());
    let highest_score = forest.highest_scenic_score();
    dbg!(highest_score);
    Ok(())
}
