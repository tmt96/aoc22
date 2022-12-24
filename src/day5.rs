use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

pub struct Input {
    stacks: Vec<Vec<char>>,
    insts: Vec<Instruction>,
}

pub struct Problem;

impl Solver for Problem {
    type Input = Input;
    type Output1 = String;
    type Output2 = String;

    fn get_day(&self) -> i32 {
        5
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        // We modified the input from the original format to make it easier to parse.
        // Will comeback and parse the original format programmatically if I feel like it

        let r = BufReader::new(r);
        let mut stacks = vec![];
        let mut insts = vec![];
        let mut finished_stack = false;

        for line in r.lines().flatten() {
            if line.is_empty() {
                finished_stack = true;
            } else if !finished_stack {
                stacks.push(line.chars().collect());
            } else {
                let slice: Vec<_> = line.split_ascii_whitespace().collect();
                let count = slice[1].parse().unwrap();
                let from = slice[3].parse::<usize>().unwrap() - 1;
                let to = slice[5].parse::<usize>().unwrap() - 1;

                let inst = Instruction { count, from, to };
                insts.push(inst);
            }
        }

        Input { stacks, insts }
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut stacks = input.stacks.clone();
        for inst in input.insts.iter() {
            let Instruction { count, from, to } = inst;
            for _ in 0..*count {
                let ch = stacks[*from].pop().unwrap();
                stacks[*to].push(ch);
            }
        }

        stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut stacks = input.stacks.clone();
        for inst in input.insts.iter() {
            let Instruction { count, from, to } = inst;
            let from_len = stacks[*from].len();
            let mut moved = stacks[*from].split_off(from_len - count);
            stacks[*to].append(&mut moved);
        }

        stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "ZN
MCD
P

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), "CMZ");
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), "MCD");
    }
}
