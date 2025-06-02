//! Fibonacci sequence iterator.

/// Fibonacci struct that implements the `Iterator` trait.
#[derive(Debug, Clone, Copy)]
pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    /// Creates a new Fibonacci sequence starting from 0 and 1.
    pub fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    /// Calculates the next Fibonacci number.
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a.checked_add(self.b)?;
        self.a = self.b;
        self.b = next;
        Some(self.a)
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
        // The correct next value is 4807526976, but the max value of a u32 is 4294967295.
        //let mut fib = Fibonacci { a: 4294967290, b: 5 };
        //let mut fib = Fibonacci { a: u64::MAX - 5, b: 10 };
        //let mut fib = Fibonacci {a: 9227468975159916031, b: 570288748885472767 };
        //let mut fib = Fibonacci {a: u64::MAX, b: 1 };
        //assert_eq!(fib.next(), None);
        //println!("{}", u64::MAX);
        //println!("{}", fib.a + fib.b);
        //assert_eq!(fib.next(), Some(1));
        //assert_eq!(fib.next(), Some(1));
        //assert_eq!(fib.next(), Some(2));
    }
}
