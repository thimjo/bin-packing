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
        let bin_maybe = ff_select_bin(&item, bin_capacity, &mut solution_bins);

        match bin_maybe {
            Some(bin) => bin.push(item),
            None => {solution_bins.push(Bin::new(vec![item]))}
        }
    }

    solution_bins.len()
}

fn select_item(items: &mut Vec<Item>) -> Option<Item> {
    items.pop()
}

fn ff_select_bin<'a>(item: &Item, bin_capacity: f64, bins: &'a mut Vec<Bin>) -> Option<&'a mut Bin> {
    bins.iter_mut()
        .find(|bin| bin.load() + item.size() <= bin_capacity)
}