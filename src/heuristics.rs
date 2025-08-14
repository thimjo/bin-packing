use crate::problem::{Item, Problem};

pub struct Bin {
    items: Vec<Item>
}
impl Bin {
    pub fn new(items: Vec<Item>) -> Bin {
        Bin { items }
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn load(&self) -> f64 {
        self.items().iter().map(|it| it.size()).sum()
    }

    pub fn push(&mut self, item: Item) {
        self.items.push(item);
    }
}

pub fn solve(problem: &Problem) -> usize {
    let mut items = problem.items().clone();

    let bin_capacity = problem.bin_capacity();
    let mut solution_bins: Vec<Bin> = Vec::new();

    while let Some(item) = select_item(&mut items) {
        place_item(item, bin_capacity, &mut solution_bins);
    }

    solution_bins.len()
}

fn select_item(items: &mut Vec<Item>) -> Option<Item> {
    items.pop()
}

fn place_item(item: Item, bin_capacity: f64, bins: &mut Vec<Bin>) {
    match bins.iter_mut()
        .find(|bin| bin.load() + item.size() <= bin_capacity) {
        None => {bins.push(Bin::new(vec![item]));},
        Some(bin) => {bin.push(item);}
    }
}