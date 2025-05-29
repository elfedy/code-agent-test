use fibonacci_iterator::Fibonacci;

#[test]
fn test_fibonacci_creation() {
    let fib = Fibonacci::new(0, 1);
    assert_eq!(fib.a, 0);
    assert_eq!(fib.b, 1);
}

#[test]
fn test_fibonacci_creation_with_custom_start() {
    let fib = Fibonacci::new(5, 8);
    assert_eq!(fib.a, 5);
    assert_eq!(fib.b, 8);
}
