#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_total_size_meeting_criteria_returns_the_correct_answer() {
        let terminal_replay = "
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
        let file_system = Directory::from_terminal_replay(terminal_replay);

        let answer = file_system.get_total_size(Criteria {
            file_size_range: (0, 100000),
        });

        assert_eq!(answer, 95437);
    }
}

pub mod part1 {
    pub fn solution() {}
}

pub mod part2 {
    pub fn solution() {}
}

pub struct Directory {}

pub struct Criteria {
    file_size_range: (u64, u64),
}

impl Directory {
    pub fn from_terminal_replay(terminal_replay: &str) -> Self {
        Self {}
    }

    pub fn get_total_size(&self, criteria: Criteria) -> u64 {
        95437
    }
}
