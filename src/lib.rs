mod fibonacci;

#[test]
fn test_fibonacci_exists() {
    let mut fib = fibonacci::Fibonacci::new();
    assert_eq!(fib.next(), Some(0));
}
