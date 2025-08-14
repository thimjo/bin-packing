use crate::heuristics::solve;
use crate::problem::Problem;

mod problem;
mod heuristics;

fn main() {
    let path = "resources/binpack5.txt";

    let problem = Problem::parse_problems(path)
        .unwrap()
        .into_iter()
        .next()
        .unwrap();

    let num_bins = solve(&problem);
    println!("{} bins used", num_bins);
}