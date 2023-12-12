use std::{collections::HashMap, ops::Range};

use regex::Regex;

fn main() {
    // task1();
    task2();
}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Category {
    value: String,
}

impl Category {
    fn new(value: String) -> Self {
        Self { value }
    }
}

struct Mapping {
    start_a: u64,
    start_b: u64,
    range_size: u64,
}

impl Mapping {
    fn new(start_a: u64, start_b: u64, range_size: u64) -> Self {
        Self {
            start_a,
            start_b,
            range_size,
        }
    }
    fn get_end_a(&self) -> u64 {
        self.start_a + self.range_size
    }
    fn get_end_b(&self) -> u64 {
        self.start_b + self.range_size
    }
    fn get_range_a(&self) -> Range<u64> {
        self.start_a..self.get_end_a()
    }
    fn get_range_b(&self) -> Range<u64> {
        self.start_b..self.get_end_b()
    }
    fn get_value_a_from_b(&self, b: u64) -> Option<u64> {
        if self.get_range_b().contains(&b) {
            Some(self.start_a + (b - self.start_b))
        } else {
            None
        }
    }
    fn get_value_b_from_a(&self, a: u64) -> Option<u64> {
        if self.get_range_a().contains(&a) {
            Some(self.start_b + (a - self.start_a))
        } else {
            None
        }
    }
}

struct CategoryMappings {
    mappings: HashMap<(Category, Category), Vec<Mapping>>,
}

impl CategoryMappings {
    fn new(mappings: HashMap<(Category, Category), Vec<Mapping>>) -> Self {
        Self { mappings }
    }
    fn get_mappings(&self, category_a: &Category, category_b: &Category) -> Option<&Vec<Mapping>> {
        let mut categories = [category_a, category_b];
        categories.sort();
        let categories_sorted_tup = (categories[0].clone(), categories[1].clone());
        self.mappings.get(&categories_sorted_tup)
    }
    fn get_mapped_value(
        &self,
        category_a: &Category,
        category_b: &Category,
        category_a_value: u64,
    ) -> u64 {
        let mut categories = [category_a, category_b];
        categories.sort();
        let categories_sorted_tup = (categories[0].clone(), categories[1].clone());
        let mappings = self.mappings.get(&categories_sorted_tup).unwrap();

        let is_switched = categories[0] != &categories_sorted_tup.0;
        for mapping in mappings {
            let value_option = if is_switched {
                mapping.get_value_b_from_a(category_a_value)
            } else {
                mapping.get_value_a_from_b(category_a_value)
            };
            if let Some(value) = value_option {
                return value;
            }
        }
        category_a_value
    }
    fn get_mapped_value_multi_categories(&self, categories: &[Category], start_value: u64) -> u64 {
        categories
            .windows(2)
            .fold(start_value, |category_a_value, categories: &[Category]| {
                self.get_mapped_value(&categories[0], &categories[1], category_a_value)
            })
    }
    fn get_mapped_ranges(
        &self,
        category_a: &Category,
        category_b: &Category,
        category_a_range: Range<u64>,
    ) -> Vec<Range<u64>> {
        let mut categories = [category_a, category_b];
        categories.sort();
        let categories_sorted_tup = (categories[0].clone(), categories[1].clone());
        let mappings = self.mappings.get(&categories_sorted_tup).unwrap();
        let is_switched = categories[0] != &categories_sorted_tup.0;

        let mut matched_ranges = vec![];
        let mut unmatched_ranges = vec![category_a_range];
        while let Some(current_range) = unmatched_ranges.pop() {
            let mut handled_range = false;
            for mapping in mappings {
                let start_range_option = if is_switched {
                    mapping
                        .get_value_b_from_a(current_range.start)
                        .and_then(|start_value| Some(start_value..mapping.get_end_a()))
                } else {
                    mapping
                        .get_value_a_from_b(current_range.start)
                        .and_then(|start_value| Some(start_value..mapping.get_end_b()))
                };
                if let Some(start_range) = start_range_option {
                    if start_range.end < current_range.end {
                        unmatched_ranges.push((start_range.end + 1)..current_range.end);
                    }
                    matched_ranges.push(start_range);
                    handled_range = true;
                    break;
                }

                let end_range_option = if is_switched {
                    mapping
                        .get_value_b_from_a(current_range.end)
                        .and_then(|end_value| Some(mapping.start_a..end_value))
                } else {
                    mapping
                        .get_value_a_from_b(current_range.end)
                        .and_then(|end_value| Some(mapping.start_b..end_value))
                };
                if let Some(end_range) = end_range_option {
                    if current_range.start < end_range.start {
                        unmatched_ranges.push(current_range.start..(end_range.start + 1));
                    }
                    matched_ranges.push(end_range);
                    handled_range = true;
                    break;
                }
            }
            if !handled_range {
                matched_ranges.push(current_range)
            }
        }
        matched_ranges
    }
    fn get_mapped_ranges_multi_categories(
        &self,
        categories: &[Category],
        start_range: Range<u64>,
    ) -> Vec<Range<u64>> {
        categories.windows(2).fold(
            vec![start_range],
            |category_a_ranges, categories: &[Category]| {
                category_a_ranges
                    .into_iter()
                    .flat_map(|category_a_range| {
                        self.get_mapped_ranges(&categories[0], &categories[1], category_a_range)
                    })
                    .collect::<Vec<_>>()
            },
        )
    }
}

struct AlmanacRanged {
    seeds: Vec<Range<u64>>,
    category_mappings: CategoryMappings,
}

impl AlmanacRanged {
    fn new(seeds: Vec<Range<u64>>, category_mappings: CategoryMappings) -> Self {
        Self {
            seeds,
            category_mappings,
        }
    }

    fn from_file(filename: &str) -> Self {
        let content = std::fs::read_to_string(filename).unwrap();
        let mut content_lines_iter = content.lines().into_iter();
        let seeds = content_lines_iter
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split(" ")
            .map(|numstr| numstr.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|numbers| numbers[0].clone()..(numbers[0] + numbers[1]))
            .collect::<Vec<_>>();
        let re = Regex::new(r"(.+)-to-(.+) map:").unwrap();
        let mut mappings_vec: Vec<((Category, Category), Vec<Mapping>)> = vec![];
        for line in content_lines_iter {
            match re.captures(line) {
                Some(capture) => {
                    let mut capture_strs = (1..=2)
                        .into_iter()
                        .map(|i| capture.get(i).unwrap().as_str())
                        .collect::<Vec<_>>();
                    capture_strs.sort();
                    let captures = capture_strs
                        .into_iter()
                        .map(|capture_str| Category::new(String::from(capture_str)))
                        .collect::<Vec<_>>();
                    let category_a = captures[0].clone();
                    let category_b = captures[1].clone();
                    let categories = (category_a, category_b);
                    mappings_vec.push((categories, vec![]));
                }
                None if !line.is_empty() => {
                    let category = mappings_vec.last_mut().unwrap();
                    let numbers = line
                        .split(" ")
                        .into_iter()
                        .map(|numstr| numstr.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    let mapping = Mapping::new(numbers[0], numbers[1], numbers[2]);
                    category.1.push(mapping)
                }
                _ => {}
            }
        }
        let mappings_hash_map = HashMap::from_iter(mappings_vec.into_iter());
        Self::new(seeds, CategoryMappings::new(mappings_hash_map))
    }
    fn get_mappings(&self) -> u64 {
        let categories = [
            "seed",
            "soil",
            "fertilizer",
            "water",
            "light",
            "temperature",
            "humidity",
            "location",
        ]
        .into_iter()
        .map(|category_str| Category::new(category_str.to_string()))
        .collect::<Vec<_>>();

        let a = self
            .seeds
            .iter()
            .flat_map(|seed| {
                self.category_mappings
                    .get_mapped_ranges_multi_categories(&categories, seed.clone())
            })
            .collect::<Vec<_>>();
        todo!()
    }
}

struct Almanac {
    seeds: Vec<u64>,
    category_mappings: CategoryMappings,
}

impl Almanac {
    fn new(seeds: Vec<u64>, category_mappings: CategoryMappings) -> Self {
        Self {
            seeds,
            category_mappings,
        }
    }
    fn from_file(filename: &str) -> Self {
        let content = std::fs::read_to_string(filename).unwrap();
        let mut content_lines_iter = content.lines().into_iter();
        let seeds = content_lines_iter
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split(" ")
            .map(|numstr| numstr.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let re = Regex::new(r"(.+)-to-(.+) map:").unwrap();
        let mut mappings_vec: Vec<((Category, Category), Vec<Mapping>)> = vec![];
        for line in content_lines_iter {
            match re.captures(line) {
                Some(capture) => {
                    let mut capture_strs = (1..=2)
                        .into_iter()
                        .map(|i| capture.get(i).unwrap().as_str())
                        .collect::<Vec<_>>();
                    capture_strs.sort();
                    let captures = capture_strs
                        .into_iter()
                        .map(|capture_str| Category::new(String::from(capture_str)))
                        .collect::<Vec<_>>();
                    let category_a = captures[0].clone();
                    let category_b = captures[1].clone();
                    let categories = (category_a, category_b);
                    mappings_vec.push((categories, vec![]));
                }
                None if !line.is_empty() => {
                    let category = mappings_vec.last_mut().unwrap();
                    let numbers = line
                        .split(" ")
                        .into_iter()
                        .map(|numstr| numstr.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    let mapping = Mapping::new(numbers[0], numbers[1], numbers[2]);
                    category.1.push(mapping)
                }
                _ => {}
            }
        }
        let mappings_hash_map = HashMap::from_iter(mappings_vec.into_iter());
        Self::new(seeds, CategoryMappings::new(mappings_hash_map))
    }
    fn get_mappings(&self) -> u64 {
        let categories = [
            "seed",
            "soil",
            "fertilizer",
            "water",
            "light",
            "temperature",
            "humidity",
            "location",
        ]
        .into_iter()
        .map(|category_str| Category::new(category_str.to_string()))
        .collect::<Vec<_>>();

        self.seeds
            .iter()
            .map(|seed| {
                self.category_mappings
                    .get_mapped_value_multi_categories(&categories, seed.clone())
            })
            .min()
            .unwrap()
    }
}

fn task1() {
    let almanac = Almanac::from_file("input1.txt");
    dbg!(almanac.get_mappings());
}

fn task2() {
    let almanac = AlmanacRanged::from_file("input1_test.txt");
    dbg!(almanac.get_mappings());
}
