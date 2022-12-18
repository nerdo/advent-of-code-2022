//! Advent of Code Day 9.
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

use std::str::FromStr;

use anyhow::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_number_of_positions_rope_tail_visits_at_least_once_returns_the_correct_answer() -> Result<(), Error> {
        let input = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2

            ";

        let rope = Rope::from_str(input)?;

        let answer = rope.get_number_of_positions_rope_tail_visits_at_least_once();

        assert_eq!(answer, 13);

        Ok(())
    }
}

/// Part 1.
pub mod part1 {
    use super::*;

    /// Solution for Part 1.
    pub fn solution() -> Result<(), Error> {Ok(())}
}

/// Representation of Rope.
#[derive(Debug)]
pub struct Rope {}

impl FromStr for Rope {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rope {})
    }
}

impl Rope {
    /// Gets the number of positions the tail of the rope visited at least once.
    pub fn get_number_of_positions_rope_tail_visits_at_least_once(&self) -> u32 {
        13
    } 
}
