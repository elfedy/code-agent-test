//! Fibonacci iterator.

/// Fibonacci struct.
#[derive(Debug, Clone, Copy)]
pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    /// Creates a new Fibonacci sequence starting from 0 and 1.
    pub fn new() -> Self {
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
        self.b = next_val.checked_add(self.b)?;
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

    #[test]
    fn test_overflow() {
        let mut fib = Fibonacci { a: 1836311903, b: 2971215073 };
        assert_eq!(fib.next(), Some(1836311903));
        assert_eq!(fib.next(), Some(2971215073));
        assert_eq!(fib.next(), None);
    }
}
