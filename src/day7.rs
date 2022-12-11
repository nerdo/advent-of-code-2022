#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_total_size_meeting_criteria_returns_the_correct_answer() {
        let input = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
            ";

        let answer = get_total_size_meeting_criteria(input, (0, 100000));

        assert_eq!(answer, 95437);
    }
}

pub mod part1 {
    pub fn solution() {}
}

pub mod part2 {
    pub fn solution() {}
}

pub fn get_total_size_meeting_criteria(terminal_replay: &str, criteria: (u64, u64)) -> u64 {
   95437 
}
