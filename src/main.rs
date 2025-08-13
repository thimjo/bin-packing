use std::io::BufRead;
use crate::problem::Problem;

mod problem;
fn main() {
    let path = "resources/binpack5.txt";

    Problem::parse_problems(path);
}