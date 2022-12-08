#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculated_rock_paper_scissors_tournament_score_with_my_strategy_returns_the_correct_score() {
        let input = "A Y
B X
C Z

";
        let total_score = calculated_rock_paper_scissors_tournament_score_with_my_strategy(&input);

        assert_eq!(total_score, 15);
    }

    #[test]
    fn calculated_rock_paper_scissors_tournament_score_with_elf_strategy_returns_the_correct_score()
    {
        let input = "A Y
B X
C Z

";
        let total_score = calculated_rock_paper_scissors_tournament_score_with_elf_strategy(&input);

        assert_eq!(total_score, 12);
    }
}

pub mod part1 {
    use super::*;
    use std::{env::current_dir, fs::read_to_string};

    pub fn solution() {
        let filename = current_dir().unwrap().join("src/data/day2.txt");
        let input = read_to_string(filename).unwrap();

        let total_score = calculated_rock_paper_scissors_tournament_score_with_my_strategy(&input);

        println!("Total Score = {}", total_score);
    }
}

pub mod part2 {
    use super::*;
    use std::{env::current_dir, fs::read_to_string};

    pub fn solution() {
        let filename = current_dir().unwrap().join("src/data/day2.txt");
        let input = read_to_string(filename).unwrap();

        let total_score = calculated_rock_paper_scissors_tournament_score_with_elf_strategy(&input);

        println!("Total Score = {}", total_score);
    }
}

#[derive(Debug, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum Goal {
    Lose,
    Draw,
    Win,
}

impl TryFrom<&str> for Goal {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err("Invalid Goal Token"),
        }
    }
}

#[derive(Debug)]
struct Round {
    _my_shape_my_strategy: Shape,
    _my_shape_elf_strategy: Shape,
    _their_shape: Shape,
    my_score_my_strategy: u32,
    my_score_elf_strategy: u32,
}

impl TryFrom<&str> for Round {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts: Vec<&str> = value.split(' ').collect();

        let my_token = parts.pop().unwrap();
        let their_token = parts.pop().unwrap();

        let my_shape_my_strategy = Shape::try_from(my_token)?;
        let their_shape = Shape::try_from(their_token)?;

        let my_goal = Goal::try_from(my_token)?;
        let my_shape_elf_strategy = Shape::meets_goal(&my_goal, &their_shape);

        let my_score_my_strategy =
            my_shape_my_strategy.point_value() + my_shape_my_strategy.score_against(&their_shape);
        let my_score_elf_strategy =
            my_shape_elf_strategy.point_value() + my_shape_elf_strategy.score_against(&their_shape);

        Ok(Round {
            _my_shape_my_strategy: my_shape_my_strategy,
            _my_shape_elf_strategy: my_shape_elf_strategy,
            _their_shape: their_shape,
            my_score_my_strategy,
            my_score_elf_strategy,
        })
    }
}

impl TryFrom<&str> for Shape {
    type Error = &'static str;

    fn try_from(token: &str) -> Result<Self, Self::Error> {
        match token {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err("Invalid Shape Token"),
        }
    }
}

impl Shape {
    fn score_against(&self, opponent: &Self) -> u32 {
        match self {
            Shape::Rock => match opponent {
                Shape::Rock => 3,
                Shape::Paper => 0,
                Shape::Scissors => 6,
            },
            Shape::Paper => match opponent {
                Shape::Rock => 6,
                Shape::Paper => 3,
                Shape::Scissors => 0,
            },
            Shape::Scissors => match opponent {
                Shape::Rock => 0,
                Shape::Paper => 6,
                Shape::Scissors => 3,
            },
        }
    }

    fn point_value(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn meets_goal(goal: &Goal, opponent: &Self) -> Self {
        match goal {
            Goal::Lose => match opponent {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Goal::Draw => opponent.clone(),
            Goal::Win => match opponent {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
        }
    }
}

pub fn calculated_rock_paper_scissors_tournament_score_with_my_strategy(input: &str) -> u32 {
    let mut total_score = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let round = Round::try_from(line).unwrap();
        total_score += round.my_score_my_strategy;
    }

    total_score
}

pub fn calculated_rock_paper_scissors_tournament_score_with_elf_strategy(input: &str) -> u32 {
    let mut total_score = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let round = Round::try_from(line).unwrap();
        total_score += round.my_score_elf_strategy;
    }

    total_score
}
