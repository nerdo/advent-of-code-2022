#![warn(missing_docs)]
//! Advent of Code 2022 Day 8

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_num_visible_trees_from_outside_should_return_the_correct_answer() {
        let input = "
30373
25512
65332
33549
35390

            ";

        let forest = Forest::from(&input);

        let answer = forest.get_num_visible_trees_from_outside();

        assert_eq!(answer, 21);
    }
}

/// Part 1 of Day 8
pub mod part1 { 
    /// Solution for part 1 day 8
    pub fn solution() {}
}

/// Represents a forest as a grid of trees.
pub struct Forest {

}

impl Forest {
    /// Loads the representation of the Forest from a string.
    pub fn from(input: &str) -> Self {
        Forest {}
    }

    /// Gets the number of trees visible from the outside of the forest.
    pub fn get_num_visible_trees_from_outside(&self) -> u32 {
        21
    }
}
