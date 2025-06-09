#[test]
fn test_fibonacci_sequence() {
    use fibonacci::Fibonacci;

    let mut fibonacci = Fibonacci::new();
    assert_eq!(fibonacci.next(), Some(0));
    assert_eq!(fibonacci.next(), Some(1));
    assert_eq!(fibonacci.next(), Some(1));
    assert_eq!(fibonacci.next(), Some(2));
    assert_eq!(fibonacci.next(), Some(3));
    assert_eq!(fibonacci.next(), Some(5));
    assert_eq!(fibonacci.next(), Some(8));
    assert_eq!(fibonacci.next(), Some(13));
}
