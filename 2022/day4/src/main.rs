#![feature(is_sorted)]
use anyhow::Result;
use std::ops::RangeInclusive;

fn range_contains_fully(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.start() <= b.start() && b.end() <= a.end()
}

fn range_contains_any(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.start() <= b.start() && b.start() <= a.end()
}

fn ranges_contains_eachother(
    ranges: &[RangeInclusive<u32>],
    func: fn(a: &RangeInclusive<u32>, &RangeInclusive<u32>) -> bool,
) -> bool {
    ranges.iter().enumerate().any(|(i, range)| {
        ranges
            .iter()
            .enumerate()
            .any(|(j, range2)| i != j && (func(range, range2)))
    })
}

fn main() -> Result<()> {
    let s = std::fs::read_to_string("input.txt")?
        .lines()
        .into_iter()
        .fold(0, |acc, cur_line| {
            let ranges = cur_line
                .split(",")
                .map(|str| {
                    let numbers = str
                        .split("-")
                        .map(|num_str| num_str.parse::<u32>().unwrap())
                        .collect::<Vec<_>>();
                    numbers[0]..=numbers[1]
                })
                .collect::<Vec<_>>();
            acc + ranges_contains_eachother(ranges.as_slice(), range_contains_any) as u32
        });
    dbg!(s);
    Ok(())
}
