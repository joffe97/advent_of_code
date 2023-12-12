use anyhow::{anyhow, Error, Result};
use itertools::{FoldWhile, Itertools};
use num::integer::lcm;
use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(anyhow!("Cannot create Direction from given char")),
        }
    }
}

#[derive(Debug)]
struct Node {
    label: String,
    target_label_left: String,
    target_label_right: String,
}

impl Node {
    fn new(label: String, target_label_left: String, target_label_right: String) -> Self {
        Self {
            label,
            target_label_left,
            target_label_right,
        }
    }
}

struct Network {
    nodes: HashMap<String, Node>,
}

impl Network {
    fn new(nodes: HashMap<String, Node>) -> Self {
        Self { nodes }
    }
    fn walk_directions(
        &self,
        directions: &Directions,
        start_labels: &Vec<&str>,
        end_labels: &Vec<&str>,
    ) -> (Vec<&str>, u64) {
        let start_nodes = start_labels
            .iter()
            .cloned()
            .map(|start_label| self.nodes.get(start_label).unwrap())
            .collect_vec();

        let (nodes, walk_count) = directions
            .directions
            .iter()
            .fold_while(
                (start_nodes, 0),
                |(cur_nodes, cur_walk_count), cur_direction| {
                    if cur_nodes
                        .iter()
                        .all(|node| end_labels.contains(&node.label.as_str()))
                    {
                        return FoldWhile::Done((cur_nodes, cur_walk_count));
                    }
                    let next_nodes = match cur_direction {
                        Direction::Left => cur_nodes
                            .into_iter()
                            .map(|node| self.nodes.get(&node.target_label_left).unwrap())
                            .collect_vec(),
                        Direction::Right => cur_nodes
                            .into_iter()
                            .map(|node| self.nodes.get(&node.target_label_right).unwrap())
                            .collect_vec(),
                    };
                    FoldWhile::Continue((next_nodes, cur_walk_count + 1))
                },
            )
            .into_inner();
        let labels = nodes.iter().map(|node| node.label.as_str()).collect_vec();
        (labels, walk_count)
    }
    fn walk_directions_until_end<'a>(
        &'a self,
        directions: &'a Directions,
        start_labels: &Vec<&'a str>,
        end_labels: &Vec<&'a str>,
    ) -> (Vec<&str>, u64) {
        let mut total_walk_count = 0;
        let mut cur_labels = start_labels.clone();

        while !cur_labels.iter().all(|label| end_labels.contains(label)) {
            let (cur_end_labels, walk_count) =
                self.walk_directions(directions, &cur_labels, &end_labels);
            total_walk_count += walk_count;
            cur_labels = cur_end_labels;
        }
        (cur_labels, total_walk_count)
    }
    fn get_nodes_ending_with_char(&self, character: char) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter(|node| node.0.chars().last().unwrap().eq(&character))
            .map(|(_, node)| node)
            .collect_vec()
    }
}

struct Directions {
    directions: Vec<Direction>,
}

impl Directions {
    fn new(directions: Vec<Direction>) -> Self {
        Self { directions }
    }
}

struct NetworkWithDirections {
    network: Network,
    directions: Directions,
}

impl NetworkWithDirections {
    fn new(network: Network, directions: Directions) -> Self {
        Self {
            network,
            directions,
        }
    }
    fn from_file(filename: &str) -> Result<Self> {
        let content = read_to_string(filename)?;
        let mut lines_iter = content.lines();

        let directions_vec = lines_iter
            .next()
            .ok_or(anyhow!("Cannot read directions from file"))?
            .chars()
            .map(|character| Direction::try_from(character))
            .collect::<Result<Vec<_>>>()?;
        let directions = Directions::new(directions_vec);

        let navigation_re = Regex::new(r"([\dA-Z]+)").unwrap();
        let nodes_vec = lines_iter
            .skip(1)
            .map(|line| {
                let (label, target_label_left, target_label_right) = navigation_re
                    .find_iter(line)
                    .map(|label| label.as_str().to_string())
                    .collect_tuple()?;
                Some((
                    label.clone(),
                    Node::new(label, target_label_left, target_label_right),
                ))
            })
            .collect::<Option<HashMap<_, _>>>()
            .ok_or(anyhow!("Cannot read nodes from file"))?;
        let network = Network::new(nodes_vec);

        Ok(NetworkWithDirections::new(network, directions))
    }
    fn walk_from_aaa_to_zzz_count(&self) -> u64 {
        let (_, walk_count) =
            self.network
                .walk_directions_until_end(&self.directions, &vec!["AAA"], &vec!["ZZZ"]);
        walk_count
    }
    fn walk_from_xxa_to_xxz_count(&self) -> u64 {
        let xxa_vec = self
            .network
            .get_nodes_ending_with_char('A')
            .into_iter()
            .map(|node| node.label.as_str())
            .collect_vec();
        let xxz_vec = self
            .network
            .get_nodes_ending_with_char('Z')
            .into_iter()
            .map(|node| node.label.as_str())
            .collect_vec();

        let start_labels_walk_count = xxa_vec
            .into_iter()
            .map(|label| {
                self.network
                    .walk_directions_until_end(&self.directions, &vec![label], &xxz_vec)
                    .1
            })
            .collect_vec();
        start_labels_walk_count.iter().collect_vec();

        let mut start_labels_walk_count_iter = start_labels_walk_count.into_iter();
        let first_walk_count = start_labels_walk_count_iter.next().unwrap();

        start_labels_walk_count_iter.fold(first_walk_count, |acc, walk_count| lcm(acc, walk_count))
    }
}

fn task1() {
    let filename = "input.txt";
    let network_with_directions = NetworkWithDirections::from_file(filename).unwrap();
    let walk_count = network_with_directions.walk_from_aaa_to_zzz_count();
    dbg!(walk_count);
}

fn task2() {
    let filename = "input.txt";
    let network_with_directions = NetworkWithDirections::from_file(filename).unwrap();
    let walk_count = network_with_directions.walk_from_xxa_to_xxz_count();
    dbg!(walk_count);
}

fn main() {
    task1();
    task2();
}
