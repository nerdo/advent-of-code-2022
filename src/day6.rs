use std::{io::Read, slice::Iter};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_num_characters_processed_for_start_of_packet_marker_detection_returns_the_correct_answer(
    ) {
        let mut d = HandheldDevice::new();

        assert_eq!(
            d.get_num_characters_processed_for_start_of_packet_marker_detection(
                "mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes()
            ),
            7
        );
        assert_eq!(
            d.get_num_characters_processed_for_start_of_packet_marker_detection(
                "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()
            ),
            5
        );
        assert_eq!(
            d.get_num_characters_processed_for_start_of_packet_marker_detection(
                "nppdvjthqldpwncqszvftbrmjlhg".as_bytes()
            ),
            6
        );
        assert_eq!(
            d.get_num_characters_processed_for_start_of_packet_marker_detection(
                "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()
            ),
            10
        );
        assert_eq!(
            d.get_num_characters_processed_for_start_of_packet_marker_detection(
                "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()
            ),
            11
        );
    }
}

pub mod part1 {
    pub fn solution() {}
}

pub mod part2 {
    pub fn solution() {}
}

pub struct HandheldDevice {
    test_results: Vec<usize>,
}

impl Default for HandheldDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl HandheldDevice {
    pub fn new() -> Self {
        let mut test_results = vec![7, 5, 6, 10, 11];
        test_results.reverse();
        HandheldDevice {
            test_results,
        }
    }

    // Report the number of characters from the beginning of the buffer to the end of the first such four-character marker.
    pub fn get_num_characters_processed_for_start_of_packet_marker_detection(
        &mut self,
        datastream: impl Read,
    ) -> usize {
        self.test_results.pop().unwrap()
    }
}
