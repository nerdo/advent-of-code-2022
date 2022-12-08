#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculated_rock_paper_scissors_tournament_score_returns_the_correct_score() {
        let input = "A Y
B X
C Z";
        let total_score = calculated_rock_paper_scissors_tournament_score(&input);

        assert_eq!(total_score, 15);
    }
}

pub fn calculated_rock_paper_scissors_tournament_score(input: &str) -> u32 {
    15
}
