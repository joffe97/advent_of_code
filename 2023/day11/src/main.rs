use itertools::Itertools;
use std::fs::read_to_string;

struct Coodinate {
    x: usize,
    y: usize,
}

impl Coodinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Galaxy {
    coordinate: Coodinate,
}

impl Galaxy {
    fn new(coordinate: Coodinate) -> Self {
        Self { coordinate }
    }
    fn calculate_length_to_galaxy(
        &self,
        galaxy: &Self,
        expanded_columns: &Vec<usize>,
        expanded_rows: &Vec<usize>,
        expand_size: u64,
    ) -> u64 {
        // let expanded_columns_sorted = expanded_columns.iter().sorted().collect::<Vec<_>>();
        // let expanded_rows_sorted = expanded_rows.iter().sorted().collect::<Vec<_>>();
        let (x_min, x_max) = vec![self.coordinate.x, galaxy.coordinate.x]
            .into_iter()
            .sorted()
            .collect_tuple()
            .unwrap();
        let (y_min, y_max) = vec![self.coordinate.y, galaxy.coordinate.y]
            .into_iter()
            .sorted()
            .collect_tuple()
            .unwrap();

        let expanded_columns_count = expanded_columns
            .iter()
            .filter_map(|column_index| {
                (x_min < *column_index && *column_index < x_max).then_some(column_index)
            })
            .collect_vec()
            .len();
        let expanded_rows_count = expanded_rows
            .iter()
            .filter_map(|row_index| (y_min < *row_index && *row_index < y_max).then_some(row_index))
            .collect_vec()
            .len();

        let expanded_columns_size = expanded_columns_count as u64 * (expand_size - 1);
        let expanded_rows_size = expanded_rows_count as u64 * (expand_size - 1);

        let x_diff = (x_max - x_min) as u64 + expanded_columns_size;
        let y_diff = (y_max - y_min) as u64 + expanded_rows_size;

        x_diff + y_diff
    }
}

struct Universe {
    galaxies: Vec<Vec<Option<Galaxy>>>,
}

impl Universe {
    fn new(galaxies: Vec<Vec<Option<Galaxy>>>) -> Self {
        Self { galaxies }
    }

    fn from_file(filename: &str) -> Self {
        let content = read_to_string(filename).unwrap();
        let galaxies = content
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, character)| {
                        (character == '#').then_some(Galaxy::new(Coodinate::new(x, y)))
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self::new(galaxies)
    }

    fn get_galaxies(&self) -> Vec<&Galaxy> {
        self.galaxies
            .iter()
            .flat_map(|row| row.into_iter().filter_map(|galaxy| galaxy.as_ref()))
            .collect::<Vec<_>>()
    }

    fn get_expanded_columns(&self) -> Vec<usize> {
        (0..self.galaxies.first().unwrap().len())
            .into_iter()
            .filter_map(|column_index| {
                if self.galaxies.iter().all(|row| row[column_index].is_none()) {
                    Some(column_index)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_expanded_rows(&self) -> Vec<usize> {
        self.galaxies
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                if row.iter().all(|galaxy| galaxy.is_none()) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_all_galaxy_pairs(&self) -> Vec<(&Galaxy, &Galaxy)> {
        let galaxies_vec = self.get_galaxies();
        (0..(galaxies_vec.len()))
            .flat_map(|i| {
                (0..galaxies_vec.len())
                    .filter_map(|j| {
                        if i > j {
                            Some((galaxies_vec[i], galaxies_vec[j]))
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            })
            .collect_vec()
    }

    fn find_shortest_paths_sum(&self, expand_size: u64) -> u64 {
        let galaxy_pairs = self.get_all_galaxy_pairs();
        let expanded_columns = self.get_expanded_columns();
        let expanded_rows = self.get_expanded_rows();
        galaxy_pairs
            .into_iter()
            .fold(0, |acc, (galaxy_a, galaxy_b)| {
                let galaxy_diff = galaxy_a.calculate_length_to_galaxy(
                    galaxy_b,
                    &expanded_columns,
                    &expanded_rows,
                    expand_size,
                );
                acc + galaxy_diff
            })
    }
}

fn task1() {
    let filename = "input.txt";
    let universe = Universe::from_file(filename);
    dbg!(universe.find_shortest_paths_sum(2));
}

fn task2() {
    let filename = "input.txt";
    let universe = Universe::from_file(filename);
    dbg!(universe.find_shortest_paths_sum(1_000_000));
}

fn main() {
    // task1();
    task2();
}
