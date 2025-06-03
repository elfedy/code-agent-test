fn main() {
    let fibonacci = fibonacci_seq::Fibonacci::new();

    for number in fibonacci.take(10) {
        println!("{}", number);
    }

    println!("\nDemonstrating overflow handling:");
    let fibonacci_overflow = fibonacci_seq::Fibonacci::new();
    for number in fibonacci_overflow.take(100) {
        println!("{}", number);
    }
}
