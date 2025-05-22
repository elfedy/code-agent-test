struct Fibonacci {
    a: u32,
    b: u32,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 } 
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next_val = self.a;
        self.a = self.b;
        self.b = next_val + self.b;
        Some(next_val)
    }
}

fn main() {
    let fibonacci_sequence = Fibonacci::new();

    for (i, num) in fibonacci_sequence.enumerate().take(10) {
        println!("Fibonacci number {} is: {}", i + 1, num);
    }
}
