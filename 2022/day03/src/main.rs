use std::collections::{hash_map::RandomState, HashMap, HashSet};

use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Item {
    item_type: char,
}

impl Item {
    fn new(item_type: char) -> Self {
        Self { item_type }
    }
    fn points(&self) -> u32 {
        let mut item_type_lowercase = self.item_type.clone();
        item_type_lowercase.make_ascii_lowercase();
        item_type_lowercase as u32 - 'a' as u32 + self.item_type.is_uppercase() as u32 * 26 + 1
    }
}

#[derive(Clone)]
struct Compartment {
    items: Vec<Item>,
}

impl Compartment {
    fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
    fn from_char(str: &str) -> Self {
        Self::new(str.chars().into_iter().map(|chr| Item::new(chr)).collect())
    }
    fn contains_item(&self, item: &Item) -> bool {
        self.items.contains(item)
    }
}

#[derive(Clone)]
struct Rucksack {
    compartments: Vec<Compartment>,
}

impl Rucksack {
    fn new(compartments: Vec<Compartment>) -> Self {
        Self { compartments }
    }
    fn from_item_str(rucksack_str: &str, compartment_count: usize) -> Self {
        let mut compartment_strs = vec![];
        let mut remaining_rucksack_str = rucksack_str.clone();
        let compartment_size = rucksack_str.len() / compartment_count;
        while remaining_rucksack_str.len() >= compartment_size * 2 {
            let cur_rucksack_str;
            (cur_rucksack_str, remaining_rucksack_str) =
                remaining_rucksack_str.split_at(compartment_size);
            compartment_strs.push(cur_rucksack_str);
        }
        compartment_strs.push(remaining_rucksack_str);

        let compartments = compartment_strs
            .into_iter()
            .map(|str| Compartment::from_char(str))
            .collect();
        Self::new(compartments)
    }
    fn common_items(&self) -> Vec<&Item> {
        let mut common_items: HashMap<&Item, u32> = HashMap::new();
        for compartment in self.compartments.iter() {
            let hashset = HashSet::<_, RandomState>::from_iter(compartment.items.iter());
            for item in hashset.into_iter() {
                common_items
                    .entry(item)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
        common_items
            .into_iter()
            .filter_map(|(key, value)| (value >= self.compartments.len() as u32).then_some(key))
            .collect()
    }
    fn all_items(&self) -> Vec<&Item> {
        self.compartments
            .iter()
            .flat_map(|compartment| &compartment.items)
            .collect()
    }
}

struct RucksackCollection {
    rucksack_collection: Vec<Rucksack>,
}

impl RucksackCollection {
    fn new(rucksack_collection: Vec<Rucksack>) -> Self {
        Self {
            rucksack_collection,
        }
    }
    fn from_rucksack_collection_strs(
        rucksack_collection_strs: Vec<&str>,
        compartments_per_rucksack: usize,
    ) -> Self {
        Self::new(
            rucksack_collection_strs
                .into_iter()
                .map(|str| Rucksack::from_item_str(str, compartments_per_rucksack))
                .collect(),
        )
    }
    fn from_filename(filename: &str, compartments_per_rucksack: usize) -> Result<Self> {
        Ok(Self::from_rucksack_collection_strs(
            std::fs::read_to_string(filename)?.lines().collect(),
            compartments_per_rucksack,
        ))
    }
    fn common_item_points(&self) -> u32 {
        let common_items = self
            .rucksack_collection
            .iter()
            .map(|rucksack| rucksack.common_items())
            .collect::<Vec<_>>();
        common_items.iter().fold(0, |acc, cur_vec| {
            acc + cur_vec
                .into_iter()
                .fold(0, |acc_inner, cur_item| acc_inner + cur_item.points())
        })
    }
    fn into_group_collection(&self, group_size: usize) -> GroupCollection {
        let groups = self
            .rucksack_collection
            .iter()
            .as_slice()
            .chunks(group_size)
            .map(|rucksacks| Group::from_rucksacks(rucksacks))
            .collect::<Vec<_>>();
        GroupCollection::new(groups)
    }
}

struct Group {
    rucksack_collection: RucksackCollection,
}

impl Group {
    fn new(rucksack_collection: RucksackCollection) -> Self {
        Self {
            rucksack_collection,
        }
    }
    fn from_rucksacks(rucksacks: &[Rucksack]) -> Self {
        let rucksack_collection = RucksackCollection::new(rucksacks.to_vec());
        Self::new(rucksack_collection)
    }
    fn common_rucksack_item(&self) -> Item {
        let item_collections = self
            .rucksack_collection
            .rucksack_collection
            .iter()
            .map(|rucksacks| Compartment::new(rucksacks.all_items().into_iter().cloned().collect()))
            .collect();
        let rucksack = Rucksack::new(item_collections);
        let common_items = rucksack.common_items();
        common_items[0].clone()
    }
}

struct GroupCollection {
    groups: Vec<Group>,
}

impl GroupCollection {
    fn new(groups: Vec<Group>) -> Self {
        Self { groups }
    }
    fn common_item_points(&self) -> u32 {
        self.groups
            .iter()
            .fold(0, |acc, cur| acc + cur.common_rucksack_item().points())
    }
}

fn main() -> Result<()> {
    let rucksack_collection = RucksackCollection::from_filename("input.txt", 2)?;
    let group_collection = rucksack_collection.into_group_collection(3);
    dbg!(group_collection.common_item_points());
    Ok(())
}
