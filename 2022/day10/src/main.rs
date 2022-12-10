use anyhow::{anyhow, Result};
use std::{
    fmt::{Display, Write},
    ops::AddAssign,
    vec,
};

trait Instruction {
    fn cycles(&self) -> usize;
    fn perform(&self, register: &mut isize);
}

struct Addx {
    value: isize,
}

impl Addx {
    fn new(value: isize) -> Self {
        Self { value }
    }
}

impl Instruction for Addx {
    fn cycles(&self) -> usize {
        2
    }
    fn perform(&self, register: &mut isize) {
        register.add_assign(self.value)
    }
}

struct Noop;

impl Instruction for Noop {
    fn cycles(&self) -> usize {
        1
    }

    fn perform(&self, register: &mut isize) {}
}

struct InstructionStack {
    collection: Vec<Box<dyn Instruction>>,
}

impl InstructionStack {
    fn new(vector: Vec<Box<dyn Instruction>>) -> Self {
        Self { collection: vector }
    }
    fn inverted(mut self) -> Self {
        self.collection.reverse();
        self
    }
    fn create_instruction_from_string(string: &str) -> Result<Box<dyn Instruction>> {
        Ok(
            match string.split_whitespace().collect::<Vec<_>>().as_slice() {
                ["addx", value_str] => Box::new(Addx::new(value_str.parse()?)),
                ["noop"] => Box::new(Noop),
                _ => return Err(anyhow!("instruction does not exist")),
            },
        )
    }
    fn try_from_filename(filename: &str) -> Result<Self> {
        let content = std::fs::read_to_string(filename)?;
        let vector = content
            .lines()
            .map(|line| Self::create_instruction_from_string(line))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self::new(vector).inverted())
    }
}

struct CPU {
    register: isize,
    ticks: usize,
    instruction_stack: InstructionStack,
    current_instruction: Option<Box<dyn Instruction>>,
    ticks_since_current_instruction_start: usize,
    task1_out: isize,
    register_history: Vec<isize>,
}

impl CPU {
    fn new(instruction_set: InstructionStack) -> Self {
        let register = 1;
        let ticks = 0;
        let current_instruction = None;
        let ticks_since_current_instruction_start = 0;
        let task1_out = 0;
        let register_history = vec![register];
        Self {
            register,
            ticks,
            instruction_stack: instruction_set,
            current_instruction,
            ticks_since_current_instruction_start,
            task1_out,
            register_history,
        }
    }
    fn wipe_current_instruction(&mut self) {
        self.current_instruction = None
    }
    fn perform_tick(&mut self) {
        let instruction = match &self.current_instruction {
            Some(instruction) => instruction,
            None => {
                self.current_instruction = self.instruction_stack.collection.pop();
                self.ticks_since_current_instruction_start = 0;
                match &self.current_instruction {
                    Some(instruction) => instruction,
                    None => return,
                }
            }
        };

        self.ticks_since_current_instruction_start.add_assign(1);
        self.ticks.add_assign(1);

        self.register_history.push(self.register.clone());

        match instruction
            .cycles()
            .cmp(&self.ticks_since_current_instruction_start)
        {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                instruction.perform(&mut self.register);
                self.wipe_current_instruction();
            }
            std::cmp::Ordering::Greater => (),
        }
    }
    fn is_finished(&self) -> bool {
        self.instruction_stack.collection.is_empty() && self.current_instruction.is_none()
    }
    fn run_instructions(&mut self) {
        while !self.is_finished() {
            self.perform_tick();
        }
    }
    fn task1(&self) -> isize {
        self.register_history
            .iter()
            .enumerate()
            .fold(0, |acc, (tick, register)| {
                if ((tick + 20) % 40) == 0 {
                    acc + tick as isize * register
                } else {
                    acc
                }
            })
    }
}

struct Screen {
    pixels: Vec<Vec<bool>>,
}

impl Screen {
    fn new(pixels: Vec<Vec<bool>>) -> Self {
        Self { pixels }
    }
    fn from_registers(registers: Vec<isize>, x_width: usize) -> Self {
        let mut pixels = vec![];
        let mut cur_row = vec![false; x_width];

        for (tick, register) in registers.iter().skip(1).enumerate() {
            let horizontal_pos = tick % x_width;
            // let horizontal_pos = tick - x_width * vertical_pos;
            let is_lit = {
                let diff = register - &(horizontal_pos as isize);
                -1 <= diff && diff <= 1
            };
            if is_lit {
                cur_row[horizontal_pos] = true;
            }
            if tick != 0 && horizontal_pos == 0 {
                pixels.push(cur_row);
                cur_row = vec![false; x_width];
            }
        }
        pixels.push(cur_row);

        Self::new(pixels)
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.pixels.iter().enumerate() {
            if i != 0 {
                f.write_char('\n')?;
            }
            let row_str = String::from_iter(row.iter().map(|pixel| if *pixel { "#" } else { "." }));
            f.write_str(&row_str)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let instruction_set = InstructionStack::try_from_filename("input.txt")?;
    let mut cpu = CPU::new(instruction_set);
    cpu.run_instructions();
    dbg!(cpu.task1());
    let screen = Screen::from_registers(cpu.register_history, 40);
    println!("{}", screen);
    Ok(())
}
