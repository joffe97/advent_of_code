use std::{path::PathBuf, vec};

use anyhow::Result;

struct Content {
    groups: Vec<Vec<u32>>,
}

impl Content {
    fn read(filename: &str) -> Result<Self> {
        let content = std::fs::read_to_string(PathBuf::from(filename).as_path())?;
        let mut groups = vec![vec![]];
        for line in content.split("\n") {
            match line.parse::<u32>() {
                Ok(num) => groups.last_mut().unwrap().push(num),
                Err(_) => groups.push(vec![]),
            }
        }
        Ok(Self { groups })
    }
}

#[derive(Clone)]
struct Inventory {
    inventory: Vec<u32>,
}

impl Inventory {
    fn new(inventory: Vec<u32>) -> Self {
        Self { inventory }
    }
    fn total_calories(&self) -> u32 {
        self.inventory.iter().sum()
    }
}

struct Inventories {
    inventories: Vec<Inventory>,
}

impl Inventories {
    fn new(inventories: Vec<Inventory>) -> Self {
        Self { inventories }
    }
    fn from_content(content: Content) -> Self {
        let inventories = content
            .groups
            .into_iter()
            .map(|inventory_vec| Inventory::new(inventory_vec))
            .collect();
        Self::new(inventories)
    }
    fn biggest_inventory(&self) -> &Inventory {
        self.inventories
            .iter()
            .max_by_key(|inventory| inventory.total_calories())
            .unwrap()
    }
    fn n_biggest_inventories(&self, n: usize) -> Self {
        let n_empty_inventories = vec![Inventory::new(vec![]); n];
        let mut found_inventories = n_empty_inventories.iter().collect::<Vec<_>>();
        for current_inventory in self.inventories.iter() {
            let smallest_found_inventory = found_inventories[0];
            if current_inventory.total_calories() > smallest_found_inventory.total_calories() {
                found_inventories[0] = current_inventory;
                found_inventories.sort_by_key(|inventory| inventory.total_calories());
            }
        }
        Inventories::new(found_inventories.into_iter().cloned().collect())
    }
    fn total_calories(&self) -> u32 {
        self.inventories
            .iter()
            .fold(0, |acc, inventory| acc + inventory.total_calories())
    }
}

fn main() -> Result<()> {
    let content = Content::read("1.txt")?;
    let inventories = Inventories::from_content(content);
    let n_biggest_inventores = inventories.n_biggest_inventories(3);
    dbg!(n_biggest_inventores.total_calories());
    Ok(())
}
