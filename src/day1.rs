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

        let elf_with_most = get_most_calories(&input).unwrap();

        assert_eq!(elf_with_most.calories, 24000);
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

#[derive(Debug, PartialEq)]
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

        if calorie_list.len() == 0 {
            break;
        }
    }

    elf_with_most
}
