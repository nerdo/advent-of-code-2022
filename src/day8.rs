//! Advent of Code 2022 Day 8.
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

use anyhow::{anyhow, bail, Error};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_num_visible_trees_from_outside_should_return_the_correct_answer() -> Result<(), Error> {
        let input = "
30373
25512
65332
33549
35390

            ";

        let forest = Forest::from(input)?;
        let answer = forest.get_num_visible_trees_from_outside();

        assert_eq!(answer, 21);

        Ok(())
    }
}

/// Part 1 of Day 8.
pub mod part1 {
    /// Solution for part 1 day 8.
    pub fn solution() {}
}

/// Represents a tree in the Forest.
pub struct Tree {
    /// The height of the tree.
    height: u32,

    /// Whether or not the tree is visible from outside the Forest.
    is_visible_from_outside: bool,
}

/// Represents a forest as a grid of trees.
pub struct Forest {
    /// The vector of trees.
    trees: Vec<Tree>,

    /// The width of the Forest.
    width: usize,

    /// The height of the Forest.
    height: usize,
}

impl Forest {
    /// Gets a tree at a given coordinate in the Forest.
    /// Both row_index and col_index start at 0.
    // pub fn get_tree(&self, row_index: usize, col_index: usize) -> Option<&Tree> {
    //     let Some(row) = self.trees.get(row_index) else {
    //         return None;
    //     };
    //
    //     row.get(col_index)
    // }

    /// Loads the representation of the Forest from a string.
    pub fn from(input: &str) -> Result<Self, Error> {
        let mut heights = vec![];
        let mut forest_width = 0usize;

        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let mut row = vec![];

            for c in line.chars() {
                let height = match c {
                    '0' => 0u32,
                    '1' => 1u32,
                    '2' => 2u32,
                    '3' => 3u32,
                    '4' => 4u32,
                    '5' => 5u32,
                    '6' => 6u32,
                    '7' => 7u32,
                    '8' => 8u32,
                    '9' => 9u32,
                    _ => bail!("Forest parse error: invalid character in input: {c}"),
                };

                row.push(height);
            }

            if row.len() > forest_width {
                forest_width = row.len();
            }

            heights.push(row);
        }

        let forest_height = heights.len();

        // Generate vectors for the columns to make them easy to work with.
        let mut cols = Vec::with_capacity(forest_width);
        for col_index in 0..forest_width {
            let mut col = Vec::with_capacity(forest_height);

            for row_index in 0..forest_height {
                let h = heights
                    .get(row_index)
                    .ok_or_else(|| anyhow!("Invalid row: ({})", row_index,))?
                    .get(col_index)
                    .ok_or_else(|| {
                        anyhow!(
                            "Invalid row, col coordinate: ({}, {})",
                            row_index,
                            col_index
                        )
                    })?;
                col.push(h);
            }

            cols.push(col);
        }

        let trees = heights
            .iter()
            .zip(0..)
            .map(|(row, row_index)| {
                row.iter()
                    .zip(0..)
                    .map(|(height, col_index)| {
                        let is_visible_from_outside = 'block: {
                            // The outside of the forest is always visible.
                            if row_index == 0
                                || col_index == 0
                                || row_index == forest_height - 1
                                || col_index == forest_width - 1
                            {
                                break 'block true;
                            }

                            let row = &heights[row_index];
                            let col = &cols[col_index];
                            let is_visible_fn = |is_visible, h| is_visible && h < height;

                            let left = &row[..col_index];
                            if left.iter().fold(true, is_visible_fn) {
                                break 'block true;
                            }

                            let right = &row[col_index + 1..];
                            if right.iter().fold(true, is_visible_fn) {
                                break 'block true;
                            }

                            let above = &col[..row_index];
                            if above.iter().copied().fold(true, is_visible_fn) {
                                break 'block true;
                            }

                            let below = &col[row_index + 1..];
                            if below.iter().copied().fold(true, is_visible_fn) {
                                break 'block true;
                            }

                            false
                        };

                        Tree {
                            height: *height,
                            is_visible_from_outside,
                        }
                    })
                    .collect::<Vec<Tree>>()
            })
            .fold(Vec::<Tree>::new(), |mut trees, mut row| {
                trees.append(&mut row);
                trees
            });

        Ok(Forest {
            trees,
            width: forest_width,
            height: forest_height,
        })
    }

    /// Gets the number of trees visible from the outside of the forest.
    pub fn get_num_visible_trees_from_outside(&self) -> u32 {
        self.trees
            .iter()
            .map(|t| match t.is_visible_from_outside {
                true => 1,
                false => 0,
            })
            .into_iter()
            .sum()
    }
}
