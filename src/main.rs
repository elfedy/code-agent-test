use fibonacci::Fibonacci;

fn main() {
    let fib = Fibonacci::new();

    for (i, number) in fib.take(10).enumerate() {
        println!("Fibonacci number {} is: {}", i, number);
    }
}
