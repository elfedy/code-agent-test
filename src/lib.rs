struct Fibonacci {
    a: u64, // Changed to u64 to handle larger numbers
    b: u64, // Changed to u64 to handle larger numbers
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
    type Item = u64; // Changed to u64 to handle larger numbers

    fn next(&mut self) -> Option<Self::Item> {
        let next_val = self.a;
        self.a = self.b;
        self.b = next_val.checked_add(self.b).unwrap_or(0); // Use checked_add to prevent overflow
        Some(next_val)
    }
}


#[cfg(test)]
mod tests {
    use super::Fibonacci;

    #[test]
    fn test_fibonacci() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(0));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
        assert_eq!(fib.next(), Some(8));
        assert_eq!(fib.next(), Some(13));
        assert_eq!(fib.next(), Some(21));
        assert_eq!(fib.next(), Some(34));
    }
}

fn main() {
    let fib = Fibonacci::new();
    for (i, number) in fib.enumerate().take(10) {
        println!("Fibonacci number {} is: {}", i, number);
    }
}
