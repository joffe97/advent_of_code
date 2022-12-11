use anyhow::{anyhow, Error, Result};
use regex::Regex;
use std::{fmt::format, str::FromStr};

#[derive(Clone)]
enum OperationValue {
    Old,
    Num(u64),
}

impl OperationValue {
    fn to_num(&self, old_value: u64) -> u64 {
        match self {
            OperationValue::Old => old_value,
            OperationValue::Num(num) => num.clone(),
        }
    }
}

impl FromStr for OperationValue {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self> {
        Ok(match string {
            "old" => Self::Old,
            _ => Self::Num(string.parse()?),
        })
    }
}

#[derive(Clone)]
struct Item {
    worry_level: u64,
}

impl Item {
    fn new(worry_level: u64) -> Self {
        Self { worry_level }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: fn(u64, u64) -> u64,
    operation_value_a: OperationValue,
    operation_value_b: OperationValue,
    divisible_denominator: u64,
    monkey_true: u64,
    monkey_false: u64,
    bored_denominator: u64,
    inspections: u64,
}

impl Monkey {
    fn new(
        items: Vec<Item>,
        operation: fn(u64, u64) -> u64,
        operation_value_a: OperationValue,
        operation_value_b: OperationValue,
        divisible_denominator: u64,
        monkey_true: u64,
        monkey_false: u64,
        bored_denominator: u64,
    ) -> Self {
        let inspections = 0;
        Self {
            items,
            operation,
            operation_value_a,
            operation_value_b,
            divisible_denominator,
            monkey_true,
            monkey_false,
            bored_denominator,
            inspections,
        }
    }
    fn perform_operation(&self, item: &Item) -> u64 {
        let old_value = item.worry_level;
        let a_value = self.operation_value_a.to_num(old_value);
        let b_value = self.operation_value_b.to_num(old_value);
        (self.operation)(a_value, b_value)
    }
    fn calculate_receiver_monkey_for_item(&self, item: &Item) -> (u64, Item) {
        let operation_value = self.perform_operation(item);
        let bored_value = operation_value / self.bored_denominator;
        let receiver_index = if bored_value % self.divisible_denominator == 0 {
            self.monkey_true
        } else {
            self.monkey_false
        };
        let new_item = Item::new(bored_value);
        (receiver_index, new_item)
    }
    fn throw_next_item(&mut self) -> Option<(u64, Item, Self)> {
        if self.items.is_empty() {
            return None;
        }
        self.inspections += 1;
        let mut self_clone = self.clone();
        let item = self_clone.items.remove(0);
        let (receiver_monkey, item) = self_clone.calculate_receiver_monkey_for_item(&item);
        Some((receiver_monkey, item, self_clone))
    }
    fn add_item(&mut self, item: Item) {
        self.items.push(item)
    }
    fn from_str(string: &str, bored_denominator: u64) -> Result<Self> {
        let starting_items_regex = Regex::new(r"Starting items: ((?:.(?:, )?)*)\n")?;
        let starting_items_capture = starting_items_regex
            .captures(string)
            .and_then(|captures| captures.get(1))
            .ok_or(anyhow!("cannot find starting items"))?;
        let items = starting_items_capture
            .as_str()
            .split(", ")
            .map(|num_str| Ok(Item::new(num_str.parse()?)))
            .collect::<Result<Vec<_>>>()?;

        let operation_regex = Regex::new(r"Operation: new = (\S+) (\S) (\S+)")?;
        let (operation_value_a, operation_value_b, operation) = operation_regex
            .captures(string)
            .and_then(|captures| {
                let operation_value_a = OperationValue::from_str(captures.get(1)?.as_str()).ok()?;
                let operation_value_b = OperationValue::from_str(captures.get(3)?.as_str()).ok()?;
                let op_str = captures.get(2)?.as_str();
                let op = match op_str {
                    "+" => u64::saturating_add,
                    "*" => u64::saturating_mul,
                    _ => return None,
                };
                Some((operation_value_a, operation_value_b, op))
            })
            .ok_or(anyhow!("cannot find operation"))?;

        let divisible_denominator_regex = Regex::new(r"Test: divisible by (\d+)")?;
        let divisible_denominator = divisible_denominator_regex
            .captures(string)
            .and_then(|captures| captures.get(1)?.as_str().parse::<u64>().ok())
            .ok_or(anyhow!("cannot find divisible operator"))?;

        let to_monkey_func = |boolean: bool| -> Result<u64> {
            let regex_str = format!(r"If {}: throw to monkey (\d+)", boolean.to_string());
            let regex = Regex::new(&regex_str)?;
            let capture = regex
                .captures(string)
                .and_then(|captures| captures.get(1))
                .ok_or(anyhow!("cannot find monkey to throw to"))?;
            Ok(capture.as_str().parse::<u64>()?)
        };

        let monkey_true = to_monkey_func(true)?;
        let monkey_false = to_monkey_func(false)?;

        Ok(Self::new(
            items,
            operation,
            operation_value_a,
            operation_value_b,
            divisible_denominator,
            monkey_true,
            monkey_false,
            bored_denominator,
        ))
    }
}

#[derive(Clone)]
struct Monkeys {
    collection: Vec<Monkey>,
}

impl Monkeys {
    fn new(collection: Vec<Monkey>) -> Self {
        Self { collection }
    }
    fn try_from_filename(filename: &str, bored_denominator: u64) -> Result<Self> {
        let content = std::fs::read_to_string(filename)?;
        let monkeys = content
            .split("\n\n")
            .map(|monkey_str| Monkey::from_str(monkey_str, bored_denominator))
            .collect::<Result<Vec<_>>>()?;
        Ok(Monkeys::new(monkeys))
    }
    fn get_at_index(&self, index: usize) -> &Monkey {
        self.collection
            .get(index)
            .expect("monkey at index doesn't exist")
    }
    fn set_at_index(&mut self, index: usize, monkey: Monkey) {
        self.collection[index] = monkey;
    }
    fn get_all_divisible_denominators(&self) -> Vec<u64> {
        self.collection
            .iter()
            .map(|monkey| monkey.divisible_denominator)
            .collect::<Vec<_>>()
    }
    fn create_monkey_with_lowered_item_values(&self, mut monkey: Monkey) -> Monkey {
        let divisible_denominator_product = self
            .get_all_divisible_denominators()
            .iter()
            .product::<u64>();
        monkey
            .items
            .iter_mut()
            .for_each(|item| item.worry_level %= divisible_denominator_product);
        monkey
    }
    fn throw_all_items_for_monkey_at_index(mut self, monkey_index: usize) -> Self {
        let mut monkey =
            self.create_monkey_with_lowered_item_values(self.get_at_index(monkey_index).clone());
        while let Some((monkey_receiver_index, item, monkey_clone)) = monkey.throw_next_item() {
            let receiver_monkey = self
                .collection
                .get_mut(monkey_receiver_index as usize)
                .expect("monkey must exist");
            receiver_monkey.add_item(item);
            monkey = monkey_clone;
        }
        self.set_at_index(monkey_index, monkey);
        self
    }
    fn perform_round(mut self) -> Self {
        for monkey_index in 0..self.collection.len() {
            self = self.throw_all_items_for_monkey_at_index(monkey_index);
        }
        self
    }
    fn perform_rounds(mut self, count: u64) -> Self {
        for _ in 0..count {
            self = self.perform_round();
        }
        self
    }
    fn task(mut self, rounds: u64) -> u64 {
        self = self.perform_rounds(rounds);
        let mut inspections = self
            .collection
            .iter()
            .map(|monkey| monkey.inspections)
            .collect::<Vec<_>>();
        inspections.sort();
        inspections[inspections.len() - 2..].iter().product()
    }
    fn task1(self) -> u64 {
        self.task(20)
    }
    fn task2(self) -> u64 {
        self.task(10_000)
    }
}

fn main() -> Result<()> {
    dbg!(Monkeys::try_from_filename("input.txt", 3)?.task1());
    dbg!(Monkeys::try_from_filename("input.txt", 1)?.task2());
    Ok(())
}
