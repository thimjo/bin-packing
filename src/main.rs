use crate::heuristics::{Solver, FFHeuristic};
use crate::problem::Problem;

mod problem;
mod heuristics;

fn main() {
    let path = "resources/binpack5.txt";

    Problem::parse_problems(path)
        .unwrap()
        .iter()
        .for_each(|problem| {
            let num_bins = <FFHeuristic as Solver>::run(problem);
            println!("Problem {} with known-best {} was solved to {} bins", problem.id(), problem.known_best(), num_bins);
        })
}