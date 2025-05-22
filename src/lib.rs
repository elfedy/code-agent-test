struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { a: 0, b: 1 } // Initialize with the first two Fibonacci numbers
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next_val = self.a;
        let next_val = self.a;
        self.a = self.b;
        self.b = next_val.wrapping_add(self.b);
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
        assert_eq!(fib.next(), Some(55));
        assert_eq!(fib.next(), Some(89));
    }
}
