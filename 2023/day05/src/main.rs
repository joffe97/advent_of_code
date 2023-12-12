use std::{fs::read_to_string, ops::RangeInclusive};

use itertools::Itertools;

fn main() {
    solution2()
}

fn solution2() {
    let filename = "input1.txt";

    let file_content = read_to_string(filename).unwrap();
    let mut file_line_iter = file_content.lines().into_iter();

    let seed_ranges = file_line_iter
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|numstr| numstr.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| chunk[0]..=(chunk[0] + chunk[1] - 1))
        .collect::<Vec<_>>();
    // let seed_ranges = vec![79..=79, 14..=14, 55..=55, 13..=13];

    let maps = file_line_iter.fold(vec![], |mut maps_vec, line| {
        if line.is_empty() {
        } else if line.ends_with("map:") {
            maps_vec.push(vec![]);
        } else {
            let numbers_line = line
                .split_whitespace()
                .map(|numstr| numstr.parse::<u64>().unwrap())
                .collect_tuple::<(u64, u64, u64)>()
                .unwrap();
            maps_vec.last_mut().unwrap().push(numbers_line)
        }
        maps_vec
    });

    let min_vec = seed_ranges
        .iter()
        .map(|seed_range| {
            let mut queue = vec![seed_range.clone()];
            maps.iter().for_each(|cur_maps| {
                let mut finished = vec![];
                'outer: while let Some(range) = queue.pop() {
                    for cur_map in cur_maps {
                        let (overlapping_range, non_overlapping_range): (
                            RangeInclusive<u64>,
                            Option<RangeInclusive<u64>>,
                        ) = if cur_map.1 <= *range.start()
                            && *range.end() <= (cur_map.1 + cur_map.2)
                        {
                            let overlapping_range = range.clone();
                            let non_overlapping_range = None;
                            (overlapping_range, non_overlapping_range)
                        } else if cur_map.1 <= *range.start()
                            && *range.start() <= (cur_map.1 + cur_map.2)
                        {
                            let overlapping_range = (*range.start() - 1)..=(cur_map.1 + cur_map.2);
                            let non_overlapping_range =
                                Some((cur_map.1 + cur_map.2 + 1)..=(*range.end() - 1));
                            (overlapping_range, non_overlapping_range)
                        } else if cur_map.1 <= *range.end()
                            && *range.end() <= (cur_map.1 + cur_map.2)
                        {
                            let overlapping_range = cur_map.1..=*range.end();
                            let non_overlapping_range = Some(*range.start()..=(cur_map.1 - 1));
                            (overlapping_range, non_overlapping_range)
                        } else {
                            continue;
                        };

                        let mapping_diff = cur_map.0 as i64 - cur_map.1 as i64;
                        let mapped_overlapping_range_i64 = (*overlapping_range.start() as i64
                            + mapping_diff)
                            ..=(*overlapping_range.end() as i64 + mapping_diff);
                        let mapped_overlapping_range = mapped_overlapping_range_i64
                            .start()
                            .clone()
                            .try_into()
                            .unwrap()
                            ..=mapped_overlapping_range_i64
                                .end()
                                .clone()
                                .try_into()
                                .unwrap();

                        finished.push(mapped_overlapping_range);
                        if let Some(non_overlapping_range_some) = non_overlapping_range {
                            queue.push(non_overlapping_range_some.clone());
                        }
                        continue 'outer;
                    }
                    finished.push(range)
                }
                queue = finished;
            });
            queue
                .into_iter()
                .map(|range| range.start().clone())
                .min()
                .unwrap()
        })
        .collect::<Vec<_>>();
    let minimum = min_vec.iter().min();
    dbg!(minimum);
    todo!()
}
