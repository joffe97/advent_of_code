use anyhow::Result;
use std::{
    collections::{hash_map::RandomState, HashSet},
    fs,
};

struct Signal {
    signal: String,
}

impl Signal {
    fn new(signal: String) -> Self {
        Self { signal }
    }
    fn find_first_maker_index(&self, chunk_size: usize) -> Option<usize> {
        for (end_i, _) in self.signal.char_indices() {
            let i_diff = chunk_size - 1;
            if end_i < i_diff {
                continue;
            }
            let start_i = end_i - i_diff;
            let chunk = self.signal[start_i..=end_i].chars();
            if HashSet::<_, RandomState>::from_iter(chunk.into_iter()).len() == chunk_size {
                return Some(end_i + 1);
            }
        }
        None
    }
}

fn read_file(filename: &str) -> Result<Signal> {
    let content = fs::read_to_string(filename)?;
    Ok(Signal::new(content))
}

fn main() -> Result<()> {
    let signal = read_file("input.txt")?;
    dbg!(signal.find_first_maker_index(14));
    Ok(())
}
