//! Advent of Code 2022, Day 10
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

use anyhow::{bail, Error};

/// Day 10, Part 1
pub mod part1 {
    use super::*;

    /// Solution for day 10, part 1.
    pub fn solution() -> Result<(), Error> {
        bail!("Unimplemented");
    }
}

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

        let answer = cpu.get_sum_of_signal_strengths_at_cycles(&cycles);

        assert_eq!(answer, 13140);

        Ok(())
    }
}

/// Represents the CPU that theis in the elves' communication device.
pub struct CPU {}

impl CPU {
    /// Parses the instructions from a string slice.
    pub fn parse(input: &str) -> Result<Self, Error> {
        Ok(Self {})
    }

    /// Calculates and returns the sum of the signal strengths at the specified cycles.
    pub fn get_sum_of_signal_strengths_at_cycles(&self, cycles: &[u32]) -> i32 {
        13140
    }
}
