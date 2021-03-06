use std::io;
use std::time::Duration;
use std::time::Instant;

fn format_time(duration: &Duration) -> (u128, &str) {
    if duration.as_millis() > 0 {
        return (duration.as_millis(), "ms");
    } else {
        return (duration.as_nanos(), "ns");
    };
}

fn fibonacci(n: u128, algo_name: &str, algo: &dyn Fn(u128) -> u128) {
    let now = Instant::now();
    let fib = algo(n);
    let duration = now.elapsed();
    let (elapsed_time, uom) = format_time(&duration);

    println!(
        "{}th Fibonacci number is: {} ({}) - It took {} {}.",
        n, fib, algo_name, elapsed_time, uom
    );
}

fn iter_fibonacci(n: u128) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut fib: u128 = 0;
    for _ in 1..n {
        fib = a + b;
        a = b;
        b = fib;
    }

    return fib;
}

fn recursive_fibonacci(n: u128) -> u128 {
    if n <= 1 {
        return n;
    }

    return recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2);
}

fn main() {
    let mut buffer = String::new();

    println!("Enter a number:");

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    let input: u128 = buffer.trim().parse().expect("Not a number");

    fibonacci(input, "iterative", &iter_fibonacci);
    fibonacci(input, "recusive", &recursive_fibonacci);
}
