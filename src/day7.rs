use crate::solver::Solver;
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Default)]
pub struct File {
    size: usize,
}

#[derive(Debug, Default)]
pub struct Dir {
    parent: usize,
    files: HashMap<String, File>,
    dirs: HashMap<String, usize>,
}

impl Dir {
    fn new(parent: usize) -> Self {
        Self {
            parent,
            files: HashMap::new(),
            dirs: HashMap::new(),
        }
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.files.insert(name.to_owned(), File { size });
    }

    fn add_dir(&mut self, name: &str, index: usize) {
        self.dirs.insert(name.to_owned(), index);
    }

    fn get_dir(&self, name: &str) -> Option<&usize> {
        self.dirs.get(name)
    }
}

pub struct Arena {
    cur_node: usize,
    dirs: Vec<Dir>,
}

impl Arena {
    fn new() -> Self {
        let root = Dir::new(0);
        Self {
            cur_node: 0,
            dirs: vec![root],
        }
    }

    fn move_to_root(&mut self) {
        self.cur_node = 0;
    }

    fn move_to_parent(&mut self) {
        let parent = self.dirs[self.cur_node].parent;
        self.cur_node = parent;
    }

    fn move_to_child(&mut self, name: &str) {
        let child = self.dirs[self.cur_node].get_dir(name).unwrap();
        self.cur_node = *child;
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.dirs[self.cur_node].add_file(name, size);
    }

    fn add_dir(&mut self, name: &str) {
        let n = self.dirs.len();
        self.dirs.push(Dir::new(self.cur_node));
        self.dirs[self.cur_node].add_dir(name, n);
    }

    fn dir_size(&self, index: usize) -> usize {
        let dir = &self.dirs[index];
        dir.files.values().map(|f| f.size).sum::<usize>()
            + dir.dirs.values().map(|&i| self.dir_size(i)).sum::<usize>()
    }

    fn size(&self) -> usize {
        self.dir_size(0)
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Arena;
    type Output1 = usize;
    type Output2 = usize;

    fn get_day(&self) -> i32 {
        7
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        let mut dir_tree = Arena::new();

        for line in r.lines().flatten() {
            if line.is_empty() {
                // pass
            }
            let line: Vec<_> = line.split_whitespace().collect();
            if line[0] == "$" {
                // parse command
                if line[1] == "cd" {
                    match line[2] {
                        "/" => dir_tree.move_to_root(),
                        ".." => dir_tree.move_to_parent(),
                        name => dir_tree.move_to_child(name),
                    }
                }
                // nothing to do yet for ls
            } else if line[0] == "dir" {
                dir_tree.add_dir(line[1]);
            } else {
                let size = line[0].parse::<usize>().unwrap();
                dir_tree.add_file(line[1], size);
            }
        }

        dir_tree
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let n = input.dirs.len();
        (0..n)
            .map(|i| input.dir_size(i))
            .filter(|&size| size <= 100000)
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        const LIMIT: usize = 70000000 - 30000000;
        let total_size = input.size();
        let to_be_recovered = total_size - LIMIT;

        let n = input.dirs.len();
        (0..n)
            .map(|i| input.dir_size(i))
            .filter(|&size| size >= to_be_recovered)
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
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

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 95437);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), 24933642);
    }
}
