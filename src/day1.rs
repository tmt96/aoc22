use crate::solver::Solver;
use std::collections::BinaryHeap;
use std::io::{self, BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        1
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut r = BufReader::new(r);
        let mut buf = String::new();
        let _ = r.read_to_string(&mut buf);
        buf.split("\n\n")
            .map(|s| s.split('\n').flat_map(|s| s.parse::<i64>()).sum::<i64>())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        *input.iter().max().unwrap()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut heap = input.iter().collect::<BinaryHeap<_>>();
        (0..3).map(|_| heap.pop().unwrap()).sum::<Self::Output2>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let raw_input = "";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_first(&input), 0);
    }

    #[test]
    fn test_second() {
        let raw_input = "";
        let problem = Problem {};
        let input = problem.parse_input(raw_input.as_bytes());
        assert_eq!(problem.solve_second(&input), 0);
    }
}
