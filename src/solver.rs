use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[allow(dead_code)]
pub fn read_to_vec<R: io::Read>(r: R) -> Vec<String> {
    let r = BufReader::new(r);
    r.lines().filter_map(|l| l.ok()).collect()
}

pub trait Solver {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn get_day(&self) -> i32;
    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input;
    fn solve_first(&self, input: &Self::Input) -> Self::Output1;
    fn solve_second(&self, input: &Self::Input) -> Self::Output2;

    fn input_file(&self) -> String {
        format!("input/day{:02}", self.get_day())
    }

    fn load_input<P: AsRef<Path>>(&self, p: P) -> io::Result<Self::Input> {
        let f = File::open(p)?;
        Ok(self.parse_input(f))
    }

    fn solve(&self) {
        let input_file = self.input_file();
        let input = self
            .load_input(input_file)
            .expect("unable to open input file");
        let s1 = self.solve_first(&input);
        let s2 = self.solve_second(&input);
        println!("Solution 1: \n{}", s1);
        println!("Solution 2: \n{}", s2);
    }
}
