struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub trait IteratorGen<T> {
    fn next(&mut self) -> Option<T>;
}

impl IteratorGen<String> for Counter {
    fn next(&mut self) -> Option<String> {
        todo!()
    }
}
