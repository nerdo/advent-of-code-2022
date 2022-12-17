#![warn(missing_docs)]
//! Advent of Code 2022 Day 7 Solution

use std::{cell::RefCell, collections::HashMap};

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

    #[test]
    fn get_size_of_smallest_directory_leaving_space_returns_the_correct_answer() {
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

        let file_system = FileSystem::from_terminal_replay(terminal_replay).unwrap();

        let answer = file_system.get_size_of_smallest_directory_leaving_space(70000000, 30000000);

        assert_eq!(answer, Some(24933642));
    }
}

/// Part 1.
pub mod part1 {
    use std::{env::current_dir, fs::read_to_string};

    use super::*;

    /// The solution for Part 1.
    pub fn solution() {
        let filename = current_dir().unwrap().join("src/data/day7.txt");
        let input = read_to_string(filename).unwrap();

        let file_system = FileSystem::from_terminal_replay(&input).unwrap();
        let answer = file_system.get_total_size(Criteria {
            size_range: (0, 100000),
        });

        println!("Part 1 Solution = {}", answer);
    }
}

/// Part 2
pub mod part2 {
    /// The solution for Part 2.
    pub fn solution() {}
}

/// A filesystem.
#[derive(Debug)]
pub struct FileSystem {
    /// Directories owned by the filesystem.
    directory_index: RefCell<HashMap<String, RefCell<Directory>>>,
}

/// A directory.
#[derive(Debug)]
pub struct Directory {
    /// The name of the directory.
    #[allow(dead_code)]
    name: String,

    /// The sum of the size of the files in this Directory.
    local_size: RefCell<u64>,

    /// The sum of the size of the files in this directory and any sub directories.
    size: RefCell<u64>,

    /// List of files keyed by the file name.
    files: RefCell<HashMap<String, RefCell<File>>>,

    /// List of directory_index keys (i.e. keys for FileSystem.directory_index)  
    /// keyed by the directory name relative to this Directory.
    sub_directory_keys: HashMap<String, String>,
}

/// A file.
#[derive(Debug)]
pub struct File {
    /// The name of the file.
    #[allow(dead_code)]
    name: String,

    /// The size of the file.
    #[allow(dead_code)]
    size: u64,
}

/// Represents a listing in the file system.
#[derive(Debug)]
pub enum FileSystemListing {
    /// A file listing containing the file name and the file size.
    File(String, u64),

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
#[derive(Debug)]
pub struct Criteria {
    /// The size range to constrain [`get_total_size`]: #method.get_total_size to.
    size_range: (u64, u64),
}

/// An error in the program.
#[derive(Debug)]
pub struct Error {
    /// The kind of error.
    #[allow(dead_code)]
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

impl FileSystem {
    /// Finds the smallest file that, if deleted, will satisfy the space needed, and returns
    /// its size.
    pub fn get_size_of_smallest_directory_leaving_space(
        &self,
        capacity: u64,
        space_needed: u64,
    ) -> Option<u64> {
        let space_used = *self
            .directory_index
            .borrow()
            .get("/")
            .unwrap()
            .borrow()
            .size
            .borrow();

        let mut sorted_directory_sizes = self
            .directory_index
            .borrow()
            .values()
            .into_iter()
            .map(|rc| *rc.borrow().size.borrow())
            .collect::<Vec<u64>>();
        sorted_directory_sizes.sort();

        for directory_size in sorted_directory_sizes {
            let free_space_if_deleted = capacity - (space_used - directory_size);
            if free_space_if_deleted >= space_needed {
                return Some(directory_size);
            }
        }

        None
    }

    /// Parses terminal replay output into a `FileSystem::Directory` variant containing the file
    /// system structure.
    pub fn from_terminal_replay(terminal_replay: &str) -> Result<Self, Error> {
        let terminal_events = Self::parsed_terminal_events(terminal_replay)?;

        let root = Directory {
            name: "/".to_string(),
            local_size: RefCell::new(0),
            size: RefCell::new(0),
            files: RefCell::new(HashMap::new()),
            sub_directory_keys: HashMap::new(),
        };

        let mut directory_index = HashMap::new();
        directory_index.insert("/".to_string(), RefCell::new(root));
        let mut file_system = FileSystem {
            directory_index: RefCell::new(directory_index),
        };

        file_system.fill_from_replay(&terminal_events).unwrap();

        Ok(file_system)
    }

    fn get_new_dir<'b>(
        &self,
        current_path: &'b str,
        target_directory: &'b str,
    ) -> Option<Directory> {
        let new_path = format!("{}{}", current_path, target_directory);
        match self.directory_index.borrow().get(&new_path) {
            None => Some(Directory {
                name: target_directory.to_string(),
                local_size: RefCell::new(0),
                size: RefCell::new(0),
                files: RefCell::new(HashMap::new()),
                sub_directory_keys: HashMap::new(),
            }),
            _ => None,
        }
    }

    /// Processes terminal events into the file system entry.
    fn fill_from_replay(&mut self, terminal_events: &[TerminalEvent]) -> Result<(), &'static str> {
        let mut stack = vec!["/".to_string()];
        let root_path = "/".to_string();

        for event in terminal_events.iter() {
            match event {
                TerminalEvent::ChangeDirectory(target_directory) => {
                    if target_directory == "/" {
                        stack.splice(1.., vec![]);
                    } else if target_directory == ".." {
                        stack.pop();
                    } else {
                        let current_path = stack.last().unwrap_or(&root_path);
                        let directory_key = format!("{}/{}", current_path, &target_directory);
                        if let Some(new_dir) = self.get_new_dir(current_path, target_directory) {
                            self.directory_index
                                .borrow_mut()
                                .insert(directory_key.to_string(), RefCell::new(new_dir));
                        }
                        stack.push(directory_key.to_string());
                    }
                }
                TerminalEvent::ListDirectoryContents => {
                    continue;
                }
                TerminalEvent::Listing(FileSystemListing::File(name, size)) => {
                    // Adjust the sizes of all the directories leading up to this file
                    let last_index = stack.len() - 1;
                    for (index, current_path) in stack.iter().enumerate() {
                        let di = self.directory_index.borrow();
                        let current_directory = di.get(current_path.as_str()).unwrap().borrow();
                        if index == last_index {
                            *current_directory.local_size.borrow_mut() += size;
                        }
                        *current_directory.size.borrow_mut() += size;
                        current_directory.files.borrow_mut().insert(
                            name.clone(),
                            RefCell::new(File {
                                name: name.clone(),
                                size: *size,
                            }),
                        );
                    }
                }
                TerminalEvent::Listing(FileSystemListing::Directory(name)) => {
                    let current_path = stack.last().unwrap_or(&root_path);

                    let new_dir = Directory {
                        name: name.clone(),
                        local_size: RefCell::new(0),
                        size: RefCell::new(0),
                        files: RefCell::new(HashMap::new()),
                        sub_directory_keys: HashMap::new(),
                    };

                    let directory_key = format!("{}/{}", current_path, &name);
                    self.directory_index
                        .borrow_mut()
                        .insert(directory_key.clone(), RefCell::new(new_dir));

                    let di = self.directory_index.borrow();
                    let current_directory = di.get(current_path.as_str()).unwrap();
                    current_directory
                        .borrow_mut()
                        .sub_directory_keys
                        .insert(name.clone(), directory_key.clone());
                }
            }
        }

        Ok(())
    }

    /// Gets total size of file system entry
    pub fn get_total_size(&self, criteria: Criteria) -> u64 {
        let mut matching_directory_sizes = vec![];

        for (_key, directory) in self.directory_index.borrow().iter() {
            let directory_size = *directory.borrow().size.borrow();
            if directory_size >= criteria.size_range.0 && directory_size <= criteria.size_range.1 {
                matching_directory_sizes.push(directory_size);
            }
        }

        matching_directory_sizes.iter().sum()
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
                        .parse::<u64>() else {
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
