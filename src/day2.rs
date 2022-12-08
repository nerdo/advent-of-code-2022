#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculated_rock_paper_scissors_tournament_score_returns_the_correct_score() {
        let input = "A Y
B X
C Z

";
        let total_score = calculated_rock_paper_scissors_tournament_score(&input);

        assert_eq!(total_score, 15);
    }
}

#[derive(Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
struct Round {
    my_shape: Shape,
    their_shape: Shape,
    my_score: u32,
}

impl TryFrom<&str> for Round {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts: Vec<&str> = value.split(' ').collect();

        let my_token = parts.pop().unwrap();
        let their_token = parts.pop().unwrap();

        let my_shape = Shape::try_from(my_token)?;
        let their_shape = Shape::try_from(their_token)?;

        let my_score = my_shape.point_value() + my_shape.score_against(&their_shape);

        Ok(Round {
            my_shape,
            their_shape,
            my_score,
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
}

pub fn calculated_rock_paper_scissors_tournament_score(input: &str) -> u32 {
    let mut total_score = 0;

    for line in input.lines() {
        let round = Round::try_from(line).unwrap();
        total_score += round.my_score;
    }

    total_score
}
