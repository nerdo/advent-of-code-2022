//! Advent of Code 2022, Day 10
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

use anyhow::{anyhow, bail, Context, Error};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_sum_of_signal_strengths_at_cycles_returns_the_correct_answer() -> Result<(), Error> {
        let input = "
            addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop

";

        let cpu = CPU::parse(input)?;
        let cycles = vec![20, 60, 100, 140, 180, 220];

        let answer = cpu.get_sum_of_signal_strengths_at_cycles(&cycles)?;

        assert_eq!(answer, 13140);

        Ok(())
    }
    #[test]

    fn get_crt_output_returns_the_correct_answer() -> Result<(), Error> {
        let input = "
            addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop

";

        let cpu = CPU::parse(input)?;

        let answer = cpu.get_crt_output()?;

        assert_eq!(
            answer,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );

        Ok(())
    }
}

/// Day 10, Part 1
pub mod part1 {
    use std::{env::current_dir, fs::read_to_string};

    use super::*;

    /// Solution for day 10, part 1.
    pub fn solution() -> Result<(), Error> {
        let filename = current_dir()
            .context("std::env::current_dir() failed")?
            .join("src/data/day10.txt");
        let input = read_to_string(filename).context("std::fs::read_to_string() failed")?;
        let cycles = vec![20, 60, 100, 140, 180, 220];
        let cpu = CPU::parse(&input)?;

        let answer = cpu.get_sum_of_signal_strengths_at_cycles(&cycles)?;

        println!("Day 10 Part 1 = {answer}");

        Ok(())
    }
}

/// Day 10, Part 2
pub mod part2 {
    use super::*;

    /// Solution for day 10, part 2.
    pub fn solution() -> Result<(), Error> {
        Ok(())
    }
}

/// Represents the CPU that theis in the elves' communication device.
#[derive(Debug)]
pub struct CPU {
    /// A list of the CPU's instructiohs.
    instructions: Vec<Instruction>,
}

impl CPU {
    /// Parses the instructions from a string slice.
    pub fn parse(input: &str) -> Result<Self, Error> {
        let mut instructions = vec![];

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let instruction = match line {
                "noop" => Instruction::Noop,
                line if line.starts_with("addx ") => {
                    let parts = line.split(" ").collect::<Vec<&str>>();
                    let number = parts
                        .get(1)
                        .context("addx instruction is missing its argument")?
                        .parse::<i32>()?;
                    Instruction::Addx(number)
                }
                _ => bail!("Invalid instruction: {line}"),
            };

            instructions.push(instruction);
        }

        Ok(Self { instructions })
    }

    /// Calculates and returns the sum of the signal strengths at the specified cycles.
    pub fn get_sum_of_signal_strengths_at_cycles(&self, cycles: &[u32]) -> Result<i32, Error> {
        let register_history = self.execute(Registers { x: 1 })?;
        let mut signal_strengths = vec![];

        for cycle_number in cycles.iter() {
            // The -1 here is to be able to see the register value AFTER the particular cycle has
            // completed.
            let cycle_index = usize::try_from(*cycle_number)
                .context("Unable to cast cycle_number from u32 to usize!")?
                - 1;
            let (Registers { x }, _) = register_history.get(cycle_index).ok_or_else(|| {
                anyhow!(
                    "out of bounds while getting register history at cycle number {cycle_number}"
                )
            })?;

            let strength = i32::try_from(*cycle_number)
                .context("Unable to cast cycle_number from u32 to i32!")?
                * x;
            signal_strengths.push(strength);
        }

        Ok(signal_strengths.iter().sum::<i32>())
    }

    /// Executes instructions and returns the register history for each cycle as a list of
    /// registers.
    fn execute(&self, initial_registers: Registers) -> Result<Vec<(Registers, String)>, Error> {
        let mut registers = initial_registers.clone();
        let mut register_history = vec![(initial_registers, "init".to_string())];

        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Noop => {
                    registers = registers.clone();
                    register_history.push((registers, "noop".to_string()));
                }
                Instruction::Addx(x) => {
                    // This takes two cycles, for the first cycle, nothing changes.
                    register_history
                        .push((registers.clone(), format!("off cycle: {instruction:?}")));

                    // The value gets updated during the next cycle.
                    registers = Registers { x: registers.x + x };
                    register_history.push((registers, format!("ON cycle: {instruction:?}")));
                }
            }
        }

        Ok(register_history)
    }

    /// Gets the crt output as a string.
    pub fn get_crt_output(&self) -> Result<String, Error> {
        Ok("##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            .to_string())
    }
}

/// A CPU instruction.
#[derive(Debug)]
pub enum Instruction {
    /// No operation.
    Noop,

    /// Add x instruction.
    Addx(i32),
}

/// CPU registers.
#[derive(Debug, Copy, Clone)]
pub struct Registers {
    x: i32,
}
