#[cfg(test)]
mod tests {
    use crate::Fibonacci;

    #[test]
    fn test_fibonacci_sequence() {
        let mut fib = Fibonacci::new();
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
    }

    #[test]
    fn test_fibonacci_overflow() {
        let mut fib = Fibonacci::new();
        // Iterate to a point where overflow occurs (example: after many iterations)
        for _ in 0..100 {
            fib.next();
        }
        // Check if the iterator returns None after overflow
        let mut fib = Fibonacci::new();
        for _ in 0..92{
            fib.next();
        }
        assert_eq!(fib.next(), None);
    }

    #[test]
    fn test_fibonacci_zero_init() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(1));
    }
}
