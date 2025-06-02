use fibonacci::Fibonacci;

#[test]
fn test_integration() {
    let mut fib = Fibonacci::new();
    let first_ten: Vec<u64> = fib.take(10).collect();
    assert_eq!(first_ten, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
}
