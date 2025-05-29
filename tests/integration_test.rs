use fibonacci_iterator::Fibonacci;

#[test]
fn test_fibonacci_sequence() {
    let mut fib = Fibonacci::new(0, 1);
    assert_eq!(fib.next(), Some(0));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));
    assert_eq!(fib.next(), Some(8));
    assert_eq!(fib.next(), Some(13));
}

#[test]
fn test_fibonacci_sequence_with_custom_start() {
    let mut fib = Fibonacci::new(1, 1);
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));
    assert_eq!(fib.next(), Some(8));
    assert_eq!(fib.next(), Some(13));
    assert_eq!(fib.next(), Some(21));
}

#[test]
fn test_fibonacci_zero_start() {
    let mut fib = Fibonacci::new(0, 0);
    assert_eq!(fib.next(), Some(0));
    assert_eq!(fib.next(), Some(0));
    assert_eq!(fib.next(), Some(0));
    assert_eq!(fib.next(), Some(0));
    assert_eq!(fib.next(), Some(0));
}
