#[test]
fn test_fibonacci_sequence() {
    let mut fibonacci = fibonacci_seq::Fibonacci::new();
    assert_eq!(fibonacci.next(), Some(0));
    assert_eq!(fibonacci.next(), Some(1));
    assert_eq!(fibonacci.next(), Some(1));
    assert_eq!(fibonacci.next(), Some(2));
    assert_eq!(fibonacci.next(), Some(3));
    assert_eq!(fibonacci.next(), Some(5));
    assert_eq!(fibonacci.next(), Some(8));
    assert_eq!(fibonacci.next(), Some(13));
    assert_eq!(fibonacci.next(), Some(21));
    assert_eq!(fibonacci.next(), Some(34));
}

#[test]
fn test_fibonacci_overflow() {
    let mut fibonacci = fibonacci_seq::Fibonacci::new();
    let mut last = 0;
    for _ in 0..93 {
        last = fibonacci.next().unwrap();
    }
    assert_eq!(last, 12200160415121876738);
    assert_eq!(fibonacci.next(), Some(12200160415121876738));
    assert_eq!(fibonacci.next(), None);
}
