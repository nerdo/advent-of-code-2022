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

pub mod part2 {
    use super::*;
    use std::{env::current_dir, fs::read_to_string};

    pub fn solution() {
        let filename = current_dir().unwrap().join("src/data/day3.txt");
        let input = read_to_string(filename).unwrap();

        let badge_priority_sum = get_badge_priority_sum(&input);

        println!("badge_priority_sum = {badge_priority_sum:#?}");
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

        let misplaced_item_priority = get_item_priority(matching_item_type);

        misplaced_item_priority_sum += misplaced_item_priority as u32;
    }

    misplaced_item_priority_sum
}

fn get_item_priority(item: char) -> u8 {
    match item as u8 {
        b'a'..=b'z' => (item as u8) - b'a' + 1,
        b'A'..=b'Z' => (item as u8) - b'A' + 27,
        _ => panic!("Item is not a letter!"),
    }
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
    let mut badge_priority_sum = 0;

    // Fold lines of 3 into groups.
    let groups = input
        .lines()
        .fold(Vec::<Vec<&str>>::new(), |mut collection, line| {
            match collection.last() {
                Some(group) if group.len() < 3 => (),
                _ => collection.push(Vec::<&str>::new()),
            }

            let group = collection.last_mut().unwrap();

            // Ensure that the shortest string is on top for optimal processing (see below).
            match group.first() {
                Some(rucksack) => match rucksack.len() {
                    existing_len if existing_len > line.len() => group.insert(0, line),
                    _ => group.push(line),
                },
                _ => group.push(line),
            }

            collection
        });

    for group in groups.iter() {
        // Good 'nuff for now, but isn't foolproof for getting rid of invalid/empty groups.
        if group.len() < 3 {
            continue;
        }

        let badge_type = {
            let mut result = None;

            // Since the first line is the shortest (see fold code above),
            // loop through it to find the common item type.
            let smallest_rucksack = group.first().unwrap();
            for (i, _) in smallest_rucksack.char_indices() {
                let c = &smallest_rucksack[i..i + 1];
                if group.get(1).unwrap().contains(&c) && group.get(2).unwrap().contains(&c) {
                    result = Some(c);
                    break;
                }
            }

            result
        }
        .unwrap();

        let badge_priority = get_item_priority(badge_type.chars().next().unwrap());

        badge_priority_sum += badge_priority as u32;
    }

    badge_priority_sum
}
