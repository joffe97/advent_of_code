use anyhow::{anyhow, Error, Result};
use regex::Regex;
use std::{fs, path::PathBuf, str::FromStr};

#[derive(Clone)]
struct Crate {
    crate_type: char,
}

impl Crate {
    fn from_char(chr: char) -> Result<Self> {
        match chr {
            'A'..='Z' => Ok(Self { crate_type: chr }),
            _ => Err(anyhow!("invalid char")),
        }
    }
}

struct CrateStack {
    crates: Vec<Crate>,
}

impl CrateStack {
    fn new(crates: Vec<Crate>) -> Self {
        Self { crates }
    }
    fn top(&self) -> Option<&Crate> {
        self.crates.last()
    }
}

struct CrateStacks {
    stacks: Vec<CrateStack>,
}

impl CrateStacks {
    fn try_from_lines(lines: &[&str]) -> Result<Self> {
        let stack_count =
            ((lines.first().ok_or(anyhow!("no lines"))?.len() as f32) / 4.0).ceil() as usize;
        let mut stacks = vec![vec![]; stack_count];
        for line in lines.iter().map(|line| line.chars()) {
            for (i, char) in line.skip(1).step_by(4).enumerate() {
                if char.is_ascii_uppercase() {
                    stacks[i].push(Crate::from_char(char)?);
                }
            }
        }
        let crate_stacks = stacks
            .into_iter()
            .map(|stack| CrateStack::new(stack.into_iter().rev().collect()))
            .collect();

        Ok(Self {
            stacks: crate_stacks,
        })
    }
    fn move_crate(&mut self, from: usize, to: usize) -> Option<()> {
        let cur_crate = self.stacks[from].crates.pop()?;
        self.stacks[to].crates.push(cur_crate);
        Some(())
    }
    fn apply_procedure_9000(&mut self, procedure: &Procedure) -> Option<()> {
        if procedure.count <= 0 {
            return Some(());
        }
        let from_index = (procedure.from - 1) as usize;
        let to_index = (procedure.to - 1) as usize;
        self.move_crate(from_index, to_index)?;

        self.apply_procedure_9000(&procedure.create_with_decremented_count())
    }
    fn apply_procedure_9001(&mut self, procedure: &Procedure) -> Option<()> {
        let from_index = (procedure.from - 1) as usize;
        let to_index = (procedure.to - 1) as usize;

        let crates = {
            let from_crates = &mut self.stacks[from_index].crates;
            from_crates
                .drain(from_crates.len() - procedure.count as usize..)
                .collect::<Vec<_>>()
        };
        self.stacks[to_index].crates.extend(crates);
        Some(())
    }
    fn apply_procedures(
        &mut self,
        procedures: &Procedures,
        apply_procedure_func: fn(&mut Self, &Procedure) -> Option<()>,
    ) -> Option<()> {
        for procedure in &procedures.procedures {
            apply_procedure_func(self, procedure)?;
        }
        Some(())
    }
    fn top(&self) -> Vec<&Crate> {
        self.stacks.iter().filter_map(|stack| stack.top()).collect()
    }
    fn top_str(&self) -> String {
        self.top().iter().fold(String::new(), |acc, cur_crate| {
            acc + cur_crate.crate_type.to_string().as_str()
        })
    }
}

#[derive(Clone)]
struct Procedure {
    count: u32,
    from: u32,
    to: u32,
}

impl FromStr for Procedure {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self> {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("invalid regex");
        let captures = re
            .captures(string)
            .ok_or(anyhow!("no captures found"))?
            .iter()
            .skip(1)
            .take(3)
            .filter_map(|capture_str_option| {
                capture_str_option.and_then(|capture_str| capture_str.as_str().parse().ok())
            })
            .collect::<Vec<u32>>();
        match captures.as_slice() {
            [count, from, to] => Ok(Self {
                count: *count,
                from: *from,
                to: *to,
            }),
            _ => Err(anyhow!("not enough captures found")),
        }
    }
}

impl Procedure {
    fn create_with_decremented_count(&self) -> Self {
        Self {
            count: self.count - 1,
            from: self.from,
            to: self.to,
        }
    }
}

struct Procedures {
    procedures: Vec<Procedure>,
}

impl Procedures {
    fn try_from_lines(lines: &[&str]) -> Result<Self> {
        let procedures = lines
            .into_iter()
            .map(|line| Procedure::from_str(line))
            .collect::<Result<_>>()?;
        Ok(Self { procedures })
    }
}

fn read_file(filename: &str) -> Result<(CrateStacks, Procedures)> {
    let path = PathBuf::from_str(filename)?;
    let content = fs::read_to_string(path)?;

    let (stacks_str, procedures_str) = content.lines().fold(
        (vec![], vec![]),
        |(mut acc_stacks, mut acc_procedures), cur_line| {
            if cur_line.is_empty() {
            } else if cur_line.starts_with("move") {
                acc_procedures.push(cur_line);
            } else {
                acc_stacks.push(cur_line);
            }
            (acc_stacks, acc_procedures)
        },
    );

    let crate_stacks = CrateStacks::try_from_lines(stacks_str.as_slice())?;
    let procedures = Procedures::try_from_lines(procedures_str.as_slice())?;

    Ok((crate_stacks, procedures))
}

fn main() -> Result<()> {
    let (mut crate_stacks, procedures) = read_file("input.txt")?;
    crate_stacks.apply_procedures(&procedures, CrateStacks::apply_procedure_9001);
    dbg!(crate_stacks.top_str());
    Ok(())
}
