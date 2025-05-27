use fibonacci_iterator::Fibonacci;

fn main() {
    let fib = Fibonacci::new();
    for val in fib.take(10) {
        println!("{}", val);
    }
}
