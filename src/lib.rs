//! Fibonacci module

/// Fibonacci struct that implements the Iterator trait.
#[derive(Debug)]
pub struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    /// Creates a new Fibonacci sequence iterator.
    pub fn new() -> Fibonacci {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    /// This function calculates the next Fibonacci number.
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current.checked_add(self.next)?;
        let new_current = self.next;
        
        self.current = new_current;
        self.next = new_next;

        Some(self.current)
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

    #[test]
    fn test_large_fibonacci() {
        let mut fib = Fibonacci::new();
        // Skip the first few numbers
        for _ in 0..10 {
            fib.next();
        }
        assert_eq!(fib.next(), Some(55));
        assert_eq!(fib.next(), Some(89));
        assert_eq!(fib.next(), Some(144));
    }
}
