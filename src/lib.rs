//! Fibonacci Iterator Library
//!
//! This library provides a `Fibonacci` struct that implements the `Iterator` trait,
//! allowing users to easily generate Fibonacci sequences with custom starting values.

/// Fibonacci struct that implements the `Iterator` trait.
#[derive(Debug, Clone, Copy)]
pub struct Fibonacci {
    pub a: u64,
    pub b: u64,
}

impl Fibonacci {
    /// Creates a new `Fibonacci` iterator with the given starting values.
    ///
    /// # Examples
    ///
    /// ```
    /// use fibonacci_iterator::Fibonacci;
    ///
    /// let mut fib = Fibonacci::new(0, 1);
    /// assert_eq!(fib.next(), Some(0));
    /// assert_eq!(fib.next(), Some(1));
    /// ```
    pub fn new(a: u64, b: u64) -> Self {
        Fibonacci { a, b }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    /// Generates the next Fibonacci number in the sequence.
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a;
        let new_b = self.a.checked_add(self.b)?;
        self.a = self.b;
        self.b = new_b;
        Some(next)
    }
}
