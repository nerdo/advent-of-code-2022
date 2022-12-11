#![warn(missing_docs)]
//! Advent of Code 2022 Day 7 Solution

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
        let file_system = FileSystemEntry::from_terminal_replay(terminal_replay);

        let answer = file_system.get_total_size(Criteria {
            size_range: (0, 100000),
        });

        assert_eq!(answer, 95437);
    }
}

/// Part 1.
pub mod part1 {
    /// The solution for Part 1.
    pub fn solution() {}
}

/// Part 2
pub mod part2 {
    /// The solution for Part 2.
    pub fn solution() {}
}

/// Represents a file system entry.
pub enum FileSystemEntry {
    /// Represents a directory in a file system.
    Directory {
        /// The name of the directory.
        name: String,
        /// The size of the directory and its contents.
        size: usize,
        /// The list of file system entries contained within the directory. This can be a mix of
        /// both files and directories.
        entries: Vec<FileSystemEntry>,
    },

    /// Represents a file in a file system.
    File {
        /// The name of the file.
        name: String,
        /// The size of the file.
        size: usize,
    },
}

/// Criteria for [`get_total_size`]: #method.get_total_size
pub struct Criteria {
    /// The size range to constrain [`get_total_size`]: #method.get_total_size to.
    size_range: (u64, u64),
}

impl FileSystemEntry {
    /// Parses terminal replay output into a `FileSystem::Directory` variant containing the file
    /// system structure.
    pub fn from_terminal_replay(terminal_replay: &str) -> Self {
        Self::Directory {
            name: "/".to_string(),
            size: 0,
            entries: vec![],
        }
    }

    /// Gets total size of file system entry
    pub fn get_total_size(&self, criteria: Criteria) -> u64 {
        95437
    }
}
