#[cfg(test)]
mod tests {
    use fibonacci::Fibonacci;

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
        // Iterate a large number of times to potentially cause overflow
        for _ in 0..100 {
            fib.next();
        }
        // The iterator should continue to produce values even after overflow
        assert!(fib.next().is_some());
    }
}
