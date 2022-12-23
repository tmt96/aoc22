use crate::solver::{read_to_vec, Solver};
use std::{
    collections::HashSet,
    io::{self},
};

pub struct Problem;

fn char_to_priority(ch: char) -> i64 {
    if ch.is_lowercase() {
        ch as i64 - 'a' as i64 + 1
    } else {
        ch as i64 - 'A' as i64 + 27
    }
}

fn get_common_item_priorities(s: &str) -> i64 {
    let n = s.len();
    let first_half: HashSet<_> = s[..n / 2].chars().collect();
    let second_half: HashSet<_> = s[n / 2..].chars().collect();
    first_half
        .intersection(&second_half)
        .map(|&ch| char_to_priority(ch))
        .sum()
}

impl Solver for Problem {
    type Input = Vec<String>;
    type Output1 = i64;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        3
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        read_to_vec(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().map(|s| get_common_item_priorities(s)).sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .chunks(3)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|s| s.chars().collect::<HashSet<_>>())
                    .reduce(|fst, snd| fst.intersection(&snd).copied().collect())
                    .unwrap()
                    .iter()
                    .map(|&ch| char_to_priority(ch))
                    .sum::<i64>()
            })
            .sum()
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
