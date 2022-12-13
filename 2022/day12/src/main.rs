use anyhow::{anyhow, Result};
use colored::Colorize;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn to_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    value: char,
    coordinate: Coordinate,
}

impl Node {
    fn new(value: char, coord: Coordinate) -> Self {
        Self {
            value,
            coordinate: coord,
        }
    }
    fn is_start(&self) -> bool {
        self.value == 'S'
    }
    fn is_end(&self) -> bool {
        self.value == 'E'
    }
    fn is_low_point(&self) -> bool {
        self.height_value() == 'a'
    }
    fn height_value(&self) -> char {
        match self.value {
            'S' => 'a',
            'E' => 'z',
            chr @ _ => chr,
        }
    }
    fn to_u8(&self) -> u8 {
        self.height_value() as u8 - 'a' as u8
    }
    fn is_walkable(&self, node: &Node) -> bool {
        (node.to_u8() as i8 - self.to_u8() as i8) <= 1
    }
}

struct Matrix {
    node_matrix: Vec<Vec<Node>>,
    start_coord: Coordinate,
    end_coord: Coordinate,
}

impl Matrix {
    fn new(node_matrix: Vec<Vec<Node>>, start_coord: Coordinate, end_coord: Coordinate) -> Self {
        Self {
            node_matrix,
            start_coord,
            end_coord,
        }
    }
    fn try_from_filename(filename: &str) -> Result<Self> {
        let content = std::fs::read_to_string(filename)?;
        let mut start_coord = None;
        let mut end_coord = None;
        let matrix = content
            .lines()
            .enumerate()
            .map(|(y_index, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x_index, value)| {
                        let coord = Coordinate::new(x_index, y_index);
                        let node = Node::new(value, coord);
                        if node.is_start() {
                            start_coord = Some(node.coordinate.clone())
                        } else if node.is_end() {
                            end_coord = Some(node.coordinate.clone())
                        }
                        node
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();
        match (start_coord, end_coord) {
            (Some(start_coord_some), Some(end_coord_some)) => {
                Ok(Self::new(matrix, start_coord_some, end_coord_some))
            }
            _ => Err(anyhow!("cannot find start or end")),
        }
    }
    fn x_len(&self) -> usize {
        self.node_matrix[0].len()
    }
    fn y_len(&self) -> usize {
        self.node_matrix.len()
    }
    fn get_node(&self, x: usize, y: usize) -> Option<&Node> {
        self.node_matrix.get(y).and_then(|row| row.get(x))
    }
    fn get_node_for_coordinate(&self, coord: &Coordinate) -> Option<&Node> {
        let (x, y) = coord.to_tuple();
        self.get_node(x, y)
    }
    fn get_start_node(&self) -> &Node {
        self.get_node_for_coordinate(&self.start_coord)
            .expect("start node must exist")
    }
    fn get_end_node(&self) -> &Node {
        self.get_node_for_coordinate(&self.end_coord)
            .expect("end node must exist")
    }
    fn get_edges_for_node(&self, node: &Node) -> Vec<&Node> {
        let (x, y) = node.coordinate.to_tuple();
        let edges = [(0, -1), (0, 1), (-1, 0), (1, 0)]
            .into_iter()
            .filter_map(|(x_diff, y_diff)| {
                let cur_x = usize::try_from(x as isize + x_diff).ok()?;
                let cur_y = usize::try_from(y as isize + y_diff).ok()?;
                let edge_node = self.get_node(cur_x, cur_y)?;
                node.is_walkable(edge_node).then_some(edge_node)
            })
            .collect();
        edges
    }
    fn get_unvisited_edges_for_node(
        &self,
        node: &Node,
        visited_nodes: &HashSet<Node>,
    ) -> Vec<&Node> {
        self.get_edges_for_node(node)
            .into_iter()
            .filter(|edge| !visited_nodes.contains(edge))
            .collect()
    }
    fn find_path_from_node_to_end_inner(
        &self,
        node: &Node,
        visited_nodes: &mut HashSet<Node>,
    ) -> Option<Vec<Node>> {
        visited_nodes.insert(node.clone());

        let mut cur_paths = vec![vec![node.clone()]];

        while !cur_paths.is_empty() {
            let mut next_paths = vec![];
            for cur_path in cur_paths {
                let last_node = cur_path.last().expect("vector cannot be empty");
                let edges = self.get_unvisited_edges_for_node(last_node, &visited_nodes);
                for edge in edges.into_iter().cloned() {
                    visited_nodes.insert(edge.clone());
                    let edge_is_end = edge.is_end();
                    let cur_path_cloned: Vec<Node> = cur_path.iter().cloned().collect::<Vec<_>>();
                    let path_with_edge = vec![cur_path_cloned, vec![edge]].concat();
                    if edge_is_end {
                        return Some(path_with_edge);
                    }
                    next_paths.push(path_with_edge);
                }
            }
            cur_paths = next_paths;
        }
        None
    }
    fn find_path_from_node_to_end(&self, node: &Node) -> Option<Vec<Node>> {
        let mut visited_nodes = HashSet::new();
        let path = self.find_path_from_node_to_end_inner(node, &mut visited_nodes);
        // self.print_with_visited_nodes(&visited_nodes);
        path
    }
    fn find_path_from_start_to_end(&self) -> Option<Vec<Node>> {
        let start_node = self.get_start_node();
        self.find_path_from_node_to_end(start_node)
    }
    fn find_all_low_nodes(&self) -> Vec<&Node> {
        self.node_matrix
            .iter()
            .flatten()
            .filter(|node| node.is_low_point())
            .collect()
    }
    fn print_with_visited_nodes(&self, visited_nodes: &HashSet<Node>) {
        for row in &self.node_matrix {
            for node in row {
                if visited_nodes.contains(node) {
                    print!("{}", node.value.to_string().cyan());
                } else {
                    print!("{}", node.value);
                }
            }
            println!();
        }
    }
    fn task1(&self) -> Option<usize> {
        let path = self.find_path_from_start_to_end()?;
        Some(path.len() - 1)
    }
    fn task2(&self) -> Option<usize> {
        Some(
            self.find_all_low_nodes()
                .iter()
                .filter_map(|node| self.find_path_from_node_to_end(node))
                .map(|path| path.len())
                .min()?
                - 1,
        )
    }
}

fn main() -> Result<()> {
    let matrix = Matrix::try_from_filename("input.txt")?;
    dbg!(matrix.task1());
    dbg!(matrix.task2());
    Ok(())
}
