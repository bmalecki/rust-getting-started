struct Counter {
    count: u32,
    id: u16,
}

pub trait Draw {
    fn draw(&self);
}

impl Draw for Counter {
    fn draw(&self) {
        println!("counter id: {} has {}", self.id, self.count)
    }
}

impl Counter {
    fn new(id: u16) -> Counter {
        Counter { count: 0, id }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 12 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn print_drawable(drawable: &impl Draw) {
    println!("----------");
    drawable.draw();
    println!("----------");
}

fn main() {
    let mut example_cnt = Counter::new(1);

    print_drawable(&example_cnt);
    example_cnt.next();
    print_drawable(&example_cnt);

}
