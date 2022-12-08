#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_most_calories_should_return_the_largest_sum_of_calories() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

";

        let elf_with_most = get_most_calories(input).unwrap();

        assert_eq!(elf_with_most.calories, 24000);
    }

    #[test]
    fn get_top_elves_should_return_the_top_n_elves_that_have_the_most_calories() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

";

        let top_elves: Vec<Elf> = get_top_elves(input, 3);

        assert_eq!(top_elves.len(), 3);
        assert_eq!(top_elves.get(0).unwrap().calories, 24000);
        assert_eq!(top_elves.get(1).unwrap().calories, 11000);
        assert_eq!(top_elves.get(2).unwrap().calories, 10000);
    }
}

use std::env;

pub mod part1 {
    use super::*;

    pub fn solution() {
        let filename = env::current_dir().unwrap().join("src/data/day1.txt");
        let input = std::fs::read_to_string(filename).unwrap();

        let elf_with_most = get_most_calories(&input).unwrap();

        println!("{elf_with_most:#?}");
    }
}

pub mod part2 {
    use super::*;

    pub fn solution() {
        let filename = env::current_dir().unwrap().join("src/data/day1.txt");
        let input = std::fs::read_to_string(filename).unwrap();

        let top_elves = get_top_elves(&input, 3);
        let total_calories = top_elves
            .iter()
            .map(|elf| elf.calories)
            .reduce(|sum, calories| sum + calories)
            .unwrap();

        println!("{top_elves:#?}");
        println!("total calories = {}", total_calories);
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Elf {
    pub calories: i32,
}

impl Elf {
    pub fn from(calorie_list: &str) -> (Elf, &str) {
        let mut elf = Elf { calories: 0 };
        let mut offset = 0;

        for line in calorie_list.lines() {
            if line.is_empty() {
                // Empty lines consume a newline character
                offset += 1;
                break;
            }

            // +1 because each line consumes a newline character
            // (except maybe the last one, but it's fine to act as if it is)
            offset += line.len() + 1;

            let calories = match line.parse::<i32>() {
                Ok(c) => c,
                Err(e) => {
                    println!("{e:#?}");
                    continue;
                }
            };

            elf.calories += calories;
        }

        (elf, &calorie_list[offset..])
    }
}

pub fn get_most_calories(calorie_list: &str) -> Option<Elf> {
    let mut elf_with_most: Option<Elf> = None;
    let mut remaining_calorie_list = calorie_list;

    loop {
        let (current_elf, calorie_list) = Elf::from(remaining_calorie_list);
        remaining_calorie_list = calorie_list;

        elf_with_most = match elf_with_most {
            None => Some(current_elf),
            Some(elf) if elf.calories < current_elf.calories => Some(current_elf),
            Some(_) => elf_with_most,
        };

        if calorie_list.is_empty() {
            break;
        }
    }

    elf_with_most
}

pub fn get_top_elves(calorie_list: &str, top_n: usize) -> Vec<Elf> {
    let mut elves = Vec::<Elf>::new();

    let index_of_smaller_elf = |elves: &Vec<Elf>, calories| {
        for (i, elf) in elves.iter().enumerate() {
            if calories > elf.calories {
                return i;
            }
        }
        elves.len()
    };

    let mut remaining_calorie_list = calorie_list;

    loop {
        let (current_elf, calorie_list) = Elf::from(remaining_calorie_list);
        remaining_calorie_list = calorie_list;

        let insert_index = index_of_smaller_elf(&elves, current_elf.calories);
        if insert_index < top_n {
            elves.insert(insert_index, current_elf);
            if elves.len() > top_n {
                elves.pop();
            }
        }

        if calorie_list.is_empty() {
            break;
        }
    }

    elves
}
