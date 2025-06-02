use fibonacci::Fibonacci;

fn main() {
    let mut fib = Fibonacci::new();
    for _ in 0..10 {
        println!("{:?}", fib.next());
    }

    println!("Testing overflow:");
    let mut fib_overflow = Fibonacci { a: u64::MAX - 2, b: 3 };
    println!("{:?}", fib_overflow.next()); // Should print Some(u64::MAX - 2)
    println!("{:?}", fib_overflow.next()); // Should print Some(3)
    println!("{:?}", fib_overflow.next()); // Should print None because (u64::MAX - 2) + 3 overflows
}
