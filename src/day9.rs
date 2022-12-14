//! Advent of Code Day 9.
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Add, Sub},
};

use anyhow::{anyhow, bail, Error};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_number_of_positions_rope_tail_visits_at_least_once_returns_the_correct_answer(
    ) -> Result<(), Error> {
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

        let rope = Rope::parse(input, 2)?;

        let answer = rope.get_number_of_positions_rope_tail_visits_at_least_once();

        assert_eq!(answer, 13);

        Ok(())
    }

    #[test]
    fn get_number_of_positions_rope_tail_visits_at_least_once_with_ten_knots_sanity_check(
    ) -> Result<(), Error> {
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

        let rope = Rope::parse(input, 10)?;

        let answer = rope.get_number_of_positions_rope_tail_visits_at_least_once();

        assert_eq!(answer, 1);

        Ok(())
    }

    #[test]
    fn get_number_of_positions_rope_tail_visits_at_least_once_with_ten_knots_complex(
    ) -> Result<(), Error> {
        let input = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20

            ";

        let rope = Rope::parse(input, 10)?;

        let answer = rope.get_number_of_positions_rope_tail_visits_at_least_once();

        assert_eq!(answer, 36);

        Ok(())
    }
}

/// Part 1.
pub mod part1 {
    use std::{env::current_dir, fs::read_to_string};

    use super::*;

    /// Solution for Part 1.
    pub fn solution() -> Result<(), Error> {
        let filename = current_dir()?.join("src/data/day9.txt");
        let input = read_to_string(filename)?;

        let rope = Rope::parse(&input, 2)?;

        let answer = rope.get_number_of_positions_rope_tail_visits_at_least_once();

        println!("{}", rope);
        println!("Solution for day 9 part 1: {}", answer);

        Ok(())
    }
}

/// Part 2.
pub mod part2 {
    use std::{env::current_dir, fs::read_to_string};

    use super::*;

    /// Solution for Part 2.
    pub fn solution() -> Result<(), Error> {
        let filename = current_dir()?.join("src/data/day9.txt");
        let input = read_to_string(filename)?;

        let rope = Rope::parse(&input, 10)?;

        let answer = rope.get_number_of_positions_rope_tail_visits_at_least_once();

        println!("{}", rope);
        println!("Solution for day 9 part 2: {}", answer);

        Ok(())
    }
}

/// Representation of a position.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    /// X coordinate.
    x: i32,

    /// Y coordinate.
    y: i32,
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Position {
    /// Sets the x and y properties to their absolute values.
    pub fn abs(&mut self) {
        self.x = self.x.abs();
        self.y = self.y.abs();
    }
}

/// Representation of Rope.
#[derive(Debug)]
pub struct Rope {
    /// List of positions the tail has visited.
    tail_positions_visited: Vec<Position>,
}

impl Rope {
    /// Processes one movement of the rope and returns where all the trailing knot should be.
    fn get_updated_positions(direction: &str, knots: &[Position]) -> Result<Vec<Position>, Error> {
        let mut updated_knots = Vec::with_capacity(knots.len());

        let Some(&start_position) = knots.first() else {
            bail!("No head knot!");
        };

        let mut head_position = start_position;

        // Move the head.
        head_position = match direction {
            "R" => Position {
                x: head_position.x + 1,
                y: head_position.y,
            },
            "L" => Position {
                x: head_position.x - 1,
                y: head_position.y,
            },
            "U" => Position {
                x: head_position.x,
                y: head_position.y + 1,
            },
            "D" => Position {
                x: head_position.x,
                y: head_position.y - 1,
            },
            _ => bail!("Invalid direction! This should never happen..."),
        };

        updated_knots.insert(0, head_position);

        // Note: this can be optimized; as soon as anything is found to be touching, the rest of
        // the knots don't need to be checked (they only move relative to their local head).
        for &tail_position in &knots[1..] {
            let is_touching = |pos| {
                let mut abs_offset = head_position - pos;
                abs_offset.abs();
                matches!(
                    abs_offset,
                    Position { x: 0, y: 0 }
                        | Position { x: 1, y: 0 }
                        | Position { x: 0, y: 1 }
                        | Position { x: 1, y: 1 }
                )
            };

            let mut new_position = tail_position;

            // Continually adjust the tail until it touches the head.
            while !is_touching(new_position) {
                let offset = head_position - new_position;

                if offset.x == 0 {
                    // Head and tail are in the same row.
                    new_position = Position {
                        x: head_position.x,
                        y: if offset.y > 0 {
                            head_position.y - 1
                        } else {
                            head_position.y + 1
                        },
                    };
                } else if offset.y == 0 {
                    // Head and tail are in the same column.
                    new_position = Position {
                        x: if offset.x > 0 {
                            head_position.x - 1
                        } else {
                            head_position.x + 1
                        },
                        y: head_position.y,
                    };
                } else {
                    // Head and tail arne't in the same rank/file, move diagonally until they
                    // touch.
                    let step_x = match offset.x {
                        x if x > 0 => 1,
                        _ => -1,
                    };
                    let step_y = match offset.y {
                        y if y > 0 => 1,
                        _ => -1,
                    };
                    new_position = new_position
                        + Position {
                            x: step_x,
                            y: step_y,
                        };
                }
            }

            head_position = new_position;
            updated_knots.insert(updated_knots.len(), new_position);
        }

        Ok(updated_knots)
    }

    /// Parses string input and simulates the rope's motions.
    fn parse(s: &str, rope_length: usize) -> Result<Self, Error> {
        if rope_length < 2 {
            bail!(
                "Rope has to have a length of at least 2! {} requested.",
                rope_length
            );
        }

        let knots = vec![Position { x: 0, y: 0 }; rope_length];

        let Some(&tail_position) = knots.last() else {
            bail!("No tail knot found!");
        };

        let mut tail_positions_visited = vec![tail_position];
        let mut updated_knots = knots;

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts = line.split(' ').collect::<Vec<&str>>();
            if parts.len() < 2 {
                bail!("Rope motion is missing parameters: {line}");
            }

            match parts.first() {
                Some(direction @ &"R")
                | Some(direction @ &"L")
                | Some(direction @ &"U")
                | Some(direction @ &"D") => {
                    let num_moves = parts
                        .get(1)
                        .ok_or_else(|| anyhow!("Error getting number of moves in line: {line}"))?
                        .parse::<i32>()?;

                    // Process the offset one by one, because the path the tail takes depends on
                    // each individual movement of the head, NOT its final position.
                    for i in 0..num_moves {
                        let Some(&prev_tail) = updated_knots.last() else {
                            bail!("Unable to grab current tail at beginning of move #{}", i);
                        };

                        updated_knots = Self::get_updated_positions(direction, &updated_knots)?;

                        let Some(&new_tail) = updated_knots.last() else {
                            bail!("Unable to grab current tail at end of move #{}", i);
                        };

                        if new_tail != prev_tail {
                            tail_positions_visited.push(new_tail);
                        }
                    }
                }
                _ => bail!("Invalid rope motion direction: {line}"),
            }
        }

        Ok(Rope {
            tail_positions_visited,
        })
    }

    /// Gets the number of positions the tail of the rope visited at least once.
    pub fn get_number_of_positions_rope_tail_visits_at_least_once(&self) -> usize {
        self.tail_positions_visited
            .iter()
            .collect::<HashSet<&Position>>()
            .len()
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unique_positions = self
            .tail_positions_visited
            .iter()
            .copied()
            .collect::<HashSet<Position>>();

        let (lower_bound, upper_bound, origin) = unique_positions.iter().fold(
            Ok((
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
            )),
            |r, p| {
                let Ok((l, u, o)) = r else {
                    return r;
                };

                let mut origin = o;

                if p.x < 0 {
                    origin.x = o.x.max(p.x.abs());
                }

                if p.y < 0 {
                    origin.y = o.y.max(p.y.abs());
                }

                let lower = Position {
                    x: l.x.min(p.x),
                    y: l.y.min(p.y),
                };
                let upper = Position {
                    x: u.x.max(p.x),
                    y: u.y.max(p.y),
                };

                Ok((lower, upper, origin))
            },
        )?;

        let width = usize::try_from(lower_bound.x.abs() + upper_bound.x.abs() + 1)
            .map_err(|_| std::fmt::Error)?;
        let height = usize::try_from(lower_bound.y.abs() + upper_bound.y.abs() + 1)
            .map_err(|_| std::fmt::Error)?;

        let mut buffer = vec![".".to_string(); width * height];
        let mut plot = |c: char, p: Position| -> std::fmt::Result {
            let s = c.to_string();
            let row_offset = usize::try_from(p.y).map_err(|_| std::fmt::Error)?;
            let col_offset = usize::try_from(p.x).map_err(|_| std::fmt::Error)?;
            let index = width * row_offset + col_offset;

            buffer[index] = s;

            Ok(())
        };

        for p in unique_positions.iter() {
            plot('#', origin + *p)?;
        }

        plot('s', origin)?;

        for i in (0..height).rev() {
            let start = i * width;
            let end = start + width;
            writeln!(f, "{}", &buffer[start..end].join(""))?;
        }

        write!(f, "({width} x {height}, origin = {origin:?})")
    }
}
