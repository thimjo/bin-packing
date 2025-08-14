use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
pub struct Item {
    id: usize,
    size: f64,
}

impl Item {
    pub fn new(id: usize, size: f64) -> Item {
        Item {id, size}
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn size(&self) -> f64 {
        self.size
    }
}


pub struct Problem {
    id: String,
    bin_capacity: f64,
    items: Vec<Item>,
    known_best: u32,
}

impl Problem {
    pub fn new(id: String, bin_capacity: f64, items: Vec<Item>, known_best: u32) -> Problem {

        Problem {
            id, bin_capacity, items, known_best
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

    fn read_problem(lines: &mut impl Iterator<Item = String>) -> io::Result<Problem> {
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
        let items: Vec<Item> = (0..num_items)
            .map(|id| {
                let size = lines.next()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Missing item size line"))?
                    .trim()
                    .parse::<f64>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

                Ok(Item::new(id, size))
            })
            .collect::<Result<Vec<Item>, io::Error>>()?;

        Ok(Problem::new(id, bin_capacity, items, known_best))
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn bin_capacity(&self) -> f64 {
        self.bin_capacity
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn known_best(&self) -> u32 {
        self.known_best
    }
}