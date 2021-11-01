use std::io;
use std::time::Duration;
use std::time::Instant;

fn format_time(duration: &Duration) -> (u128, String) {
    if duration.as_millis() > 0 {
        return (duration.as_millis(), String::from("ms"));
    } else {
        return (duration.as_nanos(), String::from("ns"));
    };
}

fn fibonacci(n: usize, algo_name: String, algo: &dyn Fn(usize) -> usize) {
    let now = Instant::now();
    let fib = algo(n);
    let (elapsed_time, uom) = format_time(&now.elapsed());

    println!(
        "{}th Fibonacci number is: {} ({}) - It took {} {}.",
        n, fib, algo_name, elapsed_time, uom
    );
}

fn iter_fibonacci(n: usize) -> usize {
    let mut a: usize = 0;
    let mut b: usize = 1;
    let mut fib: usize = 0;
    for _ in 1..n {
        fib = a + b;
        a = b;
        b = fib;
    }

    return fib;
}

fn recursive_fibonacci(n: usize) -> usize {
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

    let input: usize = buffer.trim().parse().expect("Not a number");

    fibonacci(input, String::from("iterative"), &iter_fibonacci);
    fibonacci(input, String::from("recusive"), &recursive_fibonacci);
}
