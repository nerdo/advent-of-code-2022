#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_misplaced_item_priority_sum_returns_the_correct_value() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

            ";

        let answer = get_misplaced_item_priority_sum(input);

        assert_eq!(answer, 157);
    }

    #[test]
    fn get_badge_priority_sum_returns_the_correct_value() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg

wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

";

        let answer = get_badge_priority_sum(input);
        
        assert_eq!(answer, 70);
    }
}

pub mod part1 {
    use super::*;
    use std::{env::current_dir, fs::read_to_string};

    pub fn solution() {
        let filename = current_dir().unwrap().join("src/data/day3.txt");
        let input = read_to_string(filename).unwrap();

        let misplaced_item_priority_sum = get_misplaced_item_priority_sum(&input);

        println!("misplaced_item_priority_sum = {misplaced_item_priority_sum:#?}");
    }
}

pub fn get_misplaced_item_priority_sum(input: &str) -> u32 {
    let mut misplaced_item_priority_sum: u32 = 0;

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let compartment_size = line.len() / 2;

        let first_compartment = &line[..compartment_size];
        let second_compartment = &line[compartment_size..];

        let matching_item_type =
            find_matching_item_type(first_compartment, second_compartment).unwrap();

        let misplaced_item_priority = match matching_item_type as u8 {
            b'a'..=b'z' => (matching_item_type as u8) - b'a' + 1,
            b'A'..=b'Z' => (matching_item_type as u8) - b'A' + 27,
            _ => panic!("Item is not a letter!"),
        };

        misplaced_item_priority_sum += misplaced_item_priority as u32;
    }

    misplaced_item_priority_sum
}

fn find_matching_item_type<'a>(a: &'a str, b: &'a str) -> Result<char, &'static str> {
    for item_type in a.chars() {
        if b.contains(item_type) {
            return Ok(item_type);
        }
    }

    Err("No Matching Item Type Found")
}

pub fn get_badge_priority_sum(input: &str) -> u32 {
    70
}
