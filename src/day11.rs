//! Advent of Code Day 11
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

use std::cell::RefCell;

use anyhow::{bail, Context, Error};

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
pub struct MonkeySim {
    /// The initial state of the monkeys.
    initial_monkey_states: Vec<RefCell<MonkeyState>>,
}

impl MonkeySim {
    /// Parses text input to build the initial state for the monkey simulator.
    ///
    /// # Arguments
    ///
    /// * `input` - The string description of the monkeys' initial state.
    pub fn parse(input: &str) -> Result<Self, Error> {
        let mut monkeys = vec![];
        let mut cell = RefCell::new(None);

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with("Monkey ") {
                // We're done with the previous monkey, add it to our list.
                if let Some(monkey) = cell.borrow_mut().take() {
                    monkeys.push(RefCell::new(monkey));
                }

                // Get the current monkey number from the line.
                let monkey_number = {
                    let (_, end_of_line) = line.split_at("Monkey ".len());
                    let parts = end_of_line.split(':').collect::<Vec<&str>>();

                    parts
                        .first()
                        .with_context(|| format!("Parsing monkey number from line {}", line))?
                        .parse::<usize>()?
                };

                // Reset the variables.
                cell = RefCell::new(Some(MonkeyState {
                    number: monkey_number,
                    items: RefCell::new(vec![]),
                    operation: None,
                    test: 0,
                    test_fail_monkey_number: 0,
                    test_pass_monkey_number: 0,
                    num_insepctions: RefCell::new(0),
                }));
            } else if line.starts_with("Starting items: ") {
                let Some(ref mut monkey) = *cell.borrow_mut() else {
                    bail!(
                        "No monkey initialized - parsing starting items from line {}",
                        line
                    );
                };

                // Parse items.
                let items = {
                    let (_, end_of_line) = line.split_at("Starting items: ".len());
                    let items_str = end_of_line.split(',').collect::<Vec<&str>>();
                    items_str
                        .iter()
                        .map(|item_str| {
                            item_str.trim().parse::<u32>().with_context(|| {
                                format!("Parsing starting items from line {}: '{}'", line, item_str)
                            })
                        })
                        .collect::<Result<Vec<u32>, Error>>()?
                };

                *monkey.items.borrow_mut() = items;
            } else if line.starts_with("Operation: new = ") {
                let Some(ref mut monkey) = *cell.borrow_mut() else {
                    bail!(
                        "No monkey initialized - parsing operation from line {}",
                        line
                    );
                };

                let operation = {
                    let (_, end_of_line) = line.split_at("Operation: new = ".len());
                    let parts = end_of_line.split(' ').collect::<Vec<&str>>();

                    let (a, b) = {
                        let get_operand = |input: Option<&str>| {
                            Ok(match input {
                                None => {
                                    bail!(
                                        "No operands found in the operation in the line: {}",
                                        line
                                    )
                                }
                                Some(s) if s == "old" => Operand::OldValue,
                                Some(number_str) => {
                                    Operand::Constant(number_str.trim().parse::<u32>()?)
                                }
                            })
                        };

                        let a = get_operand(parts.first().copied())?;
                        let b = get_operand(parts.last().copied())?;

                        (a, b)
                    };

                    match parts.get(1) {
                        None => bail!("No operator found in the operation in the line: {}.", line),
                        Some(s) if s == &"*" => Operation::Product(a, b),
                        Some(s) if s == &"+" => Operation::Addition(a, b),
                        _ => bail!("Unsupported operation gound on the line: {}.", line),
                    }
                };

                monkey.operation = Some(operation);
            }
        }

        // Don't forget that last monkey!
        if let Some(monkey) = cell.borrow_mut().take() {
            monkeys.push(RefCell::new(monkey));
        }

        println!("Monkeys: {:#?}", monkeys);

        Ok(MonkeySim {
            initial_monkey_states: monkeys,
        })
    }

    /// Gets the level of monkey business after r rounds have been simulated.
    ///
    /// # Arguments
    ///
    /// * `num_rounds` - Number of rounds to simulate.
    /// * `top_n` - Number of top active monkeys to consider in the monkey business level.
    pub fn get_monkey_business_level(&self, num_rounds: usize, top_n: usize) -> Result<u32, Error> {
        let mut num_monkey_inspections: Vec<(usize, u32)> = {
            let mut last_round = Box::new(&self.initial_monkey_states);

            for round_number in 1..=num_rounds {
                let new_round = last_round.clone();

                for monkey in *new_round {
                    let monkey = monkey.borrow();

                    // Inspect each item.
                    for item in monkey.items.borrow_mut().drain(..) {
                        let Some(ref operation) = monkey.operation else {
                            bail!("No operation found for monkey #{}, round #{}.", monkey.number, round_number);
                        };
                        let mut worry_level = operation.execute(item)?;

                        // Alter the worry level since it didn't break...
                        worry_level = (f64::from(worry_level) / 3.0).floor() as u32;

                        // Figure out which monkey will receive this item.
                        let recipient_monkey_number = if worry_level % monkey.test == 0 {
                            monkey.test_pass_monkey_number
                        } else {
                            monkey.test_fail_monkey_number
                        };

                        // Update the recipient's list.
                        {
                            let recipient_monkey = new_round
                                .get(recipient_monkey_number)
                                .with_context(|| format!("Tried to grab monkey #{} in round #{} from list of monkeys with len = {}.", recipient_monkey_number, round_number, new_round.len()))?;
                            let recipient_monkey = recipient_monkey.borrow_mut();
                            recipient_monkey.items.borrow_mut().push(worry_level);
                        }
                    }

                    *monkey.num_insepctions.borrow_mut() +=
                        u32::try_from(monkey.items.borrow().len())?;
                }

                last_round = new_round;
            }

            // Shadow last_round with the unboxed, immmutable reference.
            let last_round = *last_round;

            let num_monkey_inspections = last_round
                .iter()
                .map(|m| {
                    let monkey_number = m.borrow().number;
                    let num_inspections = *m.borrow().num_insepctions.borrow();
                    (monkey_number, num_inspections)
                })
                .collect();

            num_monkey_inspections
        };

        num_monkey_inspections.sort_by(|a, b| (b.1).cmp(&a.1));
        let top_monkeys = num_monkey_inspections.iter().take(top_n);
        let monkey_business_level = top_monkeys.fold(1u32, |product, tuple| product * tuple.1);

        Ok(monkey_business_level)
    }
}

/// The state of a monkey.
#[derive(Debug, Clone)]
pub struct MonkeyState {
    /// The number identifying this monkey.
    number: usize,

    /// The list of items this monkey has.
    items: RefCell<Vec<u32>>,

    /// Operation performed to calculate your new worry level.
    operation: Option<Operation>,

    /// Test value used to determine which monkey your stuff gets thrown to.
    test: u32,

    /// The monkey number that gets your item if the test fails.
    test_fail_monkey_number: usize,

    /// The monkey number that gets your item if the test passes.
    test_pass_monkey_number: usize,

    /// The number of items this monkey has inspected.
    num_insepctions: RefCell<u32>,
}

/// Represents an operation that calculates your new worry level.
#[derive(Debug, Clone)]
pub enum Operation {
    /// The addition of two operands.
    Addition(Operand, Operand),

    /// The product of two operands.
    Product(Operand, Operand),
}

impl Operation {
    /// Executes the operation on the value.
    ///
    /// # Arguments
    ///
    /// `value` - The value to execute the operation on.
    pub fn execute(&self, value: u32) -> Result<u32, Error> {
        Ok(match self {
            _ => todo!(),
        })
    }
}

/// Repersents an operand in an operation.
#[derive(Debug, Clone)]
pub enum Operand {
    /// A constant value.
    Constant(u32),

    /// The old value.
    OldValue,
}

/// Part 1
pub mod part1 {
    use super::*;

    /// The solution for Day 11 Part 1
    pub fn solution() -> Result<(), Error> {
        Ok(())
    }
}
