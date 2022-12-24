use crate::solver::Solver;
use std::{
    collections::HashSet,
    io::{self, BufReader, Read},
};

pub struct Problem;

impl Problem {
    fn solve(&self, input: &str, window_size: usize) -> usize {
        input
            .as_bytes()
            .windows(window_size)
            .take_while(|window| {
                window.iter().copied().collect::<HashSet<u8>>().len() < window_size
            })
            .count()
            + window_size
    }
}

impl Solver for Problem {
    type Input = String;
    type Output1 = usize;
    type Output2 = usize;

    fn get_day(&self) -> i32 {
        6
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut s = String::new();
        let mut r = BufReader::new(r);
        r.read_to_string(&mut s).unwrap();
        s
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        self.solve(input, 4)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        self.solve(input, 14)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 5);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), 23);
    }
}
