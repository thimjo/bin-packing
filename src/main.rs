fn main() {
    println!("Hello, world!");
}

struct Problem {
    bin_capacity: u32,
    items_size: Vec<u32>,
}

impl Problem {
    fn new(bin_capacity: u32, items_size: Vec<u32>) -> Problem {

        Problem {
            bin_capacity, items_size,
        }
    }

    fn read_from_file(path: &str) -> Problem {
        todo!("Check the standard file-formats and implement the method")
    }
}
