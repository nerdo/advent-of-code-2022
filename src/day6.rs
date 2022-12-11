use std::collections::{HashSet, VecDeque};
use std::io::Read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_num_characters_processed_for_start_marker_detection_returns_the_correct_answer() {
        let device = HandheldDevice::new();

        assert_eq!(
            device.get_num_characters_processed_for_start_marker_detection(
                "mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes(),
                4
            ),
            7
        );
        assert_eq!(
            device.get_num_characters_processed_for_start_marker_detection(
                "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(),
                4
            ),
            5
        );
        assert_eq!(
            device.get_num_characters_processed_for_start_marker_detection(
                "nppdvjthqldpwncqszvftbrmjlhg".as_bytes(),
                4
            ),
            6
        );
        assert_eq!(
            device.get_num_characters_processed_for_start_marker_detection(
                "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(),
                4
            ),
            10
        );

        assert_eq!(
            device.get_num_characters_processed_for_start_marker_detection(
                "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(),
                4
            ),
            11
        );

        assert_eq!(
            device.get_num_characters_processed_for_start_marker_detection(
                "mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes(),
                14
            ),
            19
        );
        assert_eq!(
            device.get_num_characters_processed_for_start_marker_detection(
                "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(),
                14
            ),
            23
        );
        assert_eq!(
            device.get_num_characters_processed_for_start_marker_detection(
                "nppdvjthqldpwncqszvftbrmjlhg".as_bytes(),
                14
            ),
            23
        );
        assert_eq!(
            device.get_num_characters_processed_for_start_marker_detection(
                "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(),
                14
            ),
            29
        );

        assert_eq!(
            device.get_num_characters_processed_for_start_marker_detection(
                "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(),
                14
            ),
            26
        );
    }
}

pub mod part1 {
    use std::{env::current_dir, fs::read_to_string};

    use super::*;

    pub fn solution() {
        let filename = current_dir().unwrap().join("src/data/day6.txt");
        let input = read_to_string(filename).unwrap();
        let device = HandheldDevice::new();

        let answer =
            device.get_num_characters_processed_for_start_marker_detection(input.as_bytes(), 4);

        println!("day 6 part 1 answer = {answer:#?}");
    }
}

pub mod part2 {
    use std::{env::current_dir, fs::read_to_string};

    use super::*;

    pub fn solution() {
        let filename = current_dir().unwrap().join("src/data/day6.txt");
        let input = read_to_string(filename).unwrap();
        let device = HandheldDevice::new();

        let answer =
            device.get_num_characters_processed_for_start_marker_detection(input.as_bytes(), 14);

        println!("day 6 part 2 answer = {answer:#?}");
    }
}

pub struct HandheldDevice {}

impl Default for HandheldDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl HandheldDevice {
    pub fn new() -> Self {
        HandheldDevice {}
    }

    // Report the number of characters from the beginning of the buffer to the end of the first such four-character marker.
    pub fn get_num_characters_processed_for_start_marker_detection(
        &self,
        mut datastream: impl Read,
        num_unique_bytes: usize,
    ) -> usize {
        let mut bytes_read = 0usize;
        let mut current_sequence = VecDeque::<u8>::new();

        loop {
            let mut buffer: [u8; 1] = [0];
            if let Err(e) = datastream.read(&mut buffer) {
                panic!("{}", e);
            }
            bytes_read += 1;

            current_sequence.push_back(buffer[0]);

            if bytes_read > num_unique_bytes {
                current_sequence.pop_front();
            }

            let byte_set: HashSet<u8> = current_sequence.clone().into_iter().collect();

            if bytes_read > num_unique_bytes && byte_set.len() == num_unique_bytes {
                return bytes_read;
            }
        }
    }
}
