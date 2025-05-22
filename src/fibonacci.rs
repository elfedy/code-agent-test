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
        let next_val = self.a;
        self.a = self.b;
        self.b = next_val + self.b;
        Some(next_val)
    }
}

fn main() {
    let fibonacci = Fibonacci::new();
    for i in fibonacci.take(10) {
        println!("{}", i);
    }
}
