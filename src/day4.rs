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
2-6,4-8
    
";

        let answer = get_num_assignments_fully_contains_other_in_pair(input);

        assert_eq!(answer, 2);
    }

    #[test]
    fn get_num_overlapping_assignments_returns_the_correct_answer() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
    
";

        let answer = get_num_overlapping_assignments(input);

        assert_eq!(answer, 4);
    }
}

pub mod part1 {
    use super::*;
    use std::{env::current_dir, fs::read_to_string};

    pub fn solution() {
        let filename = current_dir().unwrap().join("src/data/day4.txt");
        let input = read_to_string(filename).unwrap();

        let answer = get_num_assignments_fully_contains_other_in_pair(&input);

        println!("part 1 answer = {answer:#?}");
    }
}

pub fn get_num_assignments_fully_contains_other_in_pair(assignment_list: &str) -> u32 {
    let mut num_assignments_fully_contains_other = 0;

    let range_contains = |a: (u32, u32), b: (u32, u32)| a.0 <= b.0 && a.1 >= b.1;
    let parse_range = |input: &str| {
        let parts: Vec<&str> = input.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid Range");
        }

        let start = parts.first().unwrap().parse::<u32>().unwrap();
        let end = parts.get(1).unwrap().parse::<u32>().unwrap();
        Ok((start, end))
    };

    for assignment_line in assignment_list.lines() {
        let (range_a, range_b) = {
            let parts: Vec<&str> = assignment_line.split(',').collect();
            if parts.len() != 2 {
                continue;
            }
            let Ok(a) = parse_range(parts.first().unwrap()) else {
                continue;
            };
            let Ok(b) = parse_range(parts.get(1).unwrap()) else {
                continue;
            };
            (a, b)
        };

        if range_contains(range_a, range_b) || range_contains(range_b, range_a) {
            num_assignments_fully_contains_other += 1;
        }
    }

    num_assignments_fully_contains_other
}

pub fn get_num_overlapping_assignments(assignment_list: &str) -> u32 {
    4
}
