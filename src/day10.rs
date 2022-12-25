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

        let crt_row_cycle_ranges = vec![
            (1, 40),
            (41, 80),
            (81, 120),
            (121, 160),
            (161, 200),
            (201, 240),
        ];
        let answer = cpu.get_crt_output(&crt_row_cycle_ranges)?;
        println!("{}", answer);

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
    use std::{env::current_dir, fs::read_to_string};

    use super::*;

    /// Solution for day 10, part 2.
    pub fn solution() -> Result<(), Error> {
        let filename = current_dir()
            .context("std::env::current_dir() failed")?
            .join("src/data/day10.txt");
        let input = read_to_string(filename).context("std::fs::read_to_string() failed")?;
        let cpu = CPU::parse(&input)?;
        let crt_row_cycle_ranges = vec![
            (1, 40),
            (41, 80),
            (81, 120),
            (121, 160),
            (161, 200),
            (201, 240),
        ];

        let answer = cpu.get_crt_output(&crt_row_cycle_ranges)?;

        println!("Day 10 Part 2");
        println!("{answer}");

        Ok(())
    }
}

/// Represents the CPU that theis in the elves' communication device.
#[derive(Debug)]
pub struct CPU {
    /// A list of the CPU's instructiohs.
    instructions: Vec<Instruction>,

    /// History of the execution of CPU instructions.
    execution_history: Vec<Registers>,
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

        let execution_history = Self::execute(&instructions, Registers { x: 1 })?;

        Ok(Self {
            instructions,
            execution_history,
        })
    }

    /// Calculates and returns the sum of the signal strengths at the specified cycles.
    pub fn get_sum_of_signal_strengths_at_cycles(&self, cycles: &[u32]) -> Result<i32, Error> {
        let mut signal_strengths = vec![];

        for cycle_number in cycles.iter() {
            // The -1 here is to be able to see the register value AFTER the particular cycle has
            // completed.
            let cycle_index = usize::try_from(*cycle_number)
                .context("Unable to cast cycle_number from u32 to usize!")?
                - 1;
            let Registers { x } = self.execution_history.get(cycle_index).ok_or_else(|| {
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
    pub fn execute(
        instructions: &Vec<Instruction>,
        initial_registers: Registers,
    ) -> Result<Vec<Registers>, Error> {
        let mut registers = initial_registers.clone();
        let mut register_history = vec![initial_registers];

        for instruction in instructions.iter() {
            match instruction {
                Instruction::Noop => {
                    registers = registers.clone();
                    register_history.push(registers);
                }
                Instruction::Addx(x) => {
                    // This takes two cycles, for the first cycle, nothing changes.
                    register_history.push(registers.clone());

                    // The value gets updated during the next cycle.
                    registers = Registers { x: registers.x + x };
                    register_history.push(registers);
                }
            }
        }

        Ok(register_history)
    }

    /// Gets the crt output as a string.
    /// Takes a vector of tuples. Each tuple represents a row on the CRT and the tuple's first
    /// value is the CPU cycle that corresponds to the first pixel in that row. The tuple's last
    /// value is the CPU cycle that corresponds to the last pixel in that row.
    pub fn get_crt_output(&self, crt_row_cycle_ranges: &[(usize, usize)]) -> Result<String, Error> {
        Ok(crt_row_cycle_ranges
            .iter()
            .map(|cycle_range| {
                let mut row = String::new();

                for (index, cycle_number) in (cycle_range.0..=cycle_range.1).enumerate() {
                    // Grab the sprite position.
                    // The -1 here is to be able to see the register value AFTER the particular cycle has
                    // completed.
                    let Registers { x: sprite_position } = self
                        .execution_history
                        .get(cycle_number - 1)
                        .context("Unable to get execution history at CPU cycle {cycle_number}")?;

                    // Convert index to i32 for comparison with sprite_index.
                    let pixel_position = i32::try_from(index).with_context(|| {
                        format!("Unable to convert index from {index}usize to i32!")
                    })?;

                    // Set the pixel according to whether it falls within the 3-wide sprite.
                    let pixel = if pixel_position >= sprite_position - 1
                        && pixel_position <= sprite_position + 1
                    {
                        '#'
                    } else {
                        '.'
                    };

                    row.push(pixel);
                }

                Ok(row)
            })
            .collect::<Result<Vec<String>, Error>>()?
            .join("\n"))
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
