fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

// 1. Parse the cmds and create a virtual filesystem
// 2. Get a function to compute the size of a directory
// 3. Find all directory with size at most 100000
// 4. Sum them
fn part1(input: &str) -> usize {
    let root = parse_filesystem_from_input(input);
    get_sum_of_directory_with_max_size(&root, 100000)
}

fn get_sum_of_directory_with_max_size(root: &Directory, max_size: usize) -> usize {
    let mut sum = 0;

    sum += root
        .subdirs
        .iter()
        .map(|d| get_sum_of_directory_with_max_size(d, max_size))
        .sum::<usize>();

    let recursize_size = root.recursive_size();
    if recursize_size <= max_size {
        sum += recursize_size;
    }

    sum
}

// 1. Create filesystem
// 2. Get dir that might be deleted
// 3. Order them by size, and take the smallest
fn part2(input: &str) -> usize {
    let root = parse_filesystem_from_input(input);
    let mut directories_to_remove = find_directory_that_might_be_removed(&root, 70000000, 30000000);
    directories_to_remove.sort();
    *directories_to_remove.first().unwrap()
}

#[derive(Debug, PartialEq)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, PartialEq)]
struct Directory {
    name: String,
    subdirs: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {
    // Returns size of all files in this directory
    // And the size of all subdirectories
    fn recursive_size(&self) -> usize {
        self.size()
            + self
                .subdirs
                .iter()
                .map(|d| d.recursive_size())
                .sum::<usize>()
    }

    // Returns size of all file at current directory level
    // (not recursive)
    fn size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum()
    }

    fn get_dir(&self, path: &[String]) -> Option<&Directory> {
        if path.is_empty() {
            return Some(self);
        }
        if let Some(subdir) = self.get_subdir(&path[0]) {
            return subdir.get_dir(&path[1..]);
        }
        None
    }

    fn get_dir_mut(&mut self, path: &[String]) -> Option<&mut Directory> {
        if path.is_empty() {
            return Some(self);
        }
        if let Some(subdir) = self.get_subdir_mut(&path[0]) {
            return subdir.get_dir_mut(&path[1..]);
        }
        None
    }

    fn get_subdir(&self, name: &str) -> Option<&Directory> {
        self.subdirs.iter().find(|d| d.name == name)
    }

    fn get_subdir_mut(&mut self, name: &str) -> Option<&mut Directory> {
        self.subdirs.iter_mut().find(|d| d.name == name)
    }

    fn add_subdir(&mut self, name: &str) {
        if self.subdirs.iter().any(|d| d.name == name) {
            return;
        }
        self.subdirs.push(Directory {
            name: name.to_string(),
            subdirs: Vec::new(),
            files: Vec::new(),
        });
    }

    fn add_file(&mut self, name: &str, size: usize) {
        if self.files.iter().any(|f| f.name == name) {
            return;
        }
        self.files.push(File {
            name: name.to_string(),
            size,
        });
    }
}

fn parse_filesystem_from_input(input: &str) -> Directory {
    #[derive(Debug, PartialEq)]
    enum ParseState {
        WaitingForCmd,
        ParsingLs,
    }

    let mut current_path = Vec::<String>::new();
    let mut root = Directory {
        name: String::from("/"),
        subdirs: Vec::new(),
        files: Vec::new(),
    };
    let mut state = ParseState::WaitingForCmd;

    for line in input.lines() {
        if state == ParseState::ParsingLs {
            if line.starts_with("$") {
                state = ParseState::WaitingForCmd;
            } else if line.starts_with("dir") {
                let name = line[4..].trim();
                let current_dir = root.get_dir_mut(&current_path).unwrap();
                current_dir.add_subdir(name);
            } else {
                let mut tokens = line.split(" ");
                let size = tokens.next().unwrap().parse::<usize>().unwrap();
                let name = tokens.next().unwrap().trim();
                let current_dir = root.get_dir_mut(&current_path).unwrap();
                current_dir.add_file(name, size);
            }
        }

        if state == ParseState::WaitingForCmd {
            if line.starts_with("$ cd ") {
                let path = line[5..].trim();
                if path == "/" {
                    current_path.clear();
                } else if path == ".." {
                    current_path.pop();
                } else {
                    current_path.push(path.to_string());
                }
            } else if line.starts_with("$ ls") {
                state = ParseState::ParsingLs;
            }
        }
    }

    root
}

fn find_directory_that_might_be_removed(
    root: &Directory,
    total_space: usize,
    total_required_space: usize,
) -> Vec<usize> {
    if total_space < total_required_space {
        panic!("Not enough space to store the file, ever forever")
    }

    let used_space = root.recursive_size();
    if used_space > total_space {
        panic!("Not enough space to store the file, right now")
    }
    let unused_space = total_space - used_space;
    let required_space = total_required_space - unused_space;

    get_subdirs_that_have_size_greater_than(root, required_space)
}

fn get_subdirs_that_have_size_greater_than(root: &Directory, size: usize) -> Vec<usize> {
    let mut results = Vec::new();
    if root.recursive_size() > size {
        results.push(root.recursive_size());
    }

    for subdir in &root.subdirs {
        results.extend(get_subdirs_that_have_size_greater_than(subdir, size));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_subdirs_mut() {
        let mut root = Directory {
            name: String::from("/"),
            subdirs: vec![Directory {
                name: String::from("a"),
                subdirs: Vec::new(),
                files: Vec::new(),
            }],
            files: Vec::new(),
        };

        assert_eq!(root.get_subdir_mut("a").unwrap().name, "a");
        assert_eq!(root.get_subdir_mut("b"), None);
    }

    #[test]
    fn test_get_dir_mut() {
        let mut root = Directory {
            name: String::from("/"),
            subdirs: vec![Directory {
                name: String::from("a"),
                subdirs: vec![Directory {
                    name: String::from("b"),
                    subdirs: Vec::new(),
                    files: Vec::new(),
                }],
                files: Vec::new(),
            }],
            files: Vec::new(),
        };

        assert_eq!(
            root.get_dir_mut(&vec![String::from("a")]).unwrap().name,
            "a"
        );
        assert_eq!(root.get_dir_mut(&vec![String::from("b")]), None);
        assert_eq!(
            root.get_dir_mut(&vec![String::from("a"), String::from("b")])
                .unwrap()
                .name,
            "b"
        );
    }

    #[test]
    fn test_get_size() {
        let root = Directory {
            name: String::from("/"),
            subdirs: vec![Directory {
                name: String::from("a"),
                subdirs: vec![Directory {
                    name: String::from("b"),
                    subdirs: Vec::new(),
                    files: vec![File {
                        name: String::from("c"),
                        size: 10,
                    }],
                }],
                files: vec![
                    File {
                        name: String::from("d"),
                        size: 20,
                    },
                    File {
                        name: String::from("e"),
                        size: 30,
                    },
                ],
            }],
            files: Vec::new(),
        };

        assert_eq!(root.size(), 0);
        assert_eq!(root.get_subdir("a").unwrap().size(), 50);

        assert_eq!(root.recursive_size(), 60);
    }

    static _EXAMPLE_CMDS: &str = r#"
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
"#;

    // Create a Directory structure like:
    // - / (dir)
    //   - a (dir)
    //     - e (dir)
    //       - i (file, size=584)
    //     - f (file, size=29116)
    //     - g (file, size=2557)
    //     - h.lst (file, size=62596)
    //   - b.txt (file, size=14848514)
    //   - c.dat (file, size=8504156)
    //   - d (dir)
    //     - j (file, size=4060174)
    //     - d.log (file, size=8033020)
    //     - d.ext (file, size=5626152)
    //     - k (file, size=7214296)
    fn get_example_filesystem() -> Directory {
        Directory {
            name: "/".to_string(),
            subdirs: vec![
                Directory {
                    name: "a".to_string(),
                    subdirs: vec![Directory {
                        name: "e".to_string(),
                        subdirs: Vec::new(),
                        files: vec![File {
                            name: "i".to_string(),
                            size: 584,
                        }],
                    }],
                    files: vec![
                        File {
                            name: "f".to_string(),
                            size: 29116,
                        },
                        File {
                            name: "g".to_string(),
                            size: 2557,
                        },
                        File {
                            name: "h.lst".to_string(),
                            size: 62596,
                        },
                    ],
                },
                Directory {
                    name: "d".to_string(),
                    subdirs: Vec::new(),
                    files: vec![
                        File {
                            name: "j".to_string(),
                            size: 4060174,
                        },
                        File {
                            name: "d.log".to_string(),
                            size: 8033020,
                        },
                        File {
                            name: "d.ext".to_string(),
                            size: 5626152,
                        },
                        File {
                            name: "k".to_string(),
                            size: 7214296,
                        },
                    ],
                },
            ],
            files: vec![
                File {
                    name: "b.txt".to_string(),
                    size: 14848514,
                },
                File {
                    name: "c.dat".to_string(),
                    size: 8504156,
                },
            ],
        }
    }

    #[test]
    fn test_total_size_e() {
        assert_eq!(
            Directory {
                name: "e".to_string(),
                subdirs: Vec::new(),
                files: vec![File {
                    name: "i".to_string(),
                    size: 584,
                }],
            }
            .recursive_size(),
            584
        );
    }

    #[test]
    fn test_get_total_size_a() {
        assert_eq!(
            Directory {
                name: "a".to_string(),
                subdirs: vec![Directory {
                    name: "e".to_string(),
                    subdirs: Vec::new(),
                    files: vec![File {
                        name: "i".to_string(),
                        size: 584,
                    }],
                }],
                files: vec![
                    File {
                        name: "f".to_string(),
                        size: 29116,
                    },
                    File {
                        name: "g".to_string(),
                        size: 2557,
                    },
                    File {
                        name: "h.lst".to_string(),
                        size: 62596,
                    },
                ],
            }
            .recursive_size(),
            94853
        );
    }

    #[test]
    fn test_parse_filesystem_from_input() {
        assert_eq!(
            parse_filesystem_from_input(_EXAMPLE_CMDS),
            get_example_filesystem()
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(_EXAMPLE_CMDS), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(_EXAMPLE_CMDS), 24933642);
    }
}
