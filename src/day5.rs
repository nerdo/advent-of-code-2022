#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_message_from_rearranged_crates_returns_the_correct_answer() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

";
        let answer = get_message_from_rearranged_crates(
            input,
            &Settings {
                move_multiple_crates_at_once: false,
            },
        );

        assert_eq!(answer, "CMZ");
    }

    #[test]
    fn get_message_from_rearranged_crates_moving_multiple_crates_at_once_returns_the_correct_answer() {
        let input = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

";
        let answer = get_message_from_rearranged_crates(
            input,
            &Settings {
                move_multiple_crates_at_once: true,
            },
        );

        assert_eq!(answer, "MCD");
    }
}

pub mod part1 {
    use super::*;
    use std::{env::current_dir, fs::read_to_string};

    pub fn solution() {
        let filename = current_dir().unwrap().join("src/data/day5.txt");
        let input = read_to_string(filename).unwrap();

        let answer = get_message_from_rearranged_crates(
            &input,
            &Settings {
                move_multiple_crates_at_once: false,
            },
        );

        println!("day 5 part 1 message = {answer:?}");
    }
}

pub mod part2 {
    use super::*;
    use std::{env::current_dir, fs::read_to_string};

    pub fn solution() {
        let filename = current_dir().unwrap().join("src/data/day5.txt");
        let input = read_to_string(filename).unwrap();

        let answer = get_message_from_rearranged_crates(
            &input,
            &Settings {
                move_multiple_crates_at_once: true,
            },
        );

        println!("day 5 part 2 message = {answer:?}");
    }
}

pub struct Settings {
    move_multiple_crates_at_once: bool,
}

#[derive(Debug)]
pub struct Crates {
    pub stacks: Vec<Vec<String>>,
}

#[derive(Debug)]
pub enum Instruction {
    Move {
        quantity: usize,
        from_stack: usize,
        to_stack: usize,
    },
}

impl Crates {
    pub fn get_message(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.last().unwrap())
            .fold(String::new(), |mut message, s| {
                message.push_str(s);
                message
            })
    }

    pub fn rearrange(&mut self, instructions: &[Instruction], settings: &Settings) {
        for instruction in instructions {
            match instruction {
                Instruction::Move {
                    quantity,
                    from_stack,
                    to_stack,
                } => {
                    let source = self.stacks.get_mut(*from_stack).unwrap();

                    let mut payload = source
                        .splice(source.len() - *quantity..source.len(), [])
                        .collect::<Vec<String>>();
                    if !settings.move_multiple_crates_at_once {
                        payload.reverse();
                    }

                    let destination = self.stacks.get_mut(*to_stack).unwrap();
                    destination.append(&mut payload);
                }
            }
        }
    }
}

enum ParserState {
    InitialState { has_parsed_data: bool },
    Instructions,
}

pub fn parse_initial_state_and_instructions(
    initial_state_and_instructions: &str,
) -> (Crates, Vec<Instruction>) {
    let mut stacks = Vec::<Vec<String>>::new();
    let mut instructions = Vec::<Instruction>::new();
    let mut parser_state = ParserState::InitialState {
        has_parsed_data: false,
    };

    for line in initial_state_and_instructions.lines() {
        match parser_state {
            ParserState::InitialState { has_parsed_data } => {
                if has_parsed_data && !line.contains('[') {
                    // This is the line with the stack numbers... switch to parsing instructions...
                    parser_state = ParserState::Instructions;
                    continue;
                }

                // Each crate takes up three characters, followed by an optional space...
                // Conceptually, forcing each crate to have a trailing space and grabbing every
                // 4 characters will gather the crates present on this line.
                let num_crates = (line.trim_end().len() + 1) / 4;

                // Always ensure we have the correct number of stacks.
                while stacks.len() < num_crates {
                    stacks.push(Vec::<String>::new());
                }

                // Grab each crate and put it on the BOTTOM of its stack.
                for i in 0..num_crates {
                    // Every 4 characters is a crate, but we really only need the letter from it,
                    // e.g. `[N] ` yields `N`...
                    let starting_offset = i * 4;
                    let c = &line[starting_offset + 1..starting_offset + 2];
                    if c.trim().is_empty() {
                        continue;
                    }
                    stacks.get_mut(i).unwrap().insert(0, c.to_owned());
                }

                parser_state = ParserState::InitialState {
                    has_parsed_data: true,
                };
            }
            ParserState::Instructions => {
                if line.trim().is_empty() {
                    continue;
                }

                let parts: Vec<&str> = line.split(' ').collect();
                let quantity = parts.get(1).unwrap().parse::<usize>().unwrap();
                let from_stack = parts.get(3).unwrap().parse::<usize>().unwrap() - 1;
                let to_stack = parts.get(5).unwrap().parse::<usize>().unwrap() - 1;
                instructions.push(Instruction::Move {
                    quantity,
                    from_stack,
                    to_stack,
                });
            }
        }
    }

    let crates = Crates { stacks };
    (crates, instructions)
}

pub fn get_message_from_rearranged_crates(
    initial_state_and_instructions: &str,
    settings: &Settings,
) -> String {
    let (mut crates, instructions) =
        parse_initial_state_and_instructions(initial_state_and_instructions);

    crates.rearrange(&instructions, settings);

    crates.get_message()
}
