use Macros::offload;
offload! {
    pub struct Counter {
        count: u32,
    }

    impl Counter {
        pub fn increment(&mut self) {
            self.count += 1;
        }
    }
}

fn main() {
    let mut counter = Counter { count: 0, offload: false};
    //counter.increment();
    counter._increment();
    println!("{}", counter.count);  // prints: 2
}
