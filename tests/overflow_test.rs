use fibonacci_seq::Fibonacci;

#[test]
fn test_overflow() {
    let mut fib = Fibonacci::new();
    let mut last = 0;
    for _ in 0..93 {
        last = fib.next().unwrap();
    }
    assert_eq!(last, 12200160415121876738);
    assert_eq!(fib.next(), Some(12200160415121876738));
    assert_eq!(fib.next(), None);
}
