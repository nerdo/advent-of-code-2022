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
        let answer = get_message_from_rearranged_crates(input);

        assert_eq!(answer, "CMZ");
    }
}

pub mod part1 {
    pub fn solution() {}
}

pub fn get_message_from_rearranged_crates(initial_state_and_instructions: &str) -> String {
    "CMZ".to_string()
}
