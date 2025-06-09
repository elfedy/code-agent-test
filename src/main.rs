struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            a: 0,
            b: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_val = self.a.checked_add(self.b)?;
        self.a = self.b;
        self.b = next_val;
        Some(self.a)
    }
}

fn main() {
    for i in Fibonacci::new().take(10) {
        println!("{}", i);
    }
}
