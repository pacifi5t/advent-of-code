use std::collections::LinkedList;

pub struct Monkey {
    id: usize,
    inspections: usize,
    items: LinkedList<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> usize>,
}

impl Monkey {
    pub fn new(
        id: usize,
        items: LinkedList<u64>,
        op: Box<dyn Fn(u64) -> u64>,
        test: Box<dyn Fn(u64) -> usize>,
    ) -> Self {
        Self {
            id,
            inspections: 0,
            items,
            op,
            test,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn inspections(&self) -> usize {
        self.inspections
    }

    pub fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    pub fn inspect_next_item(&mut self, gcd: Option<u64>) -> Option<usize> {
        let worry_lvl = self.items.front().map(|&item| (self.op)(item));
        worry_lvl.map(|lvl| {
            let lvl = match gcd {
                None => lvl / 3,
                Some(gcd) => lvl % gcd,
            };
            *self.items.front_mut().unwrap() = lvl;
            self.inspections += 1;
            (self.test)(lvl)
        })
    }

    pub fn throw_item(&mut self) -> Option<u64> {
        self.items.pop_front()
    }

    pub fn catch_item(&mut self, item: u64) {
        self.items.push_back(item)
    }
}
