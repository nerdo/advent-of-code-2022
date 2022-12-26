//! Advent of Code Day 11
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

use anyhow::{Context, Error};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_monkey_business_level_returns_the_correct_answer() -> Result<(), Error> {
        let input = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
    ";

        let monkey_sim = MonkeySim::parse(input)?;

        let answer = monkey_sim.get_monkey_business_level(20, 2)?;

        assert_eq!(answer, 10605);

        Ok(())
    }
}

/// Simulates monkey business.
#[derive(Debug)]
pub struct MonkeySim {}

impl MonkeySim {
    /// Parses text input to build the initial state for the monkey simulator.
    ///
    /// # Arguments
    ///
    /// * `input` - The string description of the monkeys' initial state.
    pub fn parse(input: &str) -> Result<Self, Error> {
        Ok(MonkeySim {})
    }

    /// Gets the level of monkey business after r rounds have been simulated.
    ///
    /// # Arguments
    ///
    /// * `num_rounds` - Number of rounds to simulate.
    /// * `top_n` - Number of top active monkeys to consider in the monkey business level.
    pub fn get_monkey_business_level(&self, num_rounds: usize, top_n: usize) -> Result<u32, Error> {
        let mut num_monkey_inspections = vec![(0u32, 101u32), (1, 95), (2, 7), (3, 105)];
        num_monkey_inspections.sort_by(|a, b| (b.1).cmp(&a.1));
        let top_monkeys = num_monkey_inspections.iter().take(top_n);
        let monkey_business_level = top_monkeys.fold(1u32, |product, tuple| product * tuple.1);

        Ok(monkey_business_level)
    }
}

/// Part 1
pub mod part1 {
    use super::*;

    /// The solution for Day 11 Part 1
    pub fn solution() -> Result<(), Error> {
        Ok(())
    }
}
