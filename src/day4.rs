use crate::solver::Solver;
use std::{
    io::{self, BufRead, BufReader},
    ops::RangeInclusive,
};

fn contains<T>(fst: &RangeInclusive<T>, snd: &RangeInclusive<T>) -> bool
where
    T: Ord,
{
    fst.start() <= snd.start() && fst.end() >= snd.end()
}

fn overlaps<T>(fst: &RangeInclusive<T>, snd: &RangeInclusive<T>) -> bool
where
    T: Ord,
{
    fst.start() <= snd.end() && fst.end() >= snd.start()
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<RangeInclusive<i64>>>;
    type Output1 = usize;
    type Output2 = usize;

    fn get_day(&self) -> i32 {
        4
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines()
            .flatten()
            .map(|s| {
                s.split(',')
                    .map(|range| {
                        let mut split = range.split('-');
                        let start = split.next().unwrap().parse::<i64>().unwrap();
                        let end = split.next().unwrap().parse::<i64>().unwrap();
                        RangeInclusive::new(start, end)
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter(|v| contains(&v[0], &v[1]) || contains(&v[1], &v[0]))
            .count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().filter(|v| overlaps(&v[0], &v[1])).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 2);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), 4);
    }
}
