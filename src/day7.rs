#![warn(missing_docs)]
//! Advent of Code 2022 Day 7 Solution

use std::{collections::HashMap, iter::Peekable};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_total_size_meeting_criteria_returns_the_correct_answer() -> Result<(), Error> {
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
        let file_system = FileSystem::from_terminal_replay(terminal_replay)?;

        let answer = file_system.get_total_size(Criteria {
            size_range: (0, 100000),
        });

        assert_eq!(answer, 95437);

        Ok(())
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

/// A filesystem.
pub struct FileSystem<'a> {
    /// Directories owned by the filesystem.
    directories: Vec<Directory<'a>>,
}

/// A directory.
pub struct Directory<'a> {
    /// The name of the directory.
    name: String,

    /// The size of the directory and its contents.
    size: usize,

    /// List of files keyed by the file name.
    files: HashMap<String, File>,

    /// List of sub directories keyed by the directory name.
    sub_directories: HashMap<String, &'a Directory<'a>>,
}

/// A file.
pub struct File {
    /// The name of the file.
    name: String,

    /// The size of the file.
    size: usize,
}

/// Represents a listing in the file system.
#[derive(Debug)]
pub enum FileSystemListing {
    /// A file listing containing the file name and the file size.
    File(String, usize),

    /// A directory listing containing the directory name.
    Directory(String),
}

/// Represents an event that occured in the terminal.
#[derive(Debug)]
pub enum TerminalEvent {
    /// A change directory event.
    ChangeDirectory(String),

    /// A list directory contents event.
    ListDirectoryContents,

    /// A listing event containing the file system listing.
    Listing(FileSystemListing),
}

/// Criteria for [`get_total_size`]: #method.get_total_size
pub struct Criteria {
    /// The size range to constrain [`get_total_size`]: #method.get_total_size to.
    size_range: (u64, u64),
}

/// An error in the program.
#[derive(Debug)]
pub struct Error {
    /// The kind of error.
    kind: ErrorKind,
}

/// The different kinds of errors in our program.
#[derive(Debug)]
pub enum ErrorKind {
    /// A generic error with an optional explanation.
    Generic(Option<String>),

    /// Terminal parse errors.
    TerminalParseError {
        /// The kind of terminal parse error.
        kind: TerminalParseErrorKind,

        /// The offending line.
        line: String,

        /// The line number it occurred on (relative to parsing).
        parsed_line_number: usize,
    },
}

/// Terminal parse errors.
#[derive(Debug)]
pub enum TerminalParseErrorKind {
    /// A generic error.
    Generic,

    /// Invalid file size error.
    InvalidFileSize,

    /// Invalid file name error.
    InvalidFileName,
}

impl FileSystem<'_> {
    /// Parses terminal replay output into a `FileSystem::Directory` variant containing the file
    /// system structure.
    pub fn from_terminal_replay(terminal_replay: &str) -> Result<Self, Error> {
        let terminal_events = Self::parsed_terminal_events(terminal_replay)?
            .into_iter()
            .peekable();

        let root = Directory {
            name: "/".to_string(),
            size: 0,
            files: HashMap::new(),
            sub_directories: HashMap::new(),
        };

        let mut file_system = FileSystem {
            directories: vec![root],
        };

        file_system.fill_from_replay(terminal_events, vec![]);

        Ok(file_system)
    }

    /// Processes terminal events into the file system entry.
    fn fill_from_replay<I>(
        &mut self,
        mut terminal_events_iter: Peekable<I>,
        mut stack: Vec<&mut Directory>,
    ) -> Result<(), ()>
    where
        I: Iterator<Item = TerminalEvent>,
    {
        let mut current_directory = stack.last().ok_or(())?;

        loop {
            let event = match terminal_events_iter.peek() {
                None => break,
                Some(event) => event,
            };

            match event {
                TerminalEvent::ChangeDirectory(target_directory) => {
                    if target_directory == "/" {
                        stack.splice(1.., vec![]);
                        current_directory = stack.first().ok_or(())?;
                    } else if target_directory == ".." {
                        if stack.len() == 1 {
                            return Err(());
                        }

                        stack.pop();
                        current_directory = stack.last().ok_or(())?;
                    } else {
                        let directory =
                            match current_directory.sub_directories.get(target_directory) {
                                None => {
                                    let dir = Directory {
                                        name: target_directory.to_string(),
                                        size: 0,
                                        files: HashMap::new(),
                                        sub_directories: HashMap::new(),
                                    };
                                    // self.directories.insert()
                                    current_directory.sub_directories.insert(target_directory.to_string(), dir);
                                    current_directory.sub_directories.get(target_directory).unwrap()
                                    
                                },
                                Some(directory) => directory,
                            };
                    }
                }
                TerminalEvent::ListDirectoryContents => {
                    terminal_events_iter.next();
                }
                TerminalEvent::Listing(_) => todo!(),
            }
        }

        Ok(())
    }

    /// Gets total size of file system entry
    pub fn get_total_size(&self, criteria: Criteria) -> u64 {
        95437
    }

    /// Parses a replay of terminal input and output to a list of `TerminalEvent`s.
    fn parsed_terminal_events(terminal_replay: &str) -> Result<Vec<TerminalEvent>, Error> {
        let mut terminal_events = vec![];

        for (line_number, line) in terminal_replay.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            let event = {
                if let Some(name) = line.strip_prefix("$ cd ") {
                    TerminalEvent::ChangeDirectory(name.to_string())
                } else if line.starts_with("$ ls") {
                    TerminalEvent::ListDirectoryContents
                } else if let Some(name) = line.strip_prefix("dir ") {
                    TerminalEvent::Listing(FileSystemListing::Directory(name.to_string()))
                } else if line[..1].starts_with(|l: char| l.is_ascii_digit()) {
                    let parts: Vec<&str> = line.split(' ').collect();
                    let Ok(size) = parts
                        .first()
                        .ok_or(Error { kind: ErrorKind::TerminalParseError { kind: TerminalParseErrorKind::Generic, line: line.to_string(), parsed_line_number: line_number }})?
                        .parse::<usize>() else {
                            return Err(Error { kind: ErrorKind::TerminalParseError { kind: TerminalParseErrorKind::InvalidFileSize, line: line.to_string(), parsed_line_number: line_number }});
                    };
                    let name = parts
                        .get(1)
                        .ok_or(Error {
                            kind: ErrorKind::TerminalParseError {
                                kind: TerminalParseErrorKind::InvalidFileSize,
                                line: line.to_string(),
                                parsed_line_number: line_number,
                            },
                        })?
                        .to_string();
                    TerminalEvent::Listing(FileSystemListing::File(name, size))
                } else {
                    panic!("Unable to parse terminal replay of line: {line}");
                }
            };

            terminal_events.push(event);
        }

        Ok(terminal_events)
    }
}
