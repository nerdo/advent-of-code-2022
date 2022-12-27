//! Advent of Code Day 11
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]

use std::cell::RefCell;

use anyhow::{bail, Context, Error};

// #[cfg(test)]
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

        let answer = monkey_sim.get_monkey_business_level(20, 2, WorryManager::Constant(3.0))?;

        assert_eq!(answer, 10605);

        Ok(())
    }

    // #[test]
    pub fn get_monkey_business_level_returns_the_correct_answer_after_10000_rounds(
    ) -> Result<(), Error> {
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

        let answer = monkey_sim.get_monkey_business_level(20, 2, WorryManager::Dynamic)?;

        assert_eq!(answer, 2713310158);

        Ok(())
    }
}

/// Simulates monkey business.
#[derive(Debug)]
pub struct MonkeySim {
    /// The initial state of the monkeys.
    initial_monkey_states: Vec<MonkeyState>,
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
                    monkeys.push(monkey);
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
                    num_insepctions: 0,
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
                            item_str.trim().parse::<u64>().with_context(|| {
                                format!("Parsing starting items from line {}: '{}'", line, item_str)
                            })
                        })
                        .collect::<Result<Vec<u64>, Error>>()?
                };

                monkey.items = RefCell::new(items);
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
                                    Operand::Constant(number_str.trim().parse::<u64>()?)
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
            } else if line.starts_with("Test: divisible by ") {
                let Some(ref mut monkey) = *cell.borrow_mut() else {
                    bail!(
                        "No monkey initialized - parsing test operand from line {}",
                        line
                    );
                };

                let (_, end_of_line) = line.split_at("Test: divisible by ".len());
                let test = end_of_line.trim().parse::<u64>()?;

                monkey.test = test;
            } else if line.starts_with("If true: throw to monkey ") {
                let Some(ref mut monkey) = *cell.borrow_mut() else {
                    bail!(
                        "No monkey initialized - parsing test operand from line {}",
                        line
                    );
                };

                let (_, end_of_line) = line.split_at("If true: throw to monkey ".len());
                let target_monkey_number = end_of_line.trim().parse::<usize>()?;

                monkey.test_pass_monkey_number = target_monkey_number;
            } else if line.starts_with("If false: throw to monkey ") {
                let Some(ref mut monkey) = *cell.borrow_mut() else {
                    bail!(
                        "No monkey initialized - parsing test operand from line {}",
                        line
                    );
                };

                let (_, end_of_line) = line.split_at("If false: throw to monkey ".len());
                let target_monkey_number = end_of_line.trim().parse::<usize>()?;

                monkey.test_fail_monkey_number = target_monkey_number;
            }
        }

        // Don't forget that last monkey!
        if let Some(monkey) = cell.borrow_mut().take() {
            monkeys.push(monkey);
        }

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
    /// * `worry_level_divisor` - Number to divivide the worry level by after each monkey inspects an item.
    pub fn get_monkey_business_level(
        &self,
        num_rounds: usize,
        top_n: usize,
        worry_manager: WorryManager,
    ) -> Result<u64, Error> {
        let mut num_monkey_inspections: Vec<(usize, u64)> = {
            let round = RefCell::new(
                self.initial_monkey_states
                    .clone()
                    .into_iter()
                    .map(RefCell::new)
                    .collect::<Vec<RefCell<MonkeyState>>>(),
            );

            for round_number in 1..=num_rounds {
                // Alter the worry level since it didn't break...
                let worry_level_divisor = match worry_manager {
                    WorryManager::Constant(n) => n,
                    WorryManager::Dynamic => {
                        1.0
                        // I think it has to do with prime numbers... All the test divisors
                        // are prime, so maybe the way to keep it under control (and in
                        // line with the sample output) is to find the largest prime
                        // smaller than the round number?
                        // let mut prime_number = u32::try_from(round_number)?;
                        // let is_prime = |n| {
                        //     if n < 2 {
                        //         return true;
                        //     }
                        //     for test in (2..n - 1).rev() {
                        //         if n % test == 0 {
                        //             return false;
                        //         }
                        //     }
                        //     true
                        // };

                        // while !is_prime(prime_number) {
                        //     prime_number -= 1;
                        // }

                        // prime_number = prime_number.max(1);
                        // println!("prime = {}", prime_number);
                        // f64::try_from(prime_number)?
                    }
                };

                for monkey in round.borrow().iter() {
                    // Grab the number of inspections about to happen (because the list will get
                    // drained.
                    let num_inspections = u64::try_from(monkey.borrow().items.borrow().len())?;

                    // Inspect each item.
                    for item in monkey.borrow().items.borrow_mut().drain(..) {
                        let monkey = monkey.borrow();

                        let Some(ref operation) = monkey.operation else {
                            bail!("No operation found for monkey #{}, round #{}.", monkey.number, round_number);
                        };
                        let mut worry_level = operation.execute(item);

                        println!("worry level = {}", worry_level);
                        worry_level = (worry_level as f64 / worry_level_divisor).floor() as u64;

                        // Figure out which monkey will receive this item.
                        let recipient_monkey_number = if worry_level % monkey.test == 0 {
                            monkey.test_pass_monkey_number
                        } else {
                            monkey.test_fail_monkey_number
                        };

                        // Update the recipient's list.
                        round
                            .borrow()
                            .get(recipient_monkey_number)
                            .with_context(|| {
                                format!(
                                    "Trying to get recipient monkey #{} to throw item to.",
                                    recipient_monkey_number
                                )
                            })?
                            .borrow_mut()
                            .items
                            .borrow_mut()
                            .push(worry_level);
                    }

                    monkey.borrow_mut().num_insepctions += num_inspections;
                }

                {
                    let num_monkey_inspections = round
                        .borrow()
                        .iter()
                        .map(|m| {
                            let m = m.borrow();
                            let num_inspections = m.num_insepctions;
                            num_inspections
                        })
                        .collect::<Vec<u64>>();
                    println!("Round #{} = {:?}", round_number, num_monkey_inspections);
                }
            }

            let num_monkey_inspections = round
                .borrow()
                .iter()
                .map(|m| {
                    let m = m.borrow();
                    let monkey_number = m.number;
                    let num_inspections = m.num_insepctions;
                    (monkey_number, num_inspections)
                })
                .collect();

            num_monkey_inspections
        };

        num_monkey_inspections.sort_by(|a, b| (b.1).cmp(&a.1));
        let top_monkeys = num_monkey_inspections.iter().take(top_n);
        let monkey_business_level = top_monkeys.fold(1u64, |product, tuple| product * tuple.1);

        Ok(monkey_business_level)
    }
}

/// The state of a monkey.
#[derive(Debug, Clone)]
pub struct MonkeyState {
    /// The number identifying this monkey.
    number: usize,

    /// The list of items this monkey has.
    items: RefCell<Vec<u64>>,

    /// Operation performed to calculate your new worry level.
    operation: Option<Operation>,

    /// Test value used to determine which monkey your stuff gets thrown to.
    test: u64,

    /// The monkey number that gets your item if the test passes.
    test_pass_monkey_number: usize,

    /// The monkey number that gets your item if the test fails.
    test_fail_monkey_number: usize,

    /// The number of items this monkey has inspected.
    num_insepctions: u64,
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
    pub fn execute(&self, value: u64) -> u64 {
        let get_operands = |first_operand: &Operand, second_operand: &Operand| {
            (
                match first_operand {
                    Operand::Constant(n) => *n,
                    Operand::OldValue => value,
                },
                match second_operand {
                    Operand::Constant(n) => *n,
                    Operand::OldValue => value,
                },
            )
        };

        match self {
            Self::Product(first_operand, second_operand) => {
                let (a, b) = get_operands(first_operand, second_operand);
                a * b
            }
            Self::Addition(first_operand, second_operand) => {
                let (a, b) = get_operands(first_operand, second_operand);
                a + b
            }
        }
    }
}

/// Repersents an operand in an operation.
#[derive(Debug, Clone)]
pub enum Operand {
    /// A constant value.
    Constant(u64),

    /// The old value.
    OldValue,
}

/// Worry manager.
#[derive(Debug)]
pub enum WorryManager {
    /// A constant value to divide the worries by.
    Constant(f64),

    /// My dynamic solution.
    Dynamic,
}

/// Part 1
pub mod part1 {
    use std::{env::current_dir, fs::read_to_string};

    use super::*;

    /// The solution for Day 11 Part 1
    pub fn solution() -> Result<(), Error> {
        let filename = current_dir()?.join("src/data/day11.txt");
        let input = read_to_string(filename)?;

        let monkey_sim = MonkeySim::parse(&input)?;

        let answer = monkey_sim.get_monkey_business_level(20, 2, WorryManager::Constant(3.0))?;

        println!("Day 11 Solution = {}", answer);

        Ok(())
    }
}

/// Part 2
pub mod part2 {
    use std::{env::current_dir, fs::read_to_string};

    use super::*;

    /// The solution for Day 11 Part 1
    pub fn solution() -> Result<(), Error> {
        tests::get_monkey_business_level_returns_the_correct_answer_after_10000_rounds()?;
        // let filename = current_dir()?.join("src/data/day11.txt");
        // let input = read_to_string(filename)?;

        // let monkey_sim = MonkeySim::parse(&input)?;

        // let answer = monkey_sim.get_monkey_business_level(20, 2, WorryManager::Dynamic)?;

        // println!("Day 11 Solution = {}", answer);

        Ok(())
    }
}
