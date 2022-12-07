use std::fs;

enum Command<'a> {
    Cd(&'a str),
    Ls(Vec<Node<'a>>),
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = &'static str;
    fn try_from(s: &'a str) -> Result<Command<'a>, Self::Error> {
        let mut lines = s.lines();
        let cmd = lines.next().ok_or("Missing command")?.trim();
        if cmd == "ls" {
            let output = lines.flat_map(Node::try_from).collect::<Vec<Node>>();
            Ok(Self::Ls(output))
        } else {
            let (_, dir) = cmd.split_once(' ').ok_or("Missing argument for cd")?;
            Ok(Self::Cd(dir))
        }
    }
}

struct Node<'a> {
    name: &'a str,
    size: Option<u32>,
    children: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    fn size(&self) -> u32 {
        self.size
            .unwrap_or_else(|| self.children.iter().map(Self::size).sum())
    }
    fn get_child(&mut self, name: &str) -> Option<&mut Self> {
        self.children.iter_mut().find(|c| c.name == name)
    }
    fn directory_sizes(&self) -> Vec<u32> {
        // This is pretty inefficient, but good enough for now
        let self_size: u32 = self.children.iter().map(Node::size).sum();
        let mut dir_sizes = self
            .children
            .iter()
            .filter(|c| !c.children.is_empty())
            .flat_map(Node::directory_sizes)
            .collect::<Vec<u32>>();
        if !self.children.is_empty() {
            dir_sizes.push(self_size);
        }
        dir_sizes
    }
    fn get_node(&mut self, path: &Vec<&str>) -> Option<&mut Self> {
        let mut node: Option<&mut Self> = Some(self);
        for p in path {
            node = node.and_then(|e| e.get_child(p));
        }
        node
    }
    fn add_children(&mut self, children: &mut Vec<Self>) {
        self.children.append(children);
    }
}

impl<'a> TryFrom<&'a str> for Node<'a> {
    type Error = &'static str;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (first, second) = s.split_once(' ').ok_or("Invalid file or dir")?;
        if first == "dir" {
            let dir = Self {
                name: second,
                size: None,
                children: vec![],
            };
            Ok(dir)
        } else {
            let size = first.parse::<u32>().map_err(|_| "Invalid size for file")?;
            let file = Self {
                name: second,
                size: Some(size),
                children: vec![],
            };
            Ok(file)
        }
    }
}

fn build_file_hierarchy(commands: Vec<Command>) -> Node {
    let mut root = Node {
        name: "",
        children: vec![Node {
            name: "/",
            children: vec![],
            size: None,
        }],
        size: None,
    };

    let mut current_path: Vec<&str> = vec![];
    for cmd in commands.into_iter() {
        match cmd {
            Command::Cd(path) => {
                if path == ".." {
                    current_path.pop();
                } else {
                    current_path.push(path);
                }
            }
            Command::Ls(mut contents) => {
                if let Some(current_dir) = root.get_node(&current_path) {
                    current_dir.add_children(&mut contents);
                };
            }
        };
    }
    root
}

fn dir_sizes(input: &str) -> Vec<u32> {
    let cmds = input
        .split('$')
        .flat_map(Command::try_from)
        .collect::<Vec<Command>>();

    let files = build_file_hierarchy(cmds);
    files.directory_sizes()
}

fn part1(input: &str) -> u32 {
    dir_sizes(input).iter().filter(|s| **s < 100000).sum()
}

fn part2(input: &str) -> u32 {
    let sizes = dir_sizes(input);

    let total_used = sizes.iter().last().expect("No directories");
    let disk_space = 70000000;
    let to_free = 30000000 - (disk_space - total_used);
    *sizes
        .iter()
        .find(|s| **s >= to_free)
        .expect("No suitable directory found")
}

fn main() {
    let input = fs::read_to_string("inputs/day07.txt").expect("Could not read input file");
    let part1 = part1(&input);
    let part2 = part2(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
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
7214296 k";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 95437);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 24933642);
    }
}
