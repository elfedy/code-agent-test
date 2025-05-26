//! Fibonacci Iterator

/// Fibonacci iterator.
///
/// # Examples
///
/// ```
/// use fibonacci::Fibonacci;
///
/// let mut fib = Fibonacci::new();
/// assert_eq!(fib.next(), Some(1));
/// assert_eq!(fib.next(), Some(1));
/// assert_eq!(fib.next(), Some(2));
/// assert_eq!(fib.next(), Some(3));
/// assert_eq!(fib.next(), Some(5));
/// ```
pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    /// Creates a new Fibonacci iterator.
    pub fn new() -> Self {
        Fibonacci { a: 1, b: 0 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
    }
}
