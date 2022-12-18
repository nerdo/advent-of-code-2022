//! Advent of Code Day 9.
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Add, Sub},
    str::FromStr,
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
    pub fn solution() -> Result<(), Error> {
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

impl FromStr for Rope {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut head_position = Position { x: 0, y: 0 };
        let mut tail_position = Position { x: 0, y: 0 };
        let mut tail_positions_visited = vec![tail_position];

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
                    for _i in 0..num_moves {
                        // Move the head.
                        match *direction {
                            "R" => {
                                head_position = Position {
                                    x: head_position.x + 1,
                                    y: head_position.y,
                                };
                            }
                            "L" => {
                                head_position = Position {
                                    x: head_position.x - 1,
                                    y: head_position.y,
                                };
                            }
                            "U" => {
                                head_position = Position {
                                    x: head_position.x,
                                    y: head_position.y + 1,
                                };
                            }
                            "D" => {
                                head_position = Position {
                                    x: head_position.x,
                                    y: head_position.y - 1,
                                };
                            }
                            _ => bail!("Invalid direction! This should never happen..."),
                        };

                        // Conditionally adjust the tail.
                        let offset = head_position - tail_position;
                        let mut abs_offset = offset;
                        abs_offset.abs();
                        let mut tail_adjusted = false;
                        tail_position = match abs_offset {
                            Position { x: 0, y: 0 }
                            | Position { x: 1, y: 0 }
                            | Position { x: 0, y: 1 }
                            | Position { x: 1, y: 1 } => tail_position,
                            Position { x, y: _ } if x > 1 => {
                                // Horizontal motion.
                                tail_adjusted = true;
                                Position {
                                    x: head_position.x
                                        + match head_position.x {
                                            x if x > tail_position.x => -1,
                                            _ => 1,
                                        },
                                    y: head_position.y,
                                }
                            }
                            Position { x: _, y } if y > 1 => {
                                // Vertical motion.
                                tail_adjusted = true;
                                Position {
                                    x: head_position.x,
                                    y: head_position.y
                                        + match head_position.y {
                                            y if y > tail_position.y => -1,
                                            _ => 1,
                                        },
                                }
                            }
                            _ => {
                                bail!("Unexpected motion: abs_offset = {abs_offset:?}, offset = {offset:?}");
                            }
                        };

                        // Record the position if it was adjusted.
                        if tail_adjusted {
                            tail_positions_visited.push(tail_position);
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
}

impl Rope {
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
        let (width, height, origin) = self.tail_positions_visited.iter().fold(
            Ok((1usize, 1usize, Position { x: 0, y: 0 })),
            |r, p| {
                let Ok((w, h, o)) = r else {
                    return r;
                };

                let mut origin = o;

                if p.x < 0 {
                    origin.x = o.x.max(p.x.abs());
                }

                if p.y < 0 {
                    origin.y = o.y.max(p.y.abs());
                }

                let width = w.max(
                    (1 + p.x.abs() + origin.x)
                        .try_into()
                        .map_err(|_| std::fmt::Error)?,
                );
                let height = h.max(
                    (1 + p.y.abs() + origin.y)
                        .try_into()
                        .map_err(|_| std::fmt::Error)?,
                );

                Ok((width, height, origin))
            },
        )?;

        let mut buffer = ".".repeat(width * height);
        let mut plot = |c: char, p: Position| -> std::fmt::Result {
            let s = c.to_string();
            let row_offset: usize = p.y.try_into().map_err(|_| std::fmt::Error)?;
            let col_offset: usize = p.x.try_into().map_err(|_| std::fmt::Error)?;
            let index = width * row_offset + col_offset;

            // https://stackoverflow.com/a/66662405/2057996
            buffer.replace_range(
                buffer
                    .char_indices()
                    .nth(index)
                    .map(|(pos, ch)| (pos..pos + ch.len_utf8()))
                    .expect("Unexpected error replacing character in Rope::Display."),
                &s,
            );

            Ok(())
        };

        for p in self.tail_positions_visited.iter() {
            plot('#', origin + *p)?;
        }

        plot('s', origin)?;

        for i in (0..height).rev() {
            let start = i * width;
            let end = start + width;
            writeln!(f, "{}", &buffer[start..end])?;
        }

        write!(f, "({width} x {height}, origin = {origin:?})")
    }
}
