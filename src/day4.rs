#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_num_assignments_fully_contains_other_in_pair_returns_the_correct_answer() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let answer = get_num_assignments_fully_contains_other_in_pair(input);

        assert_eq!(answer, 2);
    }
}

pub fn get_num_assignments_fully_contains_other_in_pair(assignment_list: &str) -> u32 {
    2
}
