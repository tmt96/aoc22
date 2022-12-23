use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

#[derive(Clone, Copy)]
pub enum Shape {
    Rock = 1,
    Paper,
    Scissor,
}

impl Shape {
    fn lost_to(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape::Rock,
        }
    }

    fn beat(&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Me {
    X = 1,
    Y,
    Z,
}

#[derive(Clone, Copy)]
pub struct Round {
    opponent: Shape,
    me: Me,
}

impl Round {
    fn from_str(round: &str) -> Self {
        let mut shapes = round.split(' ');
        let opponent = match shapes.next().unwrap() {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissor,
            _ => panic!("Invalid input"),
        };

        let me = match shapes.next().unwrap() {
            "X" => Me::X,
            "Y" => Me::Y,
            "Z" => Me::Z,
            _ => panic!("Invalid input"),
        };

        Self { opponent, me }
    }

    fn score_pt1(&self) -> i64 {
        let round_score = match (self.me as i64 - self.opponent as i64).rem_euclid(3) {
            0 => 3,
            1 => 6,
            2 => 0,
            _ => panic!("Unexpected score!"),
        };

        self.me as i64 + round_score
    }

    fn score_pt2(&self) -> i64 {
        let my_shape_score = match self.me {
            Me::X => self.opponent.beat() as i64,    // to lose
            Me::Y => self.opponent as i64,           // draw, same shape
            Me::Z => self.opponent.lost_to() as i64, // to win
        };

        let my_result_score = match self.me {
            Me::X => 0, // to lose
            Me::Y => 3, // draw
            Me::Z => 6, // to win
        };

        my_shape_score + my_result_score
    }
}

impl Solver for Problem {
    type Input = Vec<Round>;
    type Output1 = i64;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        2
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines().flatten().map(|s| Round::from_str(&s)).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().map(|round| round.score_pt1()).sum::<i64>()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().map(|round| round.score_pt2()).sum::<i64>()
    }

    fn input_file(&self) -> String {
        format!("input/day{:02}", self.get_day())
    }

    fn load_input<P: AsRef<std::path::Path>>(&self, p: P) -> io::Result<Self::Input> {
        let f = std::fs::File::open(p)?;
        Ok(self.parse_input(f))
    }

    fn solve(&self) {
        let input_file = self.input_file();
        let input = self
            .load_input(input_file)
            .expect("unable to open input file");
        let s1 = self.solve_first(&input);
        let s2 = self.solve_second(&input);
        println!("Solution 1: {}", s1);
        println!("Solution 2: {}", s2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 15);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), 12);
    }
}
