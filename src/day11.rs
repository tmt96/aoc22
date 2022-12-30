use crate::solver::Solver;
use std::io::{self, BufReader, Read};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

impl Operation {
    fn operate(&self, num: usize) -> usize {
        match self {
            Self::Add(i) => num + i,
            Self::Mul(i) => num * i,
            Self::Square => num * num,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test: usize,
    true_throw: usize,
    false_throw: usize,
    count: usize,
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        op: Operation,
        test: usize,
        true_throw: usize,
        false_throw: usize,
    ) -> Self {
        Self {
            items,
            op,
            test,
            true_throw,
            false_throw,
            count: 0,
        }
    }

    fn from_str(s: &str) -> Self {
        let lines: Vec<_> = s.lines().collect();

        let items: Vec<usize> = lines[1]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|i| i.parse::<usize>().unwrap())
            .collect();

        let op = match lines[2]
            .split_whitespace()
            .rev()
            .take(2)
            .collect::<Vec<_>>()
            .as_slice()
        {
            ["old", "*"] => Operation::Square,
            [i, "*"] => Operation::Mul(i.parse::<usize>().unwrap()),
            [i, "+"] => Operation::Add(i.parse::<usize>().unwrap()),
            _ => panic!("Unexpected operation!"),
        };

        let test = lines[3]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let true_throw = lines[4]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let false_throw = lines[5]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Self::new(items, op, test, true_throw, false_throw)
    }

    fn reset(&mut self) {
        self.count += self.items.len();
        self.items = vec![];
    }

    fn add_items(&mut self, item: usize) {
        self.items.push(item);
    }

    fn inspect<F>(&self, item: usize, operate: &F) -> (usize, usize)
    where
        F: Fn(Operation, usize) -> usize,
    {
        let new_val = operate(self.op, item);
        let destination = if new_val % self.test == 0 {
            self.true_throw
        } else {
            self.false_throw
        };
        (new_val, destination)
    }

    fn inspect_all<F>(&self, operate: &F) -> Vec<(usize, usize)>
    where
        F: Fn(Operation, usize) -> usize,
    {
        self.items
            .iter()
            .map(|&i| self.inspect(i, &operate))
            .collect()
    }
}

pub struct Problem;

impl Problem {
    fn solve_helper<F>(&self, input: &[Monkey], operate: &F, rounds: usize) -> usize
    where
        F: Fn(Operation, usize) -> usize,
    {
        let mut monkeys = input.to_vec();
        let n = monkeys.len();

        for _ in 0..rounds {
            for i in 0..n {
                let result = monkeys[i].inspect_all(operate);
                for (val, destination) in result {
                    monkeys[destination].add_items(val);
                }
                monkeys[i].reset();
            }
        }

        monkeys.sort_by_key(|monkey| monkey.count);
        monkeys[n - 1].count * monkeys[n - 2].count
    }
}

impl Solver for Problem {
    type Input = Vec<Monkey>;
    type Output1 = usize;
    type Output2 = usize;

    fn get_day(&self) -> i32 {
        11
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut r = BufReader::new(r);
        let mut buf = String::new();
        r.read_to_string(&mut buf).unwrap();
        buf.split("\n\n").map(Monkey::from_str).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        const ROUNDS: usize = 20;
        self.solve_helper(input, &|op, item| op.operate(item) / 3, ROUNDS)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        const ROUNDS: usize = 10000;
        let modulo: usize = input.iter().map(|m| m.test).product();
        self.solve_helper(input, &|op, item| op.operate(item) % modulo, ROUNDS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 10605);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), 2713310158);
    }
}
