use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub enum Op {
    Noop,
    Add(i64),
}

impl Op {
    fn from_str(s: &str) -> Self {
        let v: Vec<_> = s.split_whitespace().collect();
        match v[0] {
            "noop" => Op::Noop,
            "addx" => Op::Add(v[1].parse().unwrap()),
            _ => panic!("Unexpected op"),
        }
    }

    fn operate(&self, x: i64) -> i64 {
        match self {
            Op::Noop => x,
            Op::Add(i) => x + i,
        }
    }

    fn cycles(&self) -> i64 {
        match self {
            Op::Noop => 1,
            Op::Add(i) => 2,
        }
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Op>;
    type Output1 = i64;
    type Output2 = String;

    fn get_day(&self) -> i32 {
        10
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines().flatten().map(|s| Op::from_str(&s)).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let (mut cycles, mut x, mut val, mut milestone) = (1, 1, 0, 20);
        for op in input.iter() {
            if cycles > 220 {
                break;
            }
            cycles += op.cycles();
            if cycles > milestone {
                val += milestone * x;
                milestone += 40;
            }
            x = op.operate(x);
        }

        val
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut res = vec![];

        let (mut cycles, mut x) = (0, 1);
        for op in input.iter() {
            for i in 0..op.cycles() {
                if (x - (cycles + i) % 40).abs() <= 1 {
                    res.push('#')
                } else {
                    res.push('.')
                }
            }
            cycles += op.cycles();
            x = op.operate(x);
        }

        res.chunks(40)
            .take(6)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 13140);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(
            problem.solve_second(&input),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
