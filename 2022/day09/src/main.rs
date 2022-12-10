use anyhow::{anyhow, Error, Result};
use std::{collections::HashSet, str::FromStr, vec};

enum Plane {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Rigth,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(match string {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Rigth,
            _ => return Err(anyhow!("invalid direction string")),
        })
    }
}

impl Direction {
    fn to_tuple(&self, size: isize) -> (isize, isize) {
        match self {
            Direction::Up => (0, -size),
            Direction::Down => (0, size),
            Direction::Left => (-size, 0),
            Direction::Rigth => (size, 0),
        }
    }
    fn directions_in_plane(plane: &Plane) -> Vec<Self> {
        match plane {
            Plane::Horizontal => vec![Self::Left, Self::Rigth],
            Plane::Vertical => vec![Self::Up, Self::Down],
        }
    }
    fn direction_in_plane(plane: &Plane, is_positive_direction: bool) -> Self {
        Self::directions_in_plane(plane)[is_positive_direction as usize].clone()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    fn origo() -> Self {
        Self::new(0, 0)
    }
    fn create_neighbour_in_direction(&self, direction: &Direction) -> Self {
        let (x_diff, y_diff) = direction.to_tuple(1);
        Self::new(self.x + x_diff, self.y + y_diff)
    }
    fn distance_to(&self, coordinate: &Coordinate) -> (isize, isize) {
        (coordinate.x - self.x, coordinate.y - self.y)
    }
    fn is_neighbour(&self, coordinate: &Coordinate) -> bool {
        let (x_dist, y_dist) = self.distance_to(coordinate);
        x_dist.abs() <= 1 && y_dist.abs() <= 1
    }
}

#[derive(Clone)]
struct Knot {
    coordinate: Coordinate,
    visited_coordinates: HashSet<Coordinate>,
}

impl Knot {
    fn new(coordinate: Coordinate) -> Self {
        let visited_coordinates = HashSet::from([coordinate.clone()]);
        Self {
            coordinate,
            visited_coordinates,
        }
    }
    fn new_at_origo() -> Self {
        Self::new(Coordinate::origo())
    }
    fn update_coordinate(&mut self, coordinate: Coordinate) {
        self.visited_coordinates.insert(coordinate.clone());
        self.coordinate = coordinate;
    }
    fn move_in_direction(&mut self, direction: &Direction) {
        let new_coordinate = self.coordinate.create_neighbour_in_direction(direction);
        self.update_coordinate(new_coordinate);
    }
    fn move_in_directions_at_once(&mut self, directions: &[&Direction]) {
        let mut cur_coord = self.coordinate.clone();
        directions.into_iter().for_each(|direction| {
            cur_coord = cur_coord.create_neighbour_in_direction(direction);
        });
        self.update_coordinate(cur_coord);
    }
    fn move_towards_coordinate(&mut self, coordinate: &Coordinate) {
        if self.coordinate.is_neighbour(coordinate) {
            return;
        }
        let (x_dist, y_dist) = self.coordinate.distance_to(coordinate);

        let directions = [(x_dist, Plane::Horizontal), (y_dist, Plane::Vertical)]
            .into_iter()
            .filter_map(|(distance, plane)| match distance.cmp(&0) {
                std::cmp::Ordering::Less => Some(Direction::direction_in_plane(&plane, false)),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(Direction::direction_in_plane(&plane, true)),
            })
            .collect::<Vec<Direction>>();
        self.move_in_directions_at_once(&directions.iter().collect::<Vec<_>>())
    }
    fn move_towards_knot(&mut self, knot: &Self) {
        self.move_towards_coordinate(&knot.coordinate)
    }
}

struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    fn new_at_origo(knot_count: usize) -> Self {
        assert!(knot_count >= 1, "cannot create rope with size less than 1");
        let knots = vec![Knot::new_at_origo(); knot_count];
        Self { knots }
    }
    fn head(&mut self) -> &mut Knot {
        &mut self.knots[0]
    }
    fn tail(&mut self) -> &mut Knot {
        let knot_count = self.knots.len();
        &mut self.knots[knot_count - 1]
    }
    fn perform_motion(&mut self, head_knot_motion: &HeadKnotMotion) {
        for _ in 0..head_knot_motion.movement_count {
            self.head().move_in_direction(&head_knot_motion.direction);
            for i in 1..self.knots.len() {
                let parent_knot_coordinate = &self.knots[i - 1].coordinate.clone();
                self.knots[i].move_towards_coordinate(parent_knot_coordinate);
            }
        }
    }
    fn perform_motions(&mut self, head_knot_motions: &HeadKnotMotions) {
        for motion in head_knot_motions.motions.iter() {
            self.perform_motion(motion);
        }
    }
}

struct HeadKnotMotion {
    direction: Direction,
    movement_count: isize,
}

impl HeadKnotMotion {
    fn new(direction: Direction, movement_count: isize) -> Self {
        Self {
            direction,
            movement_count,
        }
    }
}

struct HeadKnotMotions {
    motions: Vec<HeadKnotMotion>,
}

impl HeadKnotMotions {
    fn try_from_filename(filename: &str) -> Result<Self> {
        let content = std::fs::read_to_string(filename)?;
        let motions = content
            .lines()
            .filter_map(|line| {
                if let [direction_str, movement_count_str] =
                    line.split_whitespace().collect::<Vec<_>>().as_slice()
                {
                    let direction = Direction::from_str(direction_str).ok()?;
                    let movement_count = movement_count_str.parse().ok()?;
                    return Some(HeadKnotMotion::new(direction, movement_count));
                }
                None
            })
            .collect::<Vec<_>>();
        Ok(Self { motions })
    }
}

fn main() -> Result<()> {
    let head_knot_motions = HeadKnotMotions::try_from_filename("input.txt")?;
    let mut rope = Rope::new_at_origo(10);
    rope.perform_motions(&head_knot_motions);
    dbg!(rope.tail().visited_coordinates.len());
    Ok(())
}
