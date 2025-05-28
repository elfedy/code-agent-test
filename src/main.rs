struct Fibonacci {
    a: u64,
    b: u64,
    stack: Vec<u64>,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            a: 0,
            b: 1,
            stack: Vec::new(),
        }
    }

    fn push(&mut self, value: u64) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<u64> {
        self.stack.pop()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
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
    let mut fib = Fibonacci::new();

    // Demonstrate the iterator
    println!("First 10 Fibonacci numbers:");
    for i in 0..10 {
        println!("{}", fib.next().unwrap());
    }

    // Demonstrate push
    fib.push(100);
    fib.push(200);

    // Demonstrate pop
    println!("Popped value: {}", fib.pop().unwrap());
    println!("Popped value: {}", fib.pop().unwrap());

    // Demonstrate is_empty
    println!("Is the stack empty? {}", fib.is_empty());
}
