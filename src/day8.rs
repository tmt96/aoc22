use crate::solver::Solver;
use std::{
    cmp::min,
    io::{self, BufRead, BufReader},
};

pub struct Visibility {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
}

fn calculate_visibility_position(grid: &Vec<Vec<u32>>, position: (usize, usize)) -> Visibility {
    let (width, height) = (grid.len(), grid[0].len());
    let (x, y) = position;

    let left = (0..x)
        .rev()
        .take_while(|&i| grid[i][y] < grid[x][y])
        .count();
    let right = (x + 1..width)
        .take_while(|&i| grid[i][y] < grid[x][y])
        .count();
    let up = (0..y)
        .rev()
        .take_while(|&i| grid[x][i] < grid[x][y])
        .count();
    let down = (y + 1..height)
        .take_while(|&i| grid[x][i] < grid[x][y])
        .count();

    Visibility {
        left,
        right,
        up,
        down,
    }
}

fn calculate_visibility(grid: &Vec<Vec<u32>>) -> Vec<Vec<Visibility>> {
    let (width, height) = (grid.len(), grid[0].len());
    (0..width)
        .map(|x| {
            (0..height)
                .map(|y| calculate_visibility_position(grid, (x, y)))
                .collect()
        })
        .collect()
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<Visibility>>;
    type Output1 = usize;
    type Output2 = usize;

    fn get_day(&self) -> i32 {
        8
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        let grid: Vec<Vec<u32>> = r
            .lines()
            .flatten()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .collect();

        calculate_visibility(&grid)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let (width, height) = (input.len(), input[0].len());
        (0..width)
            .map(|x| {
                (0..height)
                    .filter(move |y| {
                        let Visibility {
                            left,
                            right,
                            up,
                            down,
                        } = input[x][*y];

                        left == x || right == width - x - 1 || up == *y || down == height - y - 1
                    })
                    // .inspect(|y| println!("{x} {y}"))
                    .count()
            })
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let (width, height) = (input.len(), input[0].len());
        (0..width)
            .map(|x| {
                (0..height)
                    .map(|y| {
                        let Visibility {
                            mut left,
                            mut right,
                            mut up,
                            mut down,
                        } = input[x][y];

                        // Visibility Corrections
                        left = min(left + 1, x);
                        right = min(right + 1, width - x - 1);
                        up = min(up + 1, y);
                        down = min(down + 1, height - y - 1);
                        left * right * up * down
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 21);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), 8);
    }
}
