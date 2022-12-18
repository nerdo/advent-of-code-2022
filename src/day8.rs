//! Advent of Code 2022 Day 8.
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

use std::str::FromStr;

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

        let forest = Forest::from_str(input)?;
        let answer = forest.get_num_visible_trees_from_outside();

        assert_eq!(answer, 21);

        Ok(())
    }

    #[test]
    fn get_highest_scenic_score_should_return_the_correct_answer() -> Result<(), Error> {
        let input = "
30373
25512
65332
33549
35390

            ";

        let forest = Forest::from_str(input)?;
        let answer = forest.get_highest_scenic_score()?;

        assert_eq!(answer, 8);

        Ok(())
    }
}

/// Part 1 of Day 8.
pub mod part1 {
    use super::*;
    use std::{env::current_dir, fs::read_to_string};

    /// Solution for part 1 day 8.
    pub fn solution() -> Result<(), Error> {
        let filename = current_dir()?.join("src/data/day8.txt");
        let input = read_to_string(filename)?;

        let forest = Forest::from_str(&input)?;
        let answer = forest.get_num_visible_trees_from_outside();

        println!("Day 8 Part 1 = {answer}");

        Ok(())
    }
}

/// Part 2 of Day 8.
pub mod part2 {
    use super::*;
    use std::{env::current_dir, fs::read_to_string};

    /// Solution for part 2 day 8.
    pub fn solution() -> Result<(), Error> {
        let filename = current_dir()?.join("src/data/day8.txt");
        let input = read_to_string(filename)?;

        let forest = Forest::from_str(&input)?;
        let answer = forest.get_highest_scenic_score()?;

        println!("Day 8 Part 2 = {answer}");

        Ok(())
    }
}

/// Represents a tree in the Forest.
pub struct Tree {
    /// The height of the tree.
    _height: u32,

    /// Whether or not the tree is visible from outside the Forest.
    is_visible_from_outside: bool,

    /// Scenic score for the tree.
    /// The product of the viewing distance on each side of the tree.
    scenic_score: u32,
}

/// Represents a forest as a grid of trees.
pub struct Forest {
    /// The vector of trees.
    trees: Vec<Tree>,

    /// The width of the Forest.
    _width: usize,

    /// The height of the Forest.
    _height: usize,
}

impl FromStr for Forest {
    type Err = Error;

    /// Loads the representation of the Forest from a string.
    fn from_str(input: &str) -> Result<Self, Error> {
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
                        let (is_visible_from_outside, scenic_score) = {
                            let row = &heights[row_index];
                            let col = &cols[col_index];
                            let is_visible_fn = |is_visible, h| is_visible && h < height;
                            let get_viewing_distance = |(distance, blocked), h| {
                                if blocked {
                                    return (distance, true);
                                }
                                (distance + 1, h >= height)
                            };

                            let left = &row[..col_index];
                            let visible_from_left = left.iter().fold(true, is_visible_fn);
                            let left_scenic_score =
                                left.iter().rev().fold((0, false), get_viewing_distance).0;

                            let right = &row[col_index + 1..];
                            let visible_from_right = right.iter().fold(true, is_visible_fn);
                            let right_scenic_score =
                                right.iter().fold((0, false), get_viewing_distance).0;

                            let above = &col[..row_index];
                            let visible_from_above =
                                above.iter().copied().fold(true, is_visible_fn);
                            let above_scenic_score = above
                                .iter()
                                .copied()
                                .rev()
                                .fold((0, false), get_viewing_distance)
                                .0;

                            let below = &col[row_index + 1..];
                            let visible_from_below =
                                below.iter().copied().fold(true, is_visible_fn);
                            let below_scenic_score = below
                                .iter()
                                .copied()
                                .fold((0, false), get_viewing_distance)
                                .0;

                            (
                                visible_from_left
                                    || visible_from_right
                                    || visible_from_above
                                    || visible_from_below,
                                left_scenic_score
                                    * right_scenic_score
                                    * above_scenic_score
                                    * below_scenic_score,
                            )
                        };

                        Tree {
                            _height: *height,
                            is_visible_from_outside,
                            scenic_score,
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
            _width: forest_width,
            _height: forest_height,
        })
    }
}

impl Forest {
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

    /// Gets the highest scenic score in the forest.
    pub fn get_highest_scenic_score(&self) -> Result<u32, Error> {
        self.trees
            .iter()
            .map(|t| t.scenic_score)
            .max()
            .ok_or_else(|| anyhow!("Error getting the highest scneic score."))
    }
}
