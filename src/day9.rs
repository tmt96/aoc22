use crate::solver::Solver;
use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader},
    ops::Add,
};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Vec2D {
    x: i64,
    y: i64,
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Vec2D {
    fn follow(&self, rhs: &Self) -> Self {
        let (x_diff, y_diff) = (rhs.x - self.x, rhs.y - self.y);

        let (x, y) = if x_diff.abs() > 1 || y_diff.abs() > 1 {
            (x_diff.signum(), y_diff.signum())
        } else {
            (0, 0)
        };

        *self + Self { x, y }
    }
}

type Vec2DIterator = Box<dyn Iterator<Item = Vec2D>>;

fn follow(v: Vec2DIterator) -> Vec2DIterator {
    Box::new(v.scan(Vec2D::default(), |state, el| {
        *state = state.follow(&el);
        Some(*state)
    }))
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec2D>;
    type Output1 = usize;
    type Output2 = usize;

    fn get_day(&self) -> i32 {
        9
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines()
            .flatten()
            .flat_map(|s| {
                let result: Vec<_> = s.split_whitespace().collect();
                let count = result[1].parse::<usize>().unwrap();
                match result[0] {
                    "L" => vec![Vec2D { x: -1, y: 0 }; count],
                    "R" => vec![Vec2D { x: 1, y: 0 }; count],
                    "U" => vec![Vec2D { x: 0, y: 1 }; count],
                    "D" => vec![Vec2D { x: 0, y: -1 }; count],
                    _ => panic!("Unexpected!"),
                }
            })
            .scan(Vec2D::default(), |state, el| {
                *state = *state + el;
                Some(*state)
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let v: Vec2DIterator = Box::new(input.clone().into_iter());
        follow(v).collect::<HashSet<_>>().len()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut v: Vec2DIterator = Box::new(input.clone().into_iter());
        for _ in 0..9 {
            v = follow(v);
        }

        v.collect::<HashSet<_>>().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 13);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), 36);
    }
}
