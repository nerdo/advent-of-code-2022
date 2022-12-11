use std::collections::HashSet;
use std::io::Read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_num_characters_processed_for_start_of_packet_marker_detection_returns_the_correct_answer(
    ) {
        let device = HandheldDevice::new();

        assert_eq!(
            device.get_num_characters_processed_for_start_of_packet_marker_detection(
                "mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes()
            ),
            7
        );
        assert_eq!(
            device.get_num_characters_processed_for_start_of_packet_marker_detection(
                "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()
            ),
            5
        );
        assert_eq!(
            device.get_num_characters_processed_for_start_of_packet_marker_detection(
                "nppdvjthqldpwncqszvftbrmjlhg".as_bytes()
            ),
            6
        );
        assert_eq!(
            device.get_num_characters_processed_for_start_of_packet_marker_detection(
                "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()
            ),
            10
        );

        assert_eq!(
            device.get_num_characters_processed_for_start_of_packet_marker_detection(
                "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()
            ),
            11
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

        let answer = device.get_num_characters_processed_for_start_of_packet_marker_detection(input.as_bytes());

        println!("day 6 part 1 answer = {answer:#?}");
    }
}

pub mod part2 {
    pub fn solution() {}
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
    pub fn get_num_characters_processed_for_start_of_packet_marker_detection(
        &self,
        mut datastream: impl Read,
    ) -> usize {
        let mut bytes_read = 0usize;
        let mut last_word: [u8; 4] = [0, 0, 0, 0];
        let mut byte_set = HashSet::new();

        loop {
            let mut buffer: [u8; 1] = [0];
            if let Err(e) = datastream.read(&mut buffer) {
                panic!("{}", e);
            }
            bytes_read += 1;

            match bytes_read {
                0..=4 => last_word[bytes_read - 1] = buffer[0],
                _ => {
                    last_word[0] = last_word[1];
                    last_word[1] = last_word[2];
                    last_word[2] = last_word[3];
                    last_word[3] = buffer[0];
                }
            }

            byte_set.clear();
            byte_set.insert(last_word[0]);
            byte_set.insert(last_word[1]);
            byte_set.insert(last_word[2]);
            byte_set.insert(last_word[3]);

            if bytes_read > 4 && byte_set.len() == 4 {
                return bytes_read;
            }
        }
    }
}
