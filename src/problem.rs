use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem {
    id: String,
    bin_capacity: f64,
    items_size: Vec<f64>,
    known_best: u32,
}

impl Problem {
    pub fn new(id: String, bin_capacity: f64, items_size: Vec<f64>, known_best: u32) -> Problem {

        Problem {
            id, bin_capacity, items_size, known_best
        }
    }

    pub fn parse_problems(path: &str) -> io::Result<Vec<Problem>> {
        let file = File::open(path)?;
        let mut lines = BufReader::new(file)
            .lines()
            .filter_map(Result::ok);

        // let num_projects = lines.remove(0).trim().parse().unwrap();
        // let num_projects_line = lines.remove(0);
        let num_projects: usize = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing problem count"))?
            .trim()
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let problems: Vec<Problem> = (0..num_projects)
            .map(|_| Problem::read_problem(&mut lines))
            .collect::<io::Result<Vec<_>>>()?;

        if problems.len() != num_projects {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!("Expected {num_projects} problems, found {}", problems.len()),
            ));
        }

        Ok(problems)
    }

    fn read_problem<I>(lines: &mut I) -> io::Result<Problem> where I: Iterator<Item = String> {
        // id
        let id = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing problem id"))?
            .trim()
            .to_string();

        // "<bin_capacity> <num_items> <known_best>"
        let parts_line = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing problem header line"))?;
        let mut parts = parts_line.split_whitespace();

        let bin_capacity: f64 = parts
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing bin_capacity"))?
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let num_items: usize = parts
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing num_items"))?
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let known_best: u32 = parts
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing known_best"))?
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // next `num_items` lines → u32s
        let items_size: Vec<f64> = (0..num_items)
            .map(|_| {
                lines.next()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing item size line"))?
                    .trim()
                    .parse::<f64>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
            .collect::<Result<Vec<f64>, io::Error>>()?;

        Ok(Problem::new(id, bin_capacity, items_size, known_best))
    }
}